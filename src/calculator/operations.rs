pub fn add(a:f64,b:f64) ->f64 {
    a+b
}

pub fn subtract(a:f64,b:f64) -> f64{
    a-b
}

pub fn multiply(a:f64,b:f64) -> f64{
    a*b
}

pub fn devide(a:f64,b:f64) -> Result<f64,String>{
    if b==0.0{
        Err("Can not devide a number by 0".to_string())
    } else{
        Ok(a/b)
    }
}

pub fn modulo(a:f64,b:f64) -> Result<f64,String>{
    if b==0.0{
        Err("Can not devide a number by 0".to_string())
    }else{
        Ok(a%b)
    }
}

pub fn power(a:f64,b:f64) -> f64{
    a.powf(b) 
}

