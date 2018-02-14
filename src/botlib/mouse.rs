//use axgeom::f32;
use axgeom;
use std;
use super::prop::RadiusProp;

#[derive(Copy,Clone,Debug)]
pub struct MouseProp {
    pub radius: RadiusProp,
    pub force: f32,
}



#[derive(Copy,Clone,Debug)]
pub struct Mouse{
    pub mouse_prop: MouseProp,
    pub midpoint:axgeom::Vec2,
    pub rect:axgeom::Rect<f32>
}
impl Mouse{
    pub fn new(pos:&axgeom::Vec2,prop:&MouseProp)->Mouse{
        let mut m:Mouse=unsafe{std::mem::uninitialized()};
        m.mouse_prop=*prop;
        m.move_to(pos);
        m
    }

    pub fn get_rect(&self)->&axgeom::Rect<f32>{
        &self.rect
    }
    pub fn get_midpoint(&self)->&axgeom::Vec2{
        &self.midpoint
    }
    pub fn get_radius(&self)->f32{
        self.mouse_prop.radius.radius()
    }
    pub fn move_to(&mut self,pos:&axgeom::Vec2){
        self.midpoint=*pos;
        self.rect=axgeom::Rect::from_pos_and_radius(pos,self.mouse_prop.radius.radius());
    }
}

