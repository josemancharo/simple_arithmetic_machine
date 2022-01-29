use crate::interpreter::data_types::Real;


pub fn log(x: Real, y: Real) -> Real {
    let y_float = match y {
        Real::Float(x) => {
            x
        }
        Real::Int(x) => {
            x as f64
        }
    };
    return match x {
        Real::Float(x) => {
            Real::Float(x.log(y_float))
        }
        Real::Int(x) => {
            Real::Float((x as f64).log(y_float))
        }
    }
}

pub fn pow(x: Real, y: Real) -> Real {
    return match x {
        Real::Float(x) => {
            match y {
                Real::Float(y) => Real::Float(x.powf(y)), 
                Real::Int(y) => Real::Float(x.powf(y as f64))
            }
        }
        Real::Int(x) => {
            match y {
                Real::Float(y) =>  Real::Float((x as f64).powf(y)), 
                Real::Int(y) => {
                    if let Ok(y) = y.to_string().parse::<u32>() {
                        Real::Int(x.pow(y))
                    }
                    else {
                        Real::Float((x as f64).powf(y as f64))
                    }
                }
            }
        }
    }
}

pub fn ln(x: Real) -> Real {
    return match x {
        Real::Float(x) => {
            Real::Float(x.ln())
        }
        Real::Int(x) => {
            Real::Float((x as f64).ln())
        }
    }
}