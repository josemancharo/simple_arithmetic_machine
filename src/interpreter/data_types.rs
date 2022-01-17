use std::ops::{Add, Sub, Mul, Div, Rem};

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Real {
    Float(f64),
    Int(i64),
}

impl Add for Real {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Real::Float(x) => { 
                match other {
                    Real::Float(y) => { Real::Float(x + y) }
                    Real::Int(y) => { Real::Float(x + (y as f64)) }
                }
             }
             Real::Int(x) => {
                 match other {
                     Real::Float(y) => { Real::Float((x as f64) + y) }
                     Real::Int(y) => { Real::Int(x + y) }
                 }
             }
        }
    }
}

impl Sub for Real {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match self {
            Real::Float(x) => { 
                match other {
                    Real::Float(y) => { Real::Float(x - y) }
                    Real::Int(y) => { Real::Float(x - (y as f64)) }
                }
             }
             Real::Int(x) => {
                 match other {
                     Real::Float(y) => { Real::Float((x as f64) - y) }
                     Real::Int(y) => { Real::Int(x - y) }
                 }
             }
        }
    }
}

impl Mul for Real {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match self {
            Real::Float(x) => { 
                match other {
                    Real::Float(y) => { Real::Float(x * y) }
                    Real::Int(y) => { Real::Float(x * (y as f64)) }
                }
             }
             Real::Int(x) => {
                 match other {
                     Real::Float(y) => { Real::Float((x as f64) * y) }
                     Real::Int(y) => { Real::Int(x * y) }
                 }
             }
        }
    }
}

impl Div for Real {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match self {
            Real::Float(x) => { 
                match other {
                    Real::Float(y) => { Real::Float(x / y) }
                    Real::Int(y) => { Real::Float(x / (y as f64)) }
                }
             }
             Real::Int(x) => {
                 match other {
                     Real::Float(y) => { Real::Float((x as f64) / y) }
                     Real::Int(y) => { Real::Int(x / y) }
                 }
             }
        }
    }
}

impl Rem for Real {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        match self {
            Real::Float(x) => { 
                match other {
                    Real::Float(y) => { Real::Float(x % y) }
                    Real::Int(y) => { Real::Float(x % (y as f64)) }
                }
             }
             Real::Int(x) => {
                 match other {
                     Real::Float(y) => { Real::Float((x as f64) % y) }
                     Real::Int(y) => { Real::Int(x % y) }
                 }
             }
        }
    }
}

impl ToString for Real {
    fn to_string(&self) -> String {
        match &self {
            &Real::Float(x ) => { x.to_string() }
            &Real::Int(x) => { x.to_string() }
        }
    }
}