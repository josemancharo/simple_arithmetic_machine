use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Sub, Shr, Shl, Not, Neg};

#[derive(Clone, Copy, Debug)]
pub enum Real {
    Float(f64),
    Int(i64),
}

impl Add for Real {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Real::Float(x) => match other {
                Real::Float(y) => Real::Float(x + y),
                Real::Int(y) => Real::Float(x + (y as f64)),
            },
            Real::Int(x) => match other {
                Real::Float(y) => Real::Float((x as f64) + y),
                Real::Int(y) => Real::Int(x + y),
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
            },
            Real::Int(x) => match other {
                Real::Float(y) => Real::Float((x as f64) - y),
                Real::Int(y) => Real::Int(x - y),
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
            },
            Real::Int(x) => match other {
                Real::Float(y) => Real::Float((x as f64) * y),
                Real::Int(y) => Real::Int(x * y),
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
            },
            Real::Int(x) => match other {
                Real::Float(y) => Real::Float((x as f64) / y),
                Real::Int(y) => Real::Int(x / y),
            },
        }
    }
}

impl Rem for Real {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        match self {
            Real::Float(x) => match other {
                Real::Float(y) => Real::Float(x % y),
                Real::Int(y) => Real::Float(x % (y as f64)),
            },
            Real::Int(x) => match other {
                Real::Float(y) => Real::Float((x as f64) % y),
                Real::Int(y) => Real::Int(x % y),
            },
        }
    }
}

impl ToString for Real {
    fn to_string(&self) -> String {
        match &self {
            &Real::Float(x) => x.to_string(),
            &Real::Int(x) => x.to_string(),
        }
    }
}

impl Into<f64> for Real {
    fn into(self) -> f64 {
        match self {
            Real::Float(x) => x,
            Real::Int(x) => x as f64,
        }
    }
}

impl Into<i64> for Real {
    fn into(self) -> i64 {
        match self {
            Real::Float(x) => x as i64,
            Real::Int(x) => x,
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
        }
    }
}

impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            _ => {
                let a: f64 = self.clone().into();
                let b: f64 = other.clone().into();
                a == b
            }
        }
    }
}
