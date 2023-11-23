use crate::internal::identifiers;
use crate::internal::utils;
use crate::scoped;
use crate::state::NamespaceType;
use crate::state::Scope;
use crate::state::State;
use pxp_ast::identifiers::SimpleIdentifier;
use pxp_ast::namespaces::BracedNamespace;
use pxp_ast::namespaces::BracedNamespaceBody;
use pxp_ast::namespaces::NamespaceStatement;
use pxp_ast::namespaces::UnbracedNamespace;
use pxp_ast::Block;
use pxp_ast::StatementKind;
use pxp_span::Span;
use pxp_token::TokenKind;

pub fn namespace(state: &mut State) -> StatementKind {
    let start = utils::skip(state, TokenKind::Namespace);
    let name = identifiers::optional_name(state);

    let current = state.stream.current();

    if let Some(name) = &name {
        if current.kind != TokenKind::LeftBrace {
            if let Some(NamespaceType::Braced) = state.namespace_type() {
                todo!("tolerant mode")
                // return Err(error::unbraced_namespace_declarations_in_braced_context(
                //     current.span,
                // ));
            }

            return unbraced_namespace(state, start, name.clone());
        }
    }

    match state.namespace_type() {
        Some(NamespaceType::Unbraced) => todo!("tolerant mode") /*Err(
            error::braced_namespace_declarations_in_unbraced_context(current.span),
        )*/,
        Some(NamespaceType::Braced) if state.namespace().is_some() => {
            todo!("tolerant mode")
            // Err(error::nested_namespace_declarations(start))
        }
        _ => braced_namespace(state, start, name),
    }
}

fn unbraced_namespace(state: &mut State, start: Span, name: SimpleIdentifier) -> StatementKind {
    let end = utils::skip_semicolon(state);

    let statements = scoped!(state, Scope::Namespace(name.clone()), {
        let mut statements = Block::new();
        // since this is an unbraced namespace, as soon as we encouter another
        // `namespace` token as a top level statement, this namespace scope ends.
        // otherwise we will end up with nested namespace statements.
        while state.stream.current().kind != TokenKind::Namespace && !state.stream.is_eof() {
            statements.push(crate::top_level_statement(state));
        }

        statements
    });

    StatementKind::Namespace(NamespaceStatement::Unbraced(UnbracedNamespace {
        start,
        end,
        name,
        statements,
    }))
}

fn braced_namespace(
    state: &mut State,
    span: Span,
    name: Option<SimpleIdentifier>,
) -> StatementKind {
    let body = scoped!(state, Scope::BracedNamespace(name.clone()), {
        let start = utils::skip_left_brace(state);

        let mut statements = Block::new();
        while state.stream.current().kind != TokenKind::RightBrace && !state.stream.is_eof() {
            statements.push(crate::top_level_statement(state));
        }

        let end = utils::skip_right_brace(state);

        BracedNamespaceBody {
            start,
            end,
            statements,
        }
    });

    StatementKind::Namespace(NamespaceStatement::Braced(BracedNamespace {
        namespace: span,
        name,
        body,
    }))
}
