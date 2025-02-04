#[cfg(test)]
mod tests {
    use lexer::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_single_character_tokens() {
        assert_debug_snapshot!(scan_one("."));
        assert_debug_snapshot!(scan_one("!"));
        assert_debug_snapshot!(scan_one("="));
        assert_debug_snapshot!(scan_one(","));
        assert_debug_snapshot!(scan_one("["));
        assert_debug_snapshot!(scan_one("]"));
        assert_debug_snapshot!(scan_one("("));
        assert_debug_snapshot!(scan_one(")"));
        assert_debug_snapshot!(scan_one("'"));
        assert_debug_snapshot!(scan_one("\""));
        assert_debug_snapshot!(scan_one("<"));
        assert_debug_snapshot!(scan_one(">"));
        assert_debug_snapshot!(scan_one("+"));
        assert_debug_snapshot!(scan_one("-"));
        assert_debug_snapshot!(scan_one("/"));
        assert_debug_snapshot!(scan_one("*"));
    }

    #[test]
    fn test_numeric_tokens() {
        assert_debug_snapshot!(scan_one("123"));
        assert_debug_snapshot!(scan_one("4567"));
        assert_debug_snapshot!(scan_one("0"));
    }

    #[test]
    fn test_literal_tokens() {
        assert_debug_snapshot!(scan_one("hello"));
        assert_debug_snapshot!(scan_one("world"));
        assert_debug_snapshot!(scan_one("foo bar"));
    }

    #[test]
    fn test_mixed_tokens() {
        assert_debug_snapshot!(scan_one("const var scores = [3, 2, 5]!"));
        assert_debug_snapshot!(scan_one("const const x = 10 + 20!"));
        assert_debug_snapshot!(scan_one("if (x < 5) { return! }"));
    }

    #[test]
    fn test_edge_cases() {
        assert_debug_snapshot!(scan_one(""));

        assert_debug_snapshot!(scan_one("   "));
        assert_debug_snapshot!(scan_one("\t\t\t"));
        assert_debug_snapshot!(scan_one("\n\n\n"));

        assert_debug_snapshot!(scan_one("@#$%^&*()"));

        assert_debug_snapshot!(scan_one(&"a".repeat(1000)));

        assert_debug_snapshot!(scan_one("!!!"));
        assert_debug_snapshot!(scan_one("..."));
        assert_debug_snapshot!(scan_one(",,,"));

        assert_debug_snapshot!(scan_one("123abc!@#"));

        assert_debug_snapshot!(scan_one("[(])"));
        assert_debug_snapshot!(scan_one("[{]}"));

        assert_debug_snapshot!(scan_one("\"Hello\\nWorld\""));

        assert_debug_snapshot!(scan_one("00123"));
        assert_debug_snapshot!(scan_one("0001"));

        assert_debug_snapshot!(scan_one("Hello\x00World"));

        assert_debug_snapshot!(scan_one("12345678901234567890"));
    }
}
