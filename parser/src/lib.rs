pub mod ast;
mod context;
mod error;
mod keyword;

pub use ast::Statement;
pub use error::ParserError;
pub use keyword::Keyword;
/* I don't like it being public, but tests needs it */
pub use context::ParserContext;

// Returns Some(token) only when the next token is of the give variant
macro_rules! next_token {
    ($ctx:ident, $expected:pat) => {{
        let next = $ctx.next();
        if matches!(next, $expected) {
            Ok(next)
        } else {
            println!("Next token backtrack");
            $ctx.backtrack();
            Err(ParserError::UnexpectedToken {
                expected: stringify!($expected)
                    .split("::")
                    .last()
                    .unwrap()
                    .split('(')
                    .next()
                    .unwrap()
                    .trim()
                    .to_string(),
                got: next.variant_name().to_string(),
            })
        }
    }};
}

fn unpack_lexer_litteral(ctx: &mut ParserContext) -> Result<String, ParserError> {
    match next_token!(ctx, lexer::Token::Litteral(..))? {
        lexer::Token::Litteral(literal) => Ok(literal),
        _ => unreachable!(),
    }
}

fn variable_mutability(ctx: &mut ParserContext) -> Result<bool, ParserError> {
    if keyword(ctx, Keyword::Const) {
        Ok(false)
    } else if keyword(ctx, Keyword::Var) {
        Ok(true)
    } else {
        Err(ParserError::UnexpectedToken {
            expected: {
                let mut list = vec!["const", "var"];
                list.extend_from_slice(&ctx.aliases_for(Keyword::Const));
                list.extend_from_slice(&ctx.aliases_for(Keyword::Var));

                format!(
                    "one of {}",
                    list.into_iter()
                        .fold(String::new(), |mut output, s| {
                            use std::fmt::Write as _;
                            let _ = write!(output, "`{s}`,");
                            output
                        })
                        .trim_end_matches(",")
                )
            },
            got: ctx.peek().variant_name().to_string(),
        })
    }
}

// eats any number of token as long as it matches the given on
macro_rules! eat_many {
    ($ctx:ident, $($expected:pat),*) => {{
        let mut count = 0u32;
        loop{
            let next = $ctx.next();

            let mut found = false;
            $(
                if matches!(next, $expected){
                    found = true;
                }
            )*;

            if found{
                count += 1;
                continue
            }

            $ctx.backtrack();
            break

        }
        count
    }};
}

fn keyword(ctx: &mut ParserContext, expected_keyword: Keyword) -> bool {
    let Ok(next_litteral) = unpack_lexer_litteral(ctx) else {
        return false;
    };

    let Some(kw) = ctx.is_kw(&next_litteral) else {
        // Since unpack_litteral advanced
        ctx.backtrack();
        return false;
    };

    if kw != expected_keyword {
        ctx.backtrack();
        return false;
    };

    true
}

fn parse_var_asignment<'a>(
    ctx: &'a mut ParserContext,
) -> Result<ast::AssignmentExpression, ParserError> {
let outer_mutability = variable_mutability(ctx)?;
    next_token!(ctx, lexer::Token::Space)?;
    let inner_mutability = variable_mutability(ctx)?;
    next_token!(ctx, lexer::Token::Space)?;

    // Not required
    let global_mutability = 'global: {
        let Ok(c) = unpack_lexer_litteral(ctx) else {
            break 'global None;
        };

        let Some(kw) = ctx.is_kw(&c) else {
            ctx.backtrack();
            break 'global None;
        };

        if kw == Keyword::Var {
            break 'global Some(true);
        } else if kw == Keyword::Const {
            break 'global Some(false);
        } else {
            ctx.backtrack();
            None
        }
    };

    if global_mutability.is_some() {
        next_token!(ctx, lexer::Token::Space)?;
    }

    let name = unpack_lexer_litteral(ctx)?;
    next_token!(ctx, lexer::Token::Space)?;

    next_token!(ctx, lexer::Token::Equal)?;
    next_token!(ctx, lexer::Token::Space)?;

    let expression = parse_expression(ctx)?;

    let _type = match expression {
        ast::Expression::Litteral(ast::Litteral::Int(..)) => types::DreamberdType::Int,
        ast::Expression::Litteral(ast::Litteral::Float(..)) => types::DreamberdType::Float,
        ast::Expression::Litteral(ast::Litteral::String(..)) => types::DreamberdType::String,
        ast::Expression::Litteral(ast::Litteral::Bool(..)) => types::DreamberdType::Bool,
        ast::Expression::Variable(ast::Variable { _type, .. }) => _type,
        _ => {
            // TODO: It's actually possible to deduce type from theses other variants
            types::DreamberdType::Unknown
        }
    };
    next_token!(ctx, lexer::Token::Bang)?;

    Ok(ast::AssignmentExpression {
        outer_mutability,
        inner_mutability,
        global: global_mutability.is_some(),
        global_mutability: global_mutability.unwrap_or(false),
        name,
        _type,
        value: expression,
    })
}

pub fn parse_expression(ctx: &mut ParserContext) -> Result<ast::Expression, ParserError> {
    // Ok(ast::Expression::Litteral(ast::Litteral::Bool(false)))
    // Ok(ast::Expression::Litteral(parse_litteral_expression(ctx)?))
    Ok(ast::Expression::Litteral(parse_litteral_expression(ctx)?))
}

pub fn parse_variable_expression(ctx: &mut ParserContext) -> Result<ast::Variable, ParserError> {
    Ok(ast::Variable {
        name: String::from("VarName"),
        _type: types::DreamberdType::Unknown,
    })
}

pub fn parse_litteral_expression(ctx: &mut ParserContext) -> Result<ast::Litteral, ParserError> {
    parse_litteral(ctx)
}

pub fn parse_litteral(ctx: &mut ParserContext) -> Result<ast::Litteral, ParserError> {
    parse_string_litteral(ctx)
}

pub fn parse_int_litteral(ctx: &mut ParserContext) -> Result<ast::Litteral, ParserError> {
    Ok(ast::Litteral::Int(0))
}
pub fn parse_float_litteral(ctx: &mut ParserContext) -> Result<ast::Litteral, ParserError> {
    Ok(ast::Litteral::Float(0.0))
}
pub fn parse_string_litteral(ctx: &mut ParserContext) -> Result<ast::Litteral, ParserError> {
    eat_many!(ctx, lexer::Token::SingleQuote, lexer::Token::DoubleQuote);

    let mut text = String::new();

    loop {
        let next = ctx.next();

        // This adds characters to the string
        match next {
            lexer::Token::Dot => {
                text.push('.');
                continue;
            }
            lexer::Token::Equal => {
                text.push('=');
                continue;
            }
            lexer::Token::Comma => {
                text.push(',');
                continue;
            }
            lexer::Token::Space => {
                text.push(' ');
                continue;
            }
            lexer::Token::OpenBracket => {
                text.push('[');
                continue;
            }
            lexer::Token::OpenParenthesis => {
                text.push('(');
                continue;
            }
            lexer::Token::LessThan => {
                text.push('<');
                continue;
            }
            lexer::Token::GreaterThan => {
                text.push('>');
                continue;
            }
            lexer::Token::Plus => {
                text.push('+');
                continue;
            }
            lexer::Token::Minus => {
                text.push('-');
                continue;
            }
            lexer::Token::Divide => {
                text.push('/');
                continue;
            }
            lexer::Token::Multiply => {
                text.push('*');
                continue;
            }
            lexer::Token::Numeric(vec) => {
                text.push_str(&vec.iter().fold(String::new(), |mut output, n| {
                    use std::fmt::Write as _;
                    let _ = write!(output, "{n}");
                    output
                }));
                continue;
            }
            lexer::Token::Litteral(s) => text.push_str(&s),

            // Can cause issues
            lexer::Token::CloseBracket => {
                /*
                Issue:

                We could check what's after, but a string could be "aaa)", "aaa)," or even "aaaa)!" without everything breaking
                */
                text.push(']');
                continue;
            }
            lexer::Token::CloseParenthesis => {
                /*
                Issue:
                    Litteral string in a function call
                    is it a simple ) in the string or the end of the function parameters list

                We could check what's after, but a string could be "aaa)", "aaa)," or even "aaaa)!" without everything breaking
                */
                text.push(')');
                continue;
            }

            // Double meaning
            lexer::Token::Bang if ctx.peek() != lexer::Token::NewLine => {
                text.push('!');
                continue;
            }
            lexer::Token::QuestionMark if ctx.peek() != lexer::Token::NewLine => {
                text.push('?');
                continue;
            }

            // Unwanted
            lexer::Token::Bang | lexer::Token::QuestionMark => {
                // This was an unclosed string, and we reached the end of the line
                ctx.backtrack();
                break;
            }

            // Should not be in a string
            lexer::Token::NewLine // Quick note on the newline, it's an actual new line,
                                  // \n are still allowed in a string
            | lexer::Token::EOF
            | lexer::Token::SingleQuote
            | lexer::Token::DoubleQuote => {
                ctx.backtrack();
                break;
            }
        }
    }

    eat_many!(ctx, lexer::Token::SingleQuote, lexer::Token::DoubleQuote);

    println!("Got text: {text}");

    Ok(ast::Litteral::String(text))
}
pub fn parse_bool_litteral(ctx: &mut ParserContext) -> Result<ast::Litteral, ParserError> {
    Ok(ast::Litteral::Bool(false))
}

pub fn parse(tokens: &[lexer::Token]) -> Result<Statement, ParserError> {
    let mut context = ParserContext::new(tokens);

    Ok(Statement::Empty)
}
