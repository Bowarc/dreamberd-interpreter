#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use parser::*;

    #[test]
    fn basic() {
        let src = r#"var const name: int = 5!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let mut ctx = ParserContext::new(&tokens);
        let parsed = parse_var_assignment(&mut ctx);

        assert_debug_snapshot!((tokens, parsed));
    }
    
    #[test]
    fn weird_spaces() {
        let src = r#"var const name : int = 5!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let mut ctx = ParserContext::new(&tokens);
        let parsed = parse_var_assignment(&mut ctx);

        assert_debug_snapshot!((tokens, parsed));
    }
    
    #[test]
    fn weird_type_names() {
        let src = r#"var const name: Int9_ezeze1 = 5!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let mut ctx = ParserContext::new(&tokens);
        let parsed = parse_var_assignment(&mut ctx);

        assert_debug_snapshot!((tokens, parsed));
    }
}
