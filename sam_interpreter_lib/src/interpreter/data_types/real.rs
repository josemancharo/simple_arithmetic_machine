use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use num_rational::Rational64;

#[derive(Clone, Copy, Debug)]
pub enum Real {
    Float(f64),
    Int(i64),
    Rational(Rational64),
}

impl Add for Real {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Real::Float(x) => match other {
                Real::Float(y) => Real::Float(x + y),
                Real::Int(y) => Real::Float(x + (y as f64)),
                Real::Rational(y) => {
                    let (numer, denom) = y.into();
                    Real::Float(x + ((numer as f64) / (denom as f64)))
                }
            },
            Real::Int(x) => match other {
                Real::Float(y) => Real::Float((x as f64) + y),
                Real::Int(y) => Real::Int(x + y),
                Real::Rational(y) => Real::Rational(Rational64::new(x, 1) + y),
            },
            Real::Rational(x) => match other {
                Real::Float(y) => {
                    let (numer, denom) = x.into();
                    Real::Float(((numer as f64) / (denom as f64)) + y)
                }
                Real::Int(y) => Real::Rational(x + y),
                Real::Rational(y) => Real::Rational(x + y),
            },
        }
    }
}

impl Sub for Real {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match self {
            Real::Float(x) => match other {
                Real::Float(y) => Real::Float(x - y),
                Real::Int(y) => Real::Float(x - (y as f64)),
                Real::Rational(y) => {
                    let (numer, denom) = y.into();
                    Real::Float(x - ((numer as f64) / (denom as f64)))
                }
            },
            Real::Int(x) => match other {
                Real::Float(y) => Real::Float((x as f64) - y),
                Real::Int(y) => Real::Int(x - y),
                Real::Rational(y) => Real::Rational(Rational64::new(x, 1) - y),
            },
            Real::Rational(x) => match other {
                Real::Float(y) => {
                    let (numer, denom) = x.into();
                    Real::Float(((numer as f64) / (denom as f64)) - y)
                }
                Real::Int(y) => Real::Rational(x - y),
                Real::Rational(y) => Real::Rational(x - y),
            },
        }
    }
}

impl Mul for Real {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match self {
            Real::Float(x) => match other {
                Real::Float(y) => Real::Float(x * y),
                Real::Int(y) => Real::Float(x * (y as f64)),
                Real::Rational(y) => {
                    let (numer, denom) = y.into();
                    Real::Float(x * ((numer as f64) / (denom as f64)))
                }
            },
            Real::Int(x) => match other {
                Real::Float(y) => Real::Float((x as f64) * y),
                Real::Int(y) => Real::Int(x * y),
                Real::Rational(y) => Real::Rational(Rational64::new(x, 1) * y),
            },
            Real::Rational(x) => match other {
                Real::Float(y) => {
                    let (numer, denom) = x.into();
                    Real::Float(((numer as f64) / (denom as f64)) * y)
                }
                Real::Int(y) => Real::Rational(x * y),
                Real::Rational(y) => Real::Rational(x * y),
            },
        }
    }
}

impl Div for Real {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match self {
            Real::Float(x) => match other {
                Real::Float(y) => Real::Float(x / y),
                Real::Int(y) => Real::Float(x / (y as f64)),
                Real::Rational(y) => {
                    let (numer, denom) = y.into();
                    Real::Float(x / ((numer as f64) / (denom as f64)))
                }
            },
            Real::Int(x) => match other {
                Real::Float(y) => Real::Float((x as f64) / y),
                Real::Int(y) => Real::Int(x / y),
                Real::Rational(y) => Real::Rational(Rational64::new(x, 1) / y),
            },
            Real::Rational(x) => match other {
                Real::Float(y) => {
                    let (numer, denom) = x.into();
                    Real::Float(((numer as f64) / (denom as f64)) / y)
                }
                Real::Int(y) => Real::Rational(x / y),
                Real::Rational(y) => Real::Rational(x / y),
            },
        }
    }
}

impl Rem for Real {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        match self {
            Real::Float(x) => match other {
                Real::Int(y) => Real::Float(x % (y as f64)),
                _ => {
                    let y: f64 = other.into();
                    Real::Float(x % y)
                }
            },
            Real::Int(x) => match other {
                Real::Int(y) => Real::Int(x % y),
                _ => {
                    let y: f64 = other.into();
                    Real::Float(x as f64 % y)
                }
            },
            _ => {
                let (x, y): (f64, f64) = (self.into(), other.into());
                Real::Float(x % y)
            }
        }
    }
}

impl ToString for Real {
    fn to_string(&self) -> String {
        match &self {
            &Real::Float(x) => x.to_string(),
            &Real::Int(x) => x.to_string(),
            &Real::Rational(x) => format!("{}/{}", x.numer(), x.denom()),
        }
    }
}

impl Into<f64> for Real {
    fn into(self) -> f64 {
        match self {
            Real::Float(x) => x,
            Real::Int(x) => x as f64,
            Real::Rational(x) => *x.numer() as f64 / *x.denom() as f64,
        }
    }
}

impl Into<i64> for Real {
    fn into(self) -> i64 {
        match self {
            Real::Int(x) => x,
            _ => { 
                let x: f64 = self.into();
                x as i64 
            },
        }
    }
}

impl PartialOrd for Real {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a: f64 = self.clone().into();
        let b: f64 = other.clone().into();
        return f64::partial_cmp(&a, &b);
    }
}

impl BitAnd for Real {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let a: i64 = self.into();
        let b: i64 = rhs.into();
        return Real::Int(a & b);
    }
}

impl BitOr for Real {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let a: i64 = self.into();
        let b: i64 = rhs.into();
        return Real::Int(a | b);
    }
}

impl BitXor for Real {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let a: i64 = self.into();
        let b: i64 = rhs.into();
        return Real::Int(a ^ b);
    }
}

impl Shr for Real {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        let a: i64 = self.into();
        let b: i64 = rhs.into();
        return Real::Int(a >> b);
    }
}

impl Shl for Real {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        let a: i64 = self.into();
        let b: i64 = rhs.into();
        return Real::Int(a << b);
    }
}

impl Not for Real {
    type Output = Self;

    fn not(self) -> Self::Output {
        let a: i64 = self.into();
        return Real::Int(!a);
    }
}

impl Neg for Real {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Real::Float(x) => Real::Float(-x),
            Real::Int(x) => Real::Int(-x),
            Real::Rational(x) => Real::Rational(Rational64::new(-1, 1) * x),
        }
    }
}

impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Rational(l0), Self::Rational(r0)) => l0 == r0,
            _ => {
                let a: f64 = self.clone().into();
                let b: f64 = other.clone().into();
                a == b
            }
        }
    }
}
