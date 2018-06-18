use proc_macro2::TokenStream;
use syn::buffer::TokenBuffer;
use syn::synom::{Synom, ParseError};
use syn::{Expr, LitStr};
use syn::token::Comma;
use syn::buffer;

pub struct Cursor<'a> {
    cur: buffer::Cursor<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(buf: &'a TokenBuffer) -> Self {
        Cursor {
            cur: buf.begin()
        }
    }

    pub fn parse<T: Synom>(&mut self) -> Result<T, ParseError> {
        let (v, cur) = T::parse(self.cur)?;
        self.cur = cur;
        Ok(v)
    }

    pub fn eof(&self) -> bool {
        self.cur.eof()
    }
}

pub fn parse_args(tokens: TokenStream) -> Result<(LitStr, Vec<Expr>), ParseError> {
    let buf = TokenBuffer::new2(tokens.into());
    let mut cur = Cursor::new(&buf);
    let s : LitStr = cur.parse()?;
    let mut exprs = Vec::new();

    while !(cur.eof()) {
        let _ : Comma = cur.parse()?;
        let arg : Expr = cur.parse()?;
        exprs.push(arg);
    }

    Ok((s, exprs))
}
