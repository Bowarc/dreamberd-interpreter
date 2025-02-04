#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use parser::*;

    #[test]
    fn normal() {
        let src = r#""Normal string""#;

        let tokens = lexer::scan(String::from(src).as_bytes()).unwrap();
        let parsed = parse_string_litteral(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn multiple_quotes() {
        let src = r#""""Mutliple quotes"""""#;

        let tokens = lexer::scan(String::from(src).as_bytes()).unwrap();
        let parsed = parse_string_litteral(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn different_quotes() {
        let src = r#"'"'diferent quotes'"'"#;

        let tokens = lexer::scan(String::from(src).as_bytes()).unwrap();
        let parsed = parse_string_litteral(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn unmatched_quotes() {
        let src = r#"'"'unmatched quotes""#;

        let tokens = lexer::scan(String::from(src).as_bytes()).unwrap();
        let parsed = parse_string_litteral(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn no_quotes_start_quotes() {
        let src = r#"unmatched quotes""#;

        let tokens = lexer::scan(String::from(src).as_bytes()).unwrap();
        let parsed = parse_string_litteral(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
}
