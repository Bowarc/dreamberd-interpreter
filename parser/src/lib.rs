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

// eats any number of token as long as it matches the given ones
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

pub fn parse_var_assignment(
    ctx: &mut ParserContext,
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

    let mut name = unpack_lexer_litteral(ctx)?;
    loop {
        match ctx.next() {
            lexer::Token::Underscore => name.push('_'),
            lexer::Token::Litteral(s) => name.push_str(&s),
            _ => {
                ctx.backtrack();
                break;
            }
        }
    }


    // Optional type annotation
    'colon: {
        // optional
        let optionnal_space1 = next_token!(ctx, lexer::Token::Space).is_ok();

        if ctx.peek() != lexer::Token::Colon{
            if optionnal_space1{
                ctx.backtrack();
            }

            break 'colon;
        }
        ctx.advance();

        let _ = next_token!(ctx, lexer::Token::Space);

        // Hard error because we're too far into the branch to actually backtrack,
        // if there is a colon after the variable name, there is no path other than this one
        unpack_lexer_litteral(ctx)?;

        // You can even have numbers in them, so we need to eat in a loop,
        // in case of weird things like int9float2

        eat_many!(ctx, lexer::Token::Litteral(..), lexer::Token::Numeric(..), lexer::Token::Underscore);
        
        // Technical info: Type annotations don't do anything, but they help some people to feel more comfortable.
    }

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

    {
        match ctx.next() {
            lexer::Token::Bang => (),
            lexer::Token::QuestionMark => (),
            got => {
                return Err(ParserError::UnexpectedToken {
                    expected: {
                        let list = vec!["!", "?"];
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
                    got: got.variant_name().to_string(),
                });
            }
        }
    }

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
    if let Ok(int) = parse_int_litteral(ctx) {
        return Ok(int);
    }

    parse_string_litteral(ctx)
}

pub fn parse_int_litteral(ctx: &mut ParserContext) -> Result<ast::Litteral, ParserError> {
    // eat_many!(ctx, lexer::Token::Space); ??

    let neg = next_token!(ctx, lexer::Token::Minus).is_ok();

    let mut digits = match next_token!(ctx, lexer::Token::Numeric(..))? {
        lexer::Token::Numeric(digits) => digits,
        _ => unreachable!(),
    };

    loop {
        let next = ctx.next();

        match next {
            lexer::Token::Numeric(vec) => digits.extend_from_slice(&vec),
            lexer::Token::Underscore => (), // just ignore, they are here for the user to feel better
            _ => {
                ctx.backtrack();
                break;
            }
        }
    }

    let n = digits.iter().try_fold(0i64, |acc, d| {
        let (acc, overflowed) = acc.overflowing_mul(10);
        if overflowed {
            return Err(ParserError::IntValueTooLarge(
                digits.iter().map(ToString::to_string).collect::<String>(),
            ));
        }

        let (acc, overflowed) = acc.overflowing_add((*d).into());
        if overflowed {
            return Err(ParserError::IntValueTooLarge(
                digits.iter().map(ToString::to_string).collect::<String>(),
            ));
        }

        Ok(acc)
    })?;

    Ok(ast::Litteral::Int(if neg { -n } else { n }))
}
pub fn parse_float_litteral(ctx: &mut ParserContext) -> Result<ast::Litteral, ParserError> {
    Ok(ast::Litteral::Float(0.0))
}
pub fn parse_string_litteral(ctx: &mut ParserContext) -> Result<ast::Litteral, ParserError> {
    eat_many!(ctx, lexer::Token::SingleQuote, lexer::Token::DoubleQuote);

    let mut text = String::new();

    loop {
        let next = ctx.next();
        match next {
            // This adds characters to the string
            lexer::Token::Dot |
            lexer::Token::Equal |
            lexer::Token::Comma |
            lexer::Token::Space |
            lexer::Token::Underscore |
            lexer::Token::OpenBracket |
            lexer::Token::OpenParenthesis |
            lexer::Token::LessThan |
            lexer::Token::GreaterThan |
            lexer::Token::Plus |
            lexer::Token::Minus |
            lexer::Token::Divide |
            lexer::Token::Multiply | 
            lexer::Token::Colon => {
                let c = char::try_from(next).unwrap();
                text.push(c);
                continue
            },
            lexer::Token::Numeric(vec) => {
                text.push_str(&vec.iter().fold(String::new(), |mut output, n| {
                    use std::fmt::Write as _;
                    let _ = write!(output, "{n}");
                    output
                }));
                continue;
            }
            lexer::Token::Litteral(s) => {
                text.push_str(&s);
                continue
            },
            
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
            lexer::Token::Bang if ![lexer::Token::NewLine, lexer::Token::EOF].contains(&ctx.peek()) =>  {
                text.push('!');
                continue;
            }
            lexer::Token::QuestionMark if ![lexer::Token::NewLine, lexer::Token::EOF].contains(&ctx.peek()) => {
                text.push('?');
                continue;
            }

            // Unwanted
            lexer::Token::Bang | lexer::Token::QuestionMark | lexer::Token::OpenBrace | lexer::Token::CloseBrace  => {
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
    let mut _context = ParserContext::new(tokens);

    Ok(Statement::Empty)
}
