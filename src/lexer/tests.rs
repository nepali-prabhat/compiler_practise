#[cfg(test)]
mod tests {
    use crate::lexer::{StringReader, TokenKind};

    #[test]
    fn single_length_tokens() {
        let src = r#"
let 

 type any = {any : int}
 var buffer := getchar()


  /* BODY OF MAIN PROGRAM */
  /* BODY /* OF MAIN */ PROGRAM */
  /* BODY /* OF MAIN */*/
  /* BODY /* OF MAIN */PROGRAM*/
"#;
        let mut sr = StringReader::new(src);
        let mut token = sr.next_token();
        while token.kind != TokenKind::EOF {
            let value = &src[(token.pos.0 as usize)..(token.pos.1 as usize)];
            println!(
                "{:?} \t\t [{}, {}] \t\t{}",
                token.kind, token.pos.0, token.pos.1, value,
            );
            // println!("{}", value);
            token = sr.next_token();
        }
    }
}
