use crate::expressions;
use crate::internal::blocks;
use crate::internal::utils;
use crate::state::State;
use crate::statement;
use pxp_ast::literals::Literal;
use pxp_ast::literals::LiteralKind;
use pxp_ast::loops::BreakStatement;
use pxp_ast::loops::ContinueStatement;
use pxp_ast::loops::DoWhileStatement;
use pxp_ast::loops::ForStatement;
use pxp_ast::loops::ForStatementBody;
use pxp_ast::loops::ForStatementIterator;
use pxp_ast::loops::ForeachStatement;
use pxp_ast::loops::ForeachStatementBody;
use pxp_ast::loops::ForeachStatementIterator;
use pxp_ast::loops::Level;
use pxp_ast::loops::WhileStatement;
use pxp_ast::loops::WhileStatementBody;
use pxp_ast::StatementKind;
use pxp_token::Token;
use pxp_token::TokenKind;

pub fn foreach_statement(state: &mut State) -> StatementKind {
    let foreach = utils::skip(state, TokenKind::Foreach);

    let (left_parenthesis, iterator, right_parenthesis) =
        utils::parenthesized(state, &|state: &mut State| {
            let expression = expressions::create(state);

            let r#as = utils::skip(state, TokenKind::As);

            let current = state.stream.current();
            let ampersand = if current.kind == TokenKind::Ampersand {
                state.stream.next();
                Some(current.span)
            } else {
                None
            };

            let mut value = expressions::create(state);

            let current = state.stream.current();
            if current.kind == TokenKind::DoubleArrow {
                state.stream.next();
                let arrow = current.span;

                let current = state.stream.current();
                let ampersand = if current.kind == TokenKind::Ampersand {
                    state.stream.next();
                    Some(current.span)
                } else {
                    None
                };

                let mut key = expressions::create(state);

                std::mem::swap(&mut value, &mut key);

                ForeachStatementIterator::KeyAndValue {
                    expression,
                    r#as,
                    key,
                    double_arrow: arrow,
                    ampersand,
                    value,
                }
            } else {
                ForeachStatementIterator::Value {
                    expression,
                    r#as,
                    ampersand,
                    value,
                }
            }
        });

    let body = if state.stream.current().kind == TokenKind::Colon {
        ForeachStatementBody::Block {
            colon: utils::skip_colon(state),
            statements: blocks::multiple_statements_until(state, &TokenKind::EndForeach),
            endforeach: utils::skip(state, TokenKind::EndForeach),
            ending: utils::skip_ending(state),
        }
    } else {
        ForeachStatementBody::Statement {
            statement: Box::new(statement(state)),
        }
    };

    StatementKind::Foreach(ForeachStatement {
        foreach,
        left_parenthesis,
        iterator,
        right_parenthesis,
        body,
    })
}

pub fn for_statement(state: &mut State) -> StatementKind {
    let r#for = utils::skip(state, TokenKind::For);

    let (left_parenthesis, iterator, right_parenthesis) = utils::parenthesized(state, &|state| {
        let (initializations_semicolon, initializations) =
            utils::semicolon_terminated(state, &|state| {
                utils::comma_separated_no_trailing(
                    state,
                    &expressions::create,
                    TokenKind::SemiColon,
                )
            });

        let (conditions_semicolon, conditions) = utils::semicolon_terminated(state, &|state| {
            utils::comma_separated_no_trailing(state, &expressions::create, TokenKind::SemiColon)
        });

        ForStatementIterator {
            initializations,
            initializations_semicolon,
            conditions,
            conditions_semicolon,
            r#loop: utils::comma_separated_no_trailing(
                state,
                &expressions::create,
                TokenKind::RightParen,
            ),
        }
    });

    let body = if state.stream.current().kind == TokenKind::Colon {
        ForStatementBody::Block {
            colon: utils::skip_colon(state),
            statements: blocks::multiple_statements_until(state, &TokenKind::EndFor),
            endfor: utils::skip(state, TokenKind::EndFor),
            ending: utils::skip_ending(state),
        }
    } else {
        ForStatementBody::Statement {
            statement: Box::new(statement(state)),
        }
    };

    StatementKind::For(ForStatement {
        r#for,
        left_parenthesis,
        iterator,
        right_parenthesis,
        body,
    })
}

pub fn do_while_statement(state: &mut State) -> StatementKind {
    let r#do = utils::skip(state, TokenKind::Do);

    let body = Box::new(statement(state));

    let r#while = utils::skip(state, TokenKind::While);

    let (semicolon, (left_parenthesis, condition, right_parenthesis)) =
        utils::semicolon_terminated(state, &|state| {
            utils::parenthesized(state, &expressions::create)
        });

    StatementKind::DoWhile(DoWhileStatement {
        r#do,
        body,
        r#while,
        left_parenthesis,
        condition,
        right_parenthesis,
        semicolon,
    })
}

pub fn while_statement(state: &mut State) -> StatementKind {
    let r#while = utils::skip(state, TokenKind::While);

    let (left_parenthesis, condition, right_parenthesis) =
        utils::parenthesized(state, &expressions::create);

    let body = if state.stream.current().kind == TokenKind::Colon {
        WhileStatementBody::Block {
            colon: utils::skip_colon(state),
            statements: blocks::multiple_statements_until(state, &TokenKind::EndWhile),
            endwhile: utils::skip(state, TokenKind::EndWhile),
            ending: utils::skip_ending(state),
        }
    } else {
        WhileStatementBody::Statement {
            statement: Box::new(statement(state)),
        }
    };

    StatementKind::While(WhileStatement {
        r#while,
        left_parenthesis,
        condition,
        right_parenthesis,
        body,
    })
}

pub fn continue_statement(state: &mut State) -> StatementKind {
    StatementKind::Continue(ContinueStatement {
        r#continue: utils::skip(state, TokenKind::Continue),
        level: maybe_loop_level(state),
        ending: utils::skip_ending(state),
    })
}

pub fn break_statement(state: &mut State) -> StatementKind {
    StatementKind::Break(BreakStatement {
        r#break: utils::skip(state, TokenKind::Break),
        level: maybe_loop_level(state),
        ending: utils::skip_ending(state),
    })
}

fn maybe_loop_level(state: &mut State) -> Option<Level> {
    let current = &state.stream.current().kind;

    if current == &TokenKind::SemiColon || current == &TokenKind::CloseTag {
        None
    } else {
        Some(loop_level(state))
    }
}

fn loop_level(state: &mut State) -> Level {
    let current = state.stream.current();

    if let Token {
        kind: TokenKind::LiteralInteger,
        ..
    } = current
    {
        state.stream.next();

        return Level::Literal(Literal::new(LiteralKind::Integer, *current));
    }

    let (left_parenthesis, level, right_parenthesis) =
        utils::parenthesized(state, &|state| Box::new(loop_level(state)));

    Level::Parenthesized {
        left_parenthesis,
        level,
        right_parenthesis,
    }
}
