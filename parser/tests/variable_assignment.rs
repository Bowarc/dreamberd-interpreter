#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use parser::*;

    #[test]
    fn const_const_int_declaration() {
        let src = r#"const const cool_var_name = 5!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn question_mark_end() {
        let src = r#"const const cool_var_name = 5?"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn const_const_large_int_declaration() {
        let src = r#"const const cool_var_name = 55479757570878780!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn const_const_int_with_spaces_declaration() {
        let src = r#"const const cool_var_name = 55479757 570878780!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn const_const_int_with_underscores_as_delimiter_declaration() {
        let src = r#"const const cool_var_name = 5_547_975_757_0878_780!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn const_const_neg_int_declaration() {
        let src = r#"const const cool_var_name = -5!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }

    #[test]
    fn var_var_int_declaration() {
        let src = r#"var var cool_var_name = 5!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    
    #[test]
    fn var_var_simple_string_declaration() {
        let src = r#"var var cool_var_name = "hi"!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    
    #[test]
    fn var_var_string_no_quote_declaration() {
        let src = r#"var var cool_var_name = hi!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }

    #[test]
    fn missing_internal_mutability() {
        let src = r#"var cool_var_name = 15!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn invalid_external_mutability_keyword() {
        let src = r#"v const cool_var_name = 15!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn missing_name() {
        let src = r#"var const = 15!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn missing_equal() {
        let src = r#"var const name 15!"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn missing_value() {
        let src = r#"var const name = !"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
    #[test]
    fn missing_bang() {
        let src = r#"var const name = 5"#;

        let tokens = lexer::scan(src.as_bytes()).unwrap();
        let parsed = parse_var_assignment(&mut ParserContext::new(&tokens));

        assert_debug_snapshot!((tokens, parsed));
    }
}
