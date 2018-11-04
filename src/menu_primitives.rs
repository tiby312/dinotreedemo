use axgeom;
use botlib::bot::Bot;
use ascii_num;
use vec::Vec2;

pub struct OnOffButton{
    on_but:Button,
    off_but:Button,
    dim:axgeom::Rect<f32>,
    on:bool
}

impl OnOffButton{
    pub fn new(topleft:Vec2,poses_off:Vec<(usize,usize)>,poses_on:Vec<(usize,usize)>,spacing:f32)->OnOffButton{
        let off_but=Button::new(topleft,poses_off,spacing);
        let on_but=Button::new(topleft,poses_on,spacing);
        
        //TODO use this. need to use genric num trait that uses Ord
        //let dim=on_but.dim.grow_to_fit(off_but.dim);
        let dim=*on_but.get_dim();

        OnOffButton{off_but,on_but,on:false,dim}
    }
    pub fn get_dim(&self)->&axgeom::Rect<f32>{
        &self.dim
    }

    pub fn set(&mut self,state:bool){
        self.on=state;
    }

    pub fn draw<'a,I:Iterator<Item=&'a mut Bot>>(&self,bb:&mut I){
        if self.on{
            self.on_but.draw(bb);
        }else{
            self.off_but.draw(bb);
        }
    }

}


pub struct Button{
    poses:Vec<(usize,usize)>,
    dim:axgeom::Rect<f32>,
    padding:axgeom::Rect<f32>,
    spacing:f32
}

impl Button{
    pub fn get_dim(&self)->&axgeom::Rect<f32>{
        &self.padding
    }
    pub fn new(topleft:Vec2,poses:Vec<(usize,usize)>,spacing:f32)->Button{
        let m=poses.iter().fold((0,0), |acc, &x| {(acc.0.max(x.0),acc.1.max(x.1))});
        
        let dimx=m.0 as f32*spacing;
        let dimy=m.1 as f32*spacing;
        let k=topleft.0;//get();
        let dim=axgeom::Rect::new(k[0],k[0]+dimx,k[1],k[1]+dimy);
        
        let mut padding=dim;
        padding.grow(spacing*2.0);
        Button{poses:poses,dim,padding,spacing}
    }
    pub fn draw<'a,I:Iterator<Item=&'a mut Bot>>(&self,bb:&mut I){
        for pos in self.poses.iter(){
            //use dinotree::SweepTrait;
           
            //let i=i as f32;
            let k=bb.next().unwrap();
            
            //let k=k.get_mut().1;
            
            let x=pos.0 as f32;
            let y=pos.1 as f32;
            
            k.vel=Vec2::new(0.0,0.0);
            k.acc=Vec2::new(0.0,0.0);

            let dx=self.dim.get_range(axgeom::XAXISS);
            let yx=self.dim.get_range(axgeom::YAXISS);

            k.pos=Vec2::new(dx.left+x*self.spacing,yx.left+y*self.spacing);
        }
    }
}


pub struct NumberThing{
    digits:ascii_num::PointDigitIterator,//Vec<Vec<(usize,usize)>>,
    pixel_spacing:f32,
    digit_spacing:f32,
    number:usize,
    top_right:Vec2
}

impl NumberThing{
    pub fn new(digit_spacing:f32,pixel_spacing:f32,number:usize,top_right:Vec2)->NumberThing{
        NumberThing{digits:ascii_num::get_coords(number),pixel_spacing,digit_spacing,number,top_right}
    }
    pub fn update_number(&mut self,number:usize){
        self.number=number;
        self.digits=ascii_num::get_coords(self.number);
    }
    pub fn get_number(&self)->usize{
        self.number
    }
    pub fn draw<'a,I:Iterator<Item=&'a mut Bot>>(&self,bb:&mut I){
        //use dinotree::SweepTrait;
        //use ascii_num;
        let length=self.digits.len();
        for (i,digit) in self.digits.clone().enumerate(){
            let i=(length-i) as f32;
            for pos in digit{
                let k=bb.next().unwrap();
                
                //let k=k.get_mut().1;

                let x=pos[0] as f32;
                let y=pos[1] as f32;
                k.vel=Vec2::new(0.0,0.0);
                k.acc=Vec2::new(0.0,0.0);

                let tr=self.top_right.0;
                let ds=self.digit_spacing;
                let ps=self.pixel_spacing;
                k.pos=Vec2::new(tr[0]-i*ds+x*ps,tr[1]+y*ps);
            }
        }

    }
}