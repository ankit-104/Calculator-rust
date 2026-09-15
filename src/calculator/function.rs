pub fn sqrt(x:f64)->Result<f64,String>{
    if x<0.0{
        Err(format!("Can't find Square root of {x}").to_string())
    }else{
        Ok(x.sqrt())
    }
}

pub fn abs(x:f64)->f64{
    x.abs()
}

pub fn exp(x:f64) -> f64{
    x.exp()
}

pub fn log(x:f64) -> Result<f64,String>{
    if x<=0.0{
        Err(format!("Can't find log of {x}").to_string())
    }else{
        Ok(x.log10())
    }
}
pub fn ln(x:f64) -> Result<f64,String>{
    if x<=0.0{
        Err(format!("Can't find ln of {x}").to_string())
    }else{
        Ok(x.ln())
    }
}

fn degree_to_radian(x:f64) -> f64{
    use std::f64::consts::PI;
    x*(PI/180.0)
}

pub fn sin(mut x:f64)->f64{
    x=degree_to_radian(x);
    x.sin()
}

pub fn cos(mut x:f64)->f64{
    x=degree_to_radian(x);
    x.cos()
}

pub fn tan(mut x:f64)->f64{
    x=degree_to_radian(x);
    x.tan()
}

pub fn factorial(x:f64) -> Result<f64,String>{
    if x.fract()!=0.0 || x<0.0{
        return Err(format!("Invalid nuber for finding factorial {x}").to_string());
    }
    let n=x as u64;
    let mut fact:f64=1.0;
    for i in 1..n{
        fact=fact*(i as f64);
    }
    Ok(fact)
}