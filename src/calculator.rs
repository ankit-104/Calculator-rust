pub struct calculator{
    pub expression: String,
    pub result : f64,
}

impl calculator{
    pub fn new()-> Self{
        calculator{
            expression:String::new(),
            result: 0.0,
        }
    }

    pub fn evaluate(&mut self,input: &str) -> Result<f64,String>{
        let tokens=token::tokenize(input)?;
        let mut parser=parser::Parser::new(tokens);
        let result=parser.parse_expression()?;
        self.expression=input.to_string();
        self.result=result;
        Ok(result)
    }
}

pub mod operations;
pub mod function;
pub mod token;
pub mod parser;

use parser::Parser;


