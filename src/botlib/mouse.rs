//use axgeom::f32;
use axgeom;
use std;
use super::prop::RadiusProp;
use vec::Vec2;

#[derive(Copy,Clone,Debug)]
pub struct MouseProp {
    pub radius: RadiusProp,
    pub force: f32,
}



#[derive(Copy,Clone,Debug)]
pub struct Mouse{
    pub mouse_prop: MouseProp,
    pub midpoint:Vec2,
    pub rect:axgeom::Rect<f32>
}
impl Mouse{
    pub fn new(pos:&Vec2,prop:&MouseProp)->Mouse{
        let mut m:Mouse=unsafe{std::mem::uninitialized()};
        m.mouse_prop=*prop;
        m.move_to(pos);
        m
    }

    pub fn get_rect(&self)->&axgeom::Rect<f32>{
        &self.rect
    }
    pub fn get_midpoint(&self)->&Vec2{
        &self.midpoint
    }
    pub fn get_radius(&self)->f32{
        self.mouse_prop.radius.radius()
    }
    pub fn move_to(&mut self,pos:&Vec2){
        self.midpoint=*pos;
        let p=self.midpoint.0;
        let r=self.mouse_prop.radius.radius();
        let r=axgeom::Rect::new(p[0]-r,p[0]+r,p[1]-r,p[1]+r);
        self.rect=r;
        //convert_to_nan(r)
        //self.rect=axgeom::Rect::from_pos_and_radius(pos,self.mouse_prop.radius.radius());
    }
}

