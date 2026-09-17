#[derive(Debug)]
pub enum Token{
    number(f64),
    plus,
    minus,
    multiply,
    devide,
    leftParen,
    rightParen,
    factorial,
    power,
    identifier(String)
}

pub fn tokenize(input:&str)-> Result<Vec<Token>,String>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars=input.chars().peekable();
    while let Some(&c)=chars.peek(){
        // Skip white space
        if c.is_whitespace(){
            chars.next();
            continue;
        }

        // Number
        if c.is_ascii_digit() || c=='.'{
            let mut number = String::new();
            while let Some(&next)=chars.peek(){
                if next.is_ascii_digit() || next=='.'{
                    number.push(next);
                    chars.next();
                }else{
                    break;
                }
            }
            let value = number
                .parse::<f64>()
                .map_err(|_| format!("Invalid number: {number}"))?;
            tokens.push(Token::number(value));
            continue;
        }

        // String like sin,cos,mod etc.
        if c.is_ascii_alphabetic(){
            let mut identifier_str=String::new();
            while let Some(&next)=chars.peek(){ 
                if next.is_ascii_alphabetic(){
                   identifier_str.push(next);
                   chars.next();
                   continue; 
                }else{
                    break;
                }
            }
            tokens.push(Token::identifier(identifier_str));
            continue;
        }

        // Mathematical functions
        let token = match c{
            '+'  => Token::plus,
            '-' => Token::minus,
            '*' => Token::multiply,
            '/' => Token::devide,
            '(' => Token::leftParen,
            ')' => Token::rightParen,
            '!' => Token::factorial,
            '^'=> Token::power,
            _ => return Err(format!("Unexpected character: {c}")),
        };
        chars.next();
        tokens.push(token);
    }
    Ok(tokens)
}
