use crate::interpreter::data_types::Real;

impl Real {
    pub fn log(self, y: Real) -> Real {
        let x: f64 = self.into();
        let y: f64 = y.into();
        Real::Float(x.log(y))
    }
    
    pub fn pow(self, y: Real) -> Real {
        return match self {
            Real::Int(x) => {
                match y {
                    Real::Int(y) => {
                        if let Ok(y) = y.to_string().parse::<u32>() {
                            Real::Int(x.pow(y))
                        }
                        else {
                            Real::Float((x as f64).powf(y as f64))
                        }
                    },
                    _ => {
                        let y: f64 = y.into();
                        Real::Float((x as f64).powf(y))
                    }
                }
            }
            _ => {
                let y: f64 = y.into();
                let x: f64 = self.into();
                Real::Float(x.powf(y))
            }
        }
        
    }
    
    pub fn ln(self) -> Real {
        return match self {
            Real::Int(x) => {
                Real::Float((x as f64).ln())
            },
            _ => {
                let x: f64 = self.into();
                Real::Float(x.ln())
            }
        }
    }
}
