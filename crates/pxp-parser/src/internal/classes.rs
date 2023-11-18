use crate::error;
use crate::error::ParseResult;
use crate::internal::attributes;
use crate::internal::constants::classish;
use crate::internal::functions::method;
use crate::internal::functions::Method;
use crate::internal::functions::MethodType;
use crate::internal::identifiers;
use crate::internal::modifiers;
use crate::internal::parameters;
use crate::internal::properties;
use crate::internal::traits;
use crate::internal::utils;
use crate::state::State;
use pxp_ast::classes::AnonymousClassBody;
use pxp_ast::classes::AnonymousClassExpression;
use pxp_ast::classes::AnonymousClassMember;
use pxp_ast::classes::ClassBody;
use pxp_ast::classes::ClassExtends;
use pxp_ast::classes::ClassImplements;
use pxp_ast::classes::ClassMember;
use pxp_ast::classes::ClassStatement;
use pxp_ast::comments::CommentGroup;
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_ast::Expression;
use pxp_ast::StatementKind;
use pxp_ast::{ExpressionKind, NewExpression};
use pxp_span::Span;
use pxp_token::TokenKind;

pub fn parse(state: &mut State) -> ParseResult<StatementKind> {
    let attributes = state.get_attributes();

    let modifiers = modifiers::class_group(modifiers::collect(state)?)?;
    let class = utils::skip(state, TokenKind::Class)?;
    let name = identifiers::type_identifier(state)?;
    let current = state.stream.current();
    let extends = if current.kind == TokenKind::Extends {
        let span = current.span;

        state.stream.next();
        let parent = identifiers::full_type_name(state)?;

        Some(ClassExtends {
            extends: span,
            parent,
        })
    } else {
        None
    };

    let current = state.stream.current();
    let implements = if current.kind == TokenKind::Implements {
        let span = current.span;

        state.stream.next();

        let interfaces =
            utils::at_least_one_comma_separated_no_trailing::<SimpleIdentifier>(state, &|state| {
                identifiers::full_type_name(state)
            })?;

        Some(ClassImplements {
            implements: span,
            interfaces,
        })
    } else {
        None
    };

    let has_abstract = modifiers.has_abstract();
    let body = ClassBody {
        left_brace: utils::skip_left_brace(state)?,
        members: {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace {
                members.push(member(state, has_abstract, &name)?);
            }

            members
        },
        right_brace: utils::skip_right_brace(state)?,
    };

    Ok(StatementKind::Class(ClassStatement {
        class,
        name,
        modifiers,
        extends,
        implements,
        attributes,
        body,
    }))
}

pub fn parse_anonymous(state: &mut State, span: Option<Span>) -> ParseResult<Expression> {
    let new = match span {
        Some(span) => span,
        None => utils::skip(state, TokenKind::New)?,
    };

    let start_span = new;

    attributes::gather_attributes(state)?;

    let attributes = state.get_attributes();

    let class = utils::skip(state, TokenKind::Class)?;
    let class_span = class;

    let arguments = if state.stream.current().kind == TokenKind::LeftParen {
        Some(parameters::argument_list(state)?)
    } else {
        None
    };

    let current = state.stream.current();
    let extends = if current.kind == TokenKind::Extends {
        state.stream.next();

        let extends = current.span;
        let parent = identifiers::full_name(state)?;

        Some(ClassExtends { extends, parent })
    } else {
        None
    };

    let current = state.stream.current();
    let implements = if current.kind == TokenKind::Implements {
        state.stream.next();

        let implements = current.span;
        let interfaces =
            utils::at_least_one_comma_separated_no_trailing::<SimpleIdentifier>(state, &|state| {
                identifiers::full_name(state)
            })?;

        Some(ClassImplements {
            implements,
            interfaces,
        })
    } else {
        None
    };

    let body = AnonymousClassBody {
        left_brace: utils::skip_left_brace(state)?,
        members: {
            let mut members = Vec::new();
            while state.stream.current().kind != TokenKind::RightBrace {
                members.push(anonymous_member(state)?);
            }
            members
        },
        right_brace: utils::skip_right_brace(state)?,
    };

    let end_span = body.right_brace;

    let anonymous_class = Expression::new(
        ExpressionKind::AnonymousClass(AnonymousClassExpression {
            class,
            extends,
            implements,
            attributes,
            body,
        }),
        Span::new(class_span.start, end_span.end),
        CommentGroup::default(),
    );

    Ok(Expression::new(
        ExpressionKind::New(NewExpression {
            target: Box::new(anonymous_class),
            new,
            arguments,
        }),
        Span::new(start_span.start, state.stream.previous().span.end),
        CommentGroup::default(),
    ))
}

fn member(
    state: &mut State,
    has_abstract: bool,
    name: &SimpleIdentifier,
) -> ParseResult<ClassMember> {
    let has_attributes = attributes::gather_attributes(state)?;

    if !has_attributes && state.stream.current().kind == TokenKind::Use {
        return traits::usage(state).map(ClassMember::TraitUsage);
    }

    if state.stream.current().kind == TokenKind::Var {
        return properties::parse_var(state, Some(name)).map(ClassMember::VariableProperty);
    }

    let modifiers = modifiers::collect(state)?;

    if state.stream.current().kind == TokenKind::Const {
        return classish(state, modifiers::constant_group(modifiers)?).map(ClassMember::Constant);
    }

    if state.stream.current().kind == TokenKind::Function {
        let method = method(
            state,
            MethodType::DependingOnModifiers,
            modifiers::method_group(modifiers)?,
            Some(name),
        )?;

        return match method {
            Method::Abstract(method) => {
                if has_abstract {
                    Ok(ClassMember::AbstractMethod(method))
                } else {
                    Err(error::abstract_method_on_a_non_abstract_class(
                        state,
                        name,
                        &method.name,
                        method.modifiers.get_abstract().unwrap().span(),
                        method.semicolon,
                    ))
                }
            }
            Method::Concrete(method) => Ok(ClassMember::ConcreteMethod(method)),
            Method::AbstractConstructor(ctor) => {
                if has_abstract {
                    Ok(ClassMember::AbstractConstructor(ctor))
                } else {
                    Err(error::abstract_method_on_a_non_abstract_class(
                        state,
                        name,
                        &ctor.name,
                        ctor.modifiers.get_abstract().unwrap().span(),
                        ctor.semicolon,
                    ))
                }
            }
            Method::ConcreteConstructor(ctor) => Ok(ClassMember::ConcreteConstructor(ctor)),
        };
    }

    // e.g: public static
    let modifiers = modifiers::property_group(modifiers)?;

    properties::parse(state, Some(name), modifiers).map(ClassMember::Property)
}

fn anonymous_member(state: &mut State) -> ParseResult<AnonymousClassMember> {
    let has_attributes = attributes::gather_attributes(state)?;

    if !has_attributes && state.stream.current().kind == TokenKind::Use {
        return traits::usage(state).map(AnonymousClassMember::TraitUsage);
    }

    if state.stream.current().kind == TokenKind::Var {
        return properties::parse_var(state, None).map(AnonymousClassMember::VariableProperty);
    }

    let modifiers = modifiers::collect(state)?;

    if state.stream.current().kind == TokenKind::Const {
        return classish(state, modifiers::constant_group(modifiers)?)
            .map(AnonymousClassMember::Constant);
    }

    if state.stream.current().kind == TokenKind::Function {
        let method = method(
            state,
            MethodType::Concrete,
            modifiers::method_group(modifiers)?,
            None,
        )?;

        match method {
            Method::Concrete(method) => {
                return Ok(AnonymousClassMember::ConcreteMethod(method));
            }
            Method::ConcreteConstructor(ctor) => {
                return Ok(AnonymousClassMember::ConcreteConstructor(ctor));
            }
            Method::Abstract(_) | Method::AbstractConstructor(_) => unreachable!(),
        }
    }

    // e.g: public static
    let modifiers = modifiers::property_group(modifiers)?;

    properties::parse(state, None, modifiers).map(AnonymousClassMember::Property)
}
