use crate::interpreter::data_types::Real;

impl Real {
    pub fn sin(&self) -> Real {
        let x: f64 = self.clone().into();
        Real::Float(x.sin())
    }
    pub fn tan(&self) -> Real {
        let x: f64 = self.clone().into();
        Real::Float(x.tan())
    }
    pub fn cos(&self) -> Real {
        let x: f64 = self.clone().into();
        Real::Float(x.cos())
    }
    pub fn asin(&self) -> Real {
        let x: f64 = self.clone().into();
        Real::Float(x.asin())
    }
    pub fn atan(&self) -> Real {
        let x: f64 = self.clone().into();
        Real::Float(x.atan())
    }
    pub fn acos(&self) -> Real {
        let x: f64 = self.clone().into();
        Real::Float(x.acos())
    }
}