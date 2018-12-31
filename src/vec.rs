use std;
use axgeom;
///A 2d point made up of f32's with a way to get the value on a particular axis easily.
#[repr(transparent)]
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Vec2(pub [f32;2]);

impl Vec2 {
    
    pub fn new(a:f32,b:f32)->Vec2{
        Vec2([a,b])
    }

    pub fn get_axis(&self,axis:impl axgeom::AxisTrait)->&f32{
        if axis.is_xaxis(){
            &self.0[0]
        }else{
            &self.0[1]
        }
    }

    pub fn get_axis_mut(&mut self,axis:impl axgeom::AxisTrait)->&mut f32{
        if axis.is_xaxis(){
            &mut self.0[0]
        }else{
            &mut self.0[1]
        }
    }
    ///Calculates the dot product.
    #[inline(always)]
    pub fn inner_product(self, b: Vec2) -> f32 {
        let a=&self.0;
        let b=&b.0;
        a[0] * b[0] + a[1] * b[1]
    }

    ///Force the length of the vec to of max length nlen.
    ///If the length of the vec is zero, this will panic.
    #[inline(always)]
    pub fn truncate(&mut self, nlen: f32) {
        if self.dis_sqr()<nlen.powi(2){
            *self /= self.dis();
            *self *= nlen;
        }
    }

    #[inline(always)]
    pub fn rotate90(self) -> Vec2 {
        self.rotate_by(Vec2([0.0, 1.0]))
    }

    #[inline(always)]
    pub fn rotate_by(self, b: Vec2) -> Vec2 {
        let a=&self.0;
        let b=&b.0;
        Vec2([a[0] * b[0] - a[1] * b[1],
                  a[0] * b[1] + a[1] * b[0]])

    }

    ///Calculates len using sqrt().
    #[inline(always)]
    pub fn dis(self) -> f32 {
        self.dis_sqr().sqrt()
    }


    #[inline(always)]
    pub fn dis_sqr(self) -> f32 {
        let a=&self.0;
        a[0]*a[0]+a[1]*a[1]
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn add(self, other: Vec2) -> Vec2 {
        let a=&self.0;
        let b=&other.0;
        Vec2([a[0]+b[0],a[1]+b[1]])
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, other: f32) -> Vec2 {
        let a=&self.0;
        Vec2([a[0]*other,a[1]*other])
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn div(self, other: f32) -> Vec2 {
        let a=&self.0;
        Vec2([a[0] / other, a[1] / other])
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn neg(self) -> Vec2 {
        let a=&self.0;
        Vec2([-a[0], -a[1]])
    }
}

impl std::ops::MulAssign<f32> for Vec2 {

    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        let a=&mut self.0;
        a[0]*=rhs;
        a[1]*=rhs;
    }
}

impl std::ops::DivAssign<f32> for Vec2 {

    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        let a=&mut self.0;
        a[0]/=rhs;
        a[1]/=rhs;
    }
}

impl std::ops::AddAssign for Vec2 {

    #[inline(always)]
    fn add_assign(&mut self, other: Vec2) {
        let a=&mut self.0;
        let b=&other.0;
        a[0]+=b[0];
        a[1]+=b[1];
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn sub(self, other: Vec2) -> Vec2 {
        let a=&self.0;
        let b=&other.0;
        Vec2([a[0]-b[0],a[1]-b[1]])
    }
}
