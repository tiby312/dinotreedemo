//use axgeom::f32;


///A struct with cached calculations based off of a radius.
#[derive(Copy,Clone,Debug)]
pub struct RadiusProp {
    radius: f32,
    radius2: f32,
    radius2_squared: f32,
    radius_x_root_2_inv: f32,
}
impl RadiusProp {

    #[inline]
    pub fn create(radius: f32) -> RadiusProp {
        let k = radius * 2.0;

        RadiusProp {
            radius: radius,
            radius2: k,
            radius2_squared: k.powi(2),
            radius_x_root_2_inv: radius * (1.0 / 1.4142),
        }
    }

    ///Returns the rdius
    #[inline]
    pub fn radius(&self) -> f32 {
        self.radius
    }
    
    ///Returns the cached radius*2.0
    #[inline]
    pub fn radius2(&self) -> f32 {
        self.radius2
    }
    
    ///Returns the cached radius.powi(2)
    #[inline]
    pub fn radius2_squared(&self) -> f32 {
        self.radius2_squared
    }

    ///Returns the cached radius*(1/1.4142)
    #[inline]
    pub fn radius_x_root_2_inv(&self) -> f32 {
        self.radius_x_root_2_inv
    }
}
