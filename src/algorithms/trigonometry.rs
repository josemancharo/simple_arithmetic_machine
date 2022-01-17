use crate::interpreter::data_types::Real;

pub fn sin(x: Real) -> Real {
    return match x {
        Real::Float(x) => {
            Real::Float(x.sin())
        }
        Real::Int(x) => {
            Real::Float((x as f64).sin())
        }
    }
}

pub fn asin(x: Real) -> Real {
    return match x {
        Real::Float(x) => {
            Real::Float(x.asin())
        }
        Real::Int(x) => {
            Real::Float((x as f64).asin())
        }
    }
}

pub fn cos(x: Real) -> Real {
    return match x {
        Real::Float(x) => {
            Real::Float(x.cos())
        }
        Real::Int(x) => {
            Real::Float((x as f64).cos())
        }
    }
}

pub fn acos(x: Real) -> Real {
    return match x {
        Real::Float(x) => {
            Real::Float(x.acos())
        }
        Real::Int(x) => {
            Real::Float((x as f64).acos())
        }
    }
}

pub fn tan(x: Real) -> Real {
    return match x {
        Real::Float(x) => {
            Real::Float(x.tan())
        }
        Real::Int(x) => {
            Real::Float((x as f64).tan())
        }
    }
}

pub fn atan(x: Real) -> Real {
    return match x {
        Real::Float(x) => {
            Real::Float(x.atan())
        }
        Real::Int(x) => {
            Real::Float((x as f64).atan())
        }
    }
}