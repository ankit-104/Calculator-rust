pub use super::token::Token;

pub struct Parser{
    tokens: Vec<Token>,
    position: usize,
}

impl Parser{
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser{
            tokens,
            position:0,
        }
    }
    
    pub fn parse_expression(&mut self)->Result<f64,String>{
        let result=self.parse_add_sub()?;
        if self.position<self.tokens.len(){
            return Err(format!("Unexpected token {:?}",self.tokens[self.position]).to_string());
        }
        Ok(result)
    }

    fn parse_primary(&mut self) -> Result<f64,String>{
        match self.tokens.get(self.position){
            Some(Token::number(val))=>{
                let val=*val;
                self.position+=1;
                Ok(val)
            }
            Some(Token::identifier(_))=> self.parse_function(),
            Some(Token::leftParen) => self.parse_parantheses(),
            Some(Token) => Err(format!("Unexpected token {:?}",Token).to_string()),
            None => Err("Unexpected end of expression".to_string()),
        }
    }
    
    // handles + and -
    fn parse_add_sub(&mut self) -> Result<f64,String>{
        let mut result=self.parse_mul_div()?;
        loop{
            match self.tokens.get(self.position){
                Some(Token::plus) => {
                    self.position+=1;
                    let right=self.parse_mul_div()?;
                    result+=right;
                }
                Some(Token::minus) =>{
                    self.position+=1;
                    let right=self.parse_mul_div()?;
                    result-=right;
                }
                _ => break,
            }
        }
        Ok(result)
    }
    
    // This handles +,- and %
    fn parse_mul_div(&mut self)->Result<f64,String>{
        let mut result=self.parse_power()?;
        loop{
            match self.tokens.get(self.position){
                Some(Token::devide)=>{
                    self.position+=1;
                    let right=self.parse_power()?;
                    result=super::operations::devide(result, right)?;
                }
                Some(Token::multiply)=>{
                    self.position+=1;
                    let right=self.parse_power()?;
                    result*=right;
                }
                Some(Token::identifier(name)) if name=="mod" =>{
                    self.position+=1;
                    let right=self.parse_power()?;
                    result=super::operations::modulo(result, right)?;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    // This handles power
    fn parse_power(&mut self)-> Result<f64,String>{
        let left=self.parse_unary()?;
        if let Some(Token::power) = self.tokens.get(self.position){
            self.position+=1;
            let right=self.parse_power()?;
            return Ok(left.powf(right));
        }
        Ok(left)
    }
    
    // Its job is to evaluate -5 as negative of 5
    fn parse_unary(&mut self) -> Result<f64,String>{
        match self.tokens.get(self.position){
            Some(Token::plus) => {
                self.position+=1;
                self.parse_unary()
            }
            Some (Token::minus)=>{
                self.position+=1;
                let value=self.parse_unary()?;
                Ok(-value)
            }
            _ => self.parse_postfix(),
        }
    }
    
    // Handels operator like ! which comes after the digit
    fn parse_postfix(&mut self) -> Result<f64,String>{
        let mut result=self.parse_primary()?;
        while let Some(Token::factorial)=self.tokens.get(self.position){
            self.position+=1;
            result=super::function::factorial(result)?;
        }
        Ok(result)
    }
    
    // Handles function such as sin() cos() pi e etc.
    fn parse_function(&mut self) -> Result<f64,String>{
        let name=match self.tokens.get(self.position){
            Some(Token::identifier(name))=> name.clone(),
            _ =>return Err("Expected function ".to_string()),
        };
        self.position += 1;
        match name.as_str(){
            "pi" => Ok(std::f64::consts::PI),
            "e" => Ok(std::f64::consts::E),
            "sin" =>{
                let argument=self.parse_function_argument()?;
                Ok(super::function::sin(argument))
            }
            "cos" => {
                let argument=self.parse_function_argument()?;
                Ok(super::function::cos(argument))
            }
            "tan" =>{
                let argument=self.parse_function_argument()?;
                Ok(super::function::tan(argument))
            }
            "abs" => {
                let argument = self.parse_function_argument()?;
                Ok(super::function::abs(argument))
            }
            "exp" => {
                let argument = self.parse_function_argument()?;
                Ok(super::function::exp(argument))
            }
            "log" => {
                let argument = self.parse_function_argument()?;
                super::function::log(argument)
            }
            "ln" => {
                let argument = self.parse_function_argument()?;
                super::function::ln(argument)
            }
            "sqrt" => {
                let argument = self.parse_function_argument()?;
                super::function::sqrt(argument)
            }
            _ => Err(format!("Unknown function or constant '{name}'")),
        }
    }
    fn parse_function_argument(&mut self) -> Result<f64,String>{
        match self.tokens.get(self.position){
            Some(Token::leftParen)=>{
                self.position+=1;
                let result=self.parse_add_sub()?;
                match self.tokens.get(self.position){
                    Some(Token::rightParen) =>{
                        self.position+=1;
                        Ok(result)
                    }
                    _=> Err("Expected ) after the function".to_string()),
                }
            }
            _ => self.parse_primary(),
        }
    }

    // Parse everything inside ()
    fn parse_parantheses(&mut self) -> Result<f64,String>{
        self.position+=1;
        let result=self.parse_add_sub()?;
        match self.tokens.get(self.position){
            Some(Token::rightParen)=>{
                self.position+=1;
                Ok(result) 
            }
            _ => Err("Expected )".to_string()),
        }
    }
    
}