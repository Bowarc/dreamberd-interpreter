use std::collections::HashMap;

use crate::Keyword;

#[derive(Default, Debug)]
pub struct ParserContext<'a> {
    tokens: &'a [lexer::Token],
    keyword_aliases: HashMap<String, Keyword>,
    peek_index: Option<usize>,
    index: Option<usize>,
}

impl<'a> ParserContext<'a> {
    pub fn new(tokens: &'a [lexer::Token]) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    pub fn set_kw_alias(&mut self, kw: Keyword, alias: String) {
        self.keyword_aliases.insert(alias, kw);
    }
    pub fn is_kw(&self, s: &str) -> Option<Keyword> {
        Keyword::parse(s, &self.keyword_aliases)
    }
    pub fn aliases_for(&'a self, kw: Keyword) -> Vec<&'a str> {
        self.keyword_aliases
            .iter()
            .filter_map(|(k, v)| if v == &kw { Some(k) } else { None })
            .map(|s| s.as_str())
            .collect()
    }

    pub fn advance(&mut self) {
        let old = self.index;
        if self.index.is_none() {
            self.index = Some(0);
            self.peek_index = self.index;
        } else {
            *self.index.as_mut().unwrap() += 1;
            self.peek_index = self.index;
        }
        println!("advancing ({:?} -> {:?})", old, self.index);
    }
    pub fn backtrack(&mut self) {
        let old = self.index;
        if self.index.is_none() {
            self.index = Some(0);
            self.peek_index = self.index;
        } else {
            let (new, overflowed) = self.index.as_mut().unwrap().overflowing_sub(1);
            self.index = if overflowed { None } else { Some(new) };
            self.peek_index = self.index;
        }
        println!("backtracking ({:?} -> {:?})", old, self.index);
    }

    pub fn peek(&mut self) -> lexer::Token {
        if self.peek_index.is_none() {
            self.peek_index = Some(0)
        } else {
            *self.peek_index.as_mut().unwrap() += 1;
        }
        println!("peeking {:?}", self.tokens.get(self.peek_index.unwrap()));
        self.tokens
            .get(self.peek_index.unwrap())
            .cloned()
            .unwrap_or(lexer::Token::EOF)
    }

    pub fn next(&mut self) -> lexer::Token {
        self.advance();
        println!("next {:?}", self.tokens.get(self.index.unwrap()));
        self.tokens
            .get(self.index.unwrap())
            .cloned()
            .unwrap_or(lexer::Token::EOF)
    }
}
