#[cfg(test)]
mod tests {
    use crate::lexer::{StringReader, TokenKind};

    #[test]
    fn single_length_tokens() {
        let src = "let 

 type any = {any : int}
 var buffer := getchar()
";
        let mut sr = StringReader::new(src);
        let mut token = sr.next_token();
        while token.kind != TokenKind::EOF {
            let value = &src[(token.pos.0 as usize)..(token.pos.1 as usize)];
            println!("sr: {:?} `{}`", token, value);
            token = sr.next_token();
        }
    }
}
