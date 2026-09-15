struct calculator{
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
}

pub mod operations;
pub mod function;
pub mod parser;