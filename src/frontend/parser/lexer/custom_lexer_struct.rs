use logos::{Lexer, Logos, Source, Span};

pub struct SavedLexerPosition(usize);

#[derive(Debug)]
struct TokenWithExtras<TokenType> {
    pub token: TokenType,
    pub span: Span,
}

pub struct CustomLexerStruct<'a, TokenType: core::fmt::Debug + Logos<'a> + Clone> {
    orig_lexer: Lexer<'a, TokenType>,

    tokens: Vec<TokenWithExtras<TokenType>>,

    next_token_index: usize,
}

impl<'a, TokenType: Clone + core::fmt::Debug + Logos<'a>> From<Lexer<'a, TokenType>>
    for CustomLexerStruct<'a, TokenType>
{
    fn from(lexer: Lexer<'a, TokenType>) -> CustomLexerStruct<'a, TokenType> {
        CustomLexerStruct {
            orig_lexer: lexer,
            tokens: Vec::new(),
            next_token_index: 0,
        }
    }
}

impl<'a, TokenType: Clone + core::fmt::Debug + Logos<'a>> Iterator
    for CustomLexerStruct<'a, TokenType>
{
    type Item = TokenType;

    fn next(&mut self) -> Option<TokenType> {
        if self.tokens.len() > self.next_token_index {
            let token = self.tokens[self.next_token_index].token.clone();
            self.next_token_index += 1;
            Some(token)
        } else {
            let optional_token = self.orig_lexer.next();

            if let Some(token) = &optional_token {
                self.tokens.push(TokenWithExtras {
                    token: token.clone(),
                    span: self.orig_lexer.span(),
                });
            }

            self.next_token_index += 1;

            optional_token
        }
    }
}

impl<'a, TokenType: Clone + core::fmt::Debug + Logos<'a>> CustomLexerStruct<'a, TokenType> {
    pub fn span(&self) -> Option<Span> {
        if self.tokens.len() > self.next_token_index - 1 {
            Some(self.tokens[self.next_token_index - 1].span.clone())
        } else {
            None
        }
    }

    pub fn slice(&self) -> Option<&'a <TokenType::Source as Source>::Slice> {
        if let Some(span) = self.span() {
            self.orig_lexer.source().slice(span)
        } else {
            None
        }
    }
}

impl<'a, TokenType: Clone + core::fmt::Debug + Logos<'a>> CustomLexerStruct<'a, TokenType> {
    pub fn peek(&mut self) -> Option<TokenType> {
        let optional_token = self.next();

        self.next_token_index -= 1;

        optional_token
    }

    pub fn peek_span(&mut self) -> Option<Span> {
        if self.tokens.len() > self.next_token_index {
            Some(self.tokens[self.next_token_index].span.clone())
        } else {
            self.peek();
            if self.tokens.len() > self.next_token_index {
                Some(self.tokens[self.next_token_index].span.clone())
            } else {
                None
            }
        }
    }

    pub fn peek_slice(&mut self) -> Option<&'a <TokenType::Source as Source>::Slice> {
        if let Some(peeked_span) = self.peek_span() {
            self.orig_lexer.source().slice(peeked_span)
        } else {
            None
        }
    }
}

impl<'a, TokenType: Clone + core::fmt::Debug + Logos<'a>> CustomLexerStruct<'a, TokenType> {
    pub fn save_position(&self) -> SavedLexerPosition {
        SavedLexerPosition(self.next_token_index)
    }

    pub fn return_to_position(&mut self, position: SavedLexerPosition) {
        self.next_token_index = position.0
    }
}

impl<'a, TokenType: Clone + core::fmt::Debug + Logos<'a>> CustomLexerStruct<'a, TokenType> {
    pub fn source(&self) -> &'a TokenType::Source {
        self.orig_lexer.source()
    }
}
