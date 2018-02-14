
use axgeom;
//use testy;
use Vert;
use sys::BotSysTrait;
use std::marker::PhantomData;
use botlib::bot::BotProp;
//use sys::BotSysGraphics;
use botlib::graphics::BotLibGraphics;
use botlib::mouse::MouseProp;
use dinotree::TreeCache;
use dinotree::support::Numf32;
use axgeom::XAXIS_S;
use axgeom::YAXIS_S;
use sys::TreeNoDraw;
use botlib::bot;
use botlib::bot::BBot;
use super::*;
use ascii_num;

//TODO put this somewhere else
struct IteratorCounter<I:Iterator> { iter: I, count: usize }

impl<I:Iterator> IteratorCounter<I> {
    pub fn new(iter:I)->IteratorCounter<I>{
        IteratorCounter{iter,count:0}
    }
    pub fn steps_taken(&self) -> usize {
        self.count  
    }
}

impl<I: Iterator> Iterator for IteratorCounter<I> {
    type Item = <I as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.iter.next() {
                self.count += 1; Some(n) 
        } else {
            None 
        } 
    } 
}




struct GameState(Box<BotSysTrait>);

impl MenuState for GameState{
    fn step(&mut self, poses: &[axgeom::Vec2])->(Option<Box<MenuState>>,(Option<[f32;3]>,bool)){
        self.0.step(poses);
        (None,(None,true))
    }    
    fn get_verticies(&self,a:&mut [Vert]){
        self.0.get_verticies(a)
    }
    fn num_verticies(&self)->usize{
        self.0.get_num_verticies()
    }


}

trait MenuState{
    fn step(&mut self, poses: &[axgeom::Vec2])->(Option<Box<MenuState>>,(Option<[f32;3]>,bool));
    fn get_verticies(&self,v:&mut [Vert]); 
    fn num_verticies(&self)->usize;
}



use self::menu::MenuSystem;


pub static COLS:&'static [[f32;3]]=
    &[
        [0.9,0.9,0.9],
        [1.0,0.2,0.2],
        [0.0,1.0,0.0],
        [0.6,0.5,1.0],
        [1.0,1.0,0.0],
        [1.0,0.0,1.0],
        [0.0,1.0,1.0],
    ];


mod menu{
    use super::*;

    pub struct MenuSystem{
        //graphics:BotSysGraphics,
        bot_graphics:BotLibGraphics,
        mouse_prop:MouseProp,
        bot_prop:BotProp,
        bots: Vec<BBot>,
        border: axgeom::Rect<f32>,
        treecache:TreeCache<XAXIS_S,Numf32>, 
        dim:(usize,usize),
        buttons:[Button;3],
        color_button:Button,
        color_clicker:Clicker,
        col_counter:usize,
        numberthing:NumberThing,
        debug_button:OnOffButton,
        debug_clicker:Clicker,
        draw_debug:bool
    }


    use self::primitives::Button;
    use self::primitives::OnOffButton;
    use self::primitives::NumberThing;

    struct Clicker{
        there_was_finger:bool,
        there_is_finger:bool
    }
    impl Clicker{
        fn new()->Clicker{
            Clicker{there_was_finger:false,there_is_finger:false}
        }
        fn update(&mut self,dim:&axgeom::Rect<f32>,poses:&[axgeom::Vec2])->bool{

            for i in poses.iter(){
                if dim.contains_vec(i){
                    self.there_is_finger=true;
                } 
            }
            let ret=if !self.there_was_finger & self.there_is_finger{
                // If the button is pushed and wasn't before change color
                //graphy.set_bot_color(COLS[self.col_counter]);
                //self.col_counter=(self.col_counter+1) % COLS.len();
                true
            }else{
                false
            };
            // Otherwise set stored value to current
            self.there_was_finger = self.there_is_finger;
            // Reset current variable to false
            self.there_is_finger = false;

            ret
        }
    }


    mod primitives{
        use super::*;

        pub struct OnOffButton{
            on_but:Button,
            off_but:Button,
            dim:axgeom::Rect<f32>,
            on:bool
        }

        impl OnOffButton{
            pub fn new(topleft:axgeom::Vec2,poses_off:Vec<(usize,usize)>,poses_on:Vec<(usize,usize)>,spacing:f32)->OnOffButton{
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

            pub fn draw<'a,I:Iterator<Item=&'a mut BBot>>(&self,bb:&mut I){
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
            pub fn new(topleft:axgeom::Vec2,poses:Vec<(usize,usize)>,spacing:f32)->Button{
                let m=poses.iter().fold((0,0), |acc, &x| {(acc.0.max(x.0),acc.1.max(x.1))});
                
                let dimx=m.0 as f32*spacing;
                let dimy=m.1 as f32*spacing;
                let k=topleft.get();
                let dim=axgeom::Rect::new(*(k.0),*(k.0)+dimx,*(k.1),*(k.1)+dimy);
                
                let mut padding=dim;
                padding.grow(spacing*2.0);
                Button{poses:poses,dim,padding,spacing}
            }
            pub fn draw<'a,I:Iterator<Item=&'a mut BBot>>(&self,bb:&mut I){
                for pos in self.poses.iter(){
                    use dinotree::SweepTrait;
                   
                    //let i=i as f32;
                    let k=bb.next().unwrap();
                    
                    let k=k.get_mut().1;
                    let x=pos.0 as f32;
                    let y=pos.1 as f32;
                    k.vel=axgeom::Vec2::new(0.0,0.0);
                    k.acc=axgeom::Vec2::new(0.0,0.0);

                    let dx=self.dim.get_range2::<XAXIS_S>();
                    let yx=self.dim.get_range2::<YAXIS_S>();

                    k.pos=axgeom::Vec2::new(dx.start+x*self.spacing,yx.start+y*self.spacing);
                }
            }
        }


        pub struct NumberThing{
            digits:Vec<Vec<(usize,usize)>>,
            pixel_spacing:f32,
            digit_spacing:f32,
            number:usize,
            top_right:axgeom::Vec2
        }

        impl NumberThing{
            pub fn new(digit_spacing:f32,pixel_spacing:f32,number:usize,top_right:axgeom::Vec2)->NumberThing{
                NumberThing{digits:ascii_num::get_coords(number),pixel_spacing,digit_spacing,number,top_right}
            }
            pub fn update_number(&mut self,number:usize){
                self.number=number;
                self.digits=ascii_num::get_coords(self.number);
            }
            pub fn get_number(&self)->usize{
                self.number
            }
            pub fn draw<'a,I:Iterator<Item=&'a mut BBot>>(&self,bb:&mut I){
                use dinotree::SweepTrait;
                use ascii_num;
                for (i,digit) in self.digits.iter().rev().enumerate(){
                    let i=i as f32;
                    for pos in digit{
                        let k=bb.next().unwrap();
                        
                        let k=k.get_mut().1;

                        let x=pos.0 as f32;
                        let y=pos.1 as f32;
                        k.vel=axgeom::Vec2::new(0.0,0.0);
                        k.acc=axgeom::Vec2::new(0.0,0.0);

                        let tr=self.top_right.get();
                        let ds=self.digit_spacing;
                        let ps=self.pixel_spacing;
                        k.pos=axgeom::Vec2::new(tr.0-i*ds+x*ps,tr.1+y*ps);
                    }
                }

            }
        }
    }


    impl MenuSystem{
        pub fn new(startx:usize,starty:usize)->(MenuSystem,[f32;3]){
            
            let height=3;
            let num_bots=5000;

            let border=axgeom::Rect::new(0.0,startx as f32,0.0,starty as f32);
            
            //used as the building block for all positions
            let unit=bot::get_unit(startx,starty);
            
            let br=unit*1.0;
            let mr=unit*10.0;

            let (bot_prop,mouse_prop)=bot::create_from_radius(br,mr);

            let bots=bot::create_bots(num_bots,&border,&bot_prop);

            //let graphics=BotSysGraphics::new::<TreeNoDraw>(&bots,height);

            
            let buttons={
                let mut v=axgeom::Vec2::new(unit*5.0,starty as f32-unit*30.0);
                
                let b1=Button::new(v,ascii_num::get_misc(0),unit*2.0);
                *(v.get_mut().0)+=unit*20.0;
                let b2=Button::new(v,ascii_num::get_misc(1),unit*2.0);
                *(v.get_mut().0)+=unit*20.0;
                let b3=Button::new(v,ascii_num::get_misc(2),unit*2.0);
                *(v.get_mut().0)+=unit*20.0;
                [b1,b2,b3]
            };

            let kk=axgeom::Vec2::new(unit*5.0,starty as f32-unit*90.0);
            let color_button=Button::new(kk,ascii_num::get_misc(3),unit*2.0);


            let kk=axgeom::Vec2::new(unit*5.0,starty as f32-unit*70.0);    
            let debug_button=OnOffButton::new(kk,
                    ascii_num::get_misc(4),
                    ascii_num::get_misc(5),
                    unit*2.0);

            let numberthing={
                let x=startx as f32-unit*20.0;
                let y=starty as f32-unit*50.0;
                NumberThing::new(unit*15.0,unit*2.0,5000,axgeom::Vec2::new(x,y))
            };

            let col=COLS[0];

            (MenuSystem{
                //graphics,
                bot_graphics:BotLibGraphics::new(&bot_prop),
                mouse_prop,
                bot_prop,
                bots,
                border,
                treecache:TreeCache::new(height),
                dim:(startx,starty),
                buttons,
                color_button,
                col_counter:0 , //TODO hack
                color_clicker:Clicker::new(),
                numberthing,
                debug_button,
                debug_clicker:Clicker::new(),
                draw_debug:false
            },col)
        }
    }
    impl MenuState for MenuSystem{
        fn step(&mut self, poses: &[axgeom::Vec2])->(Option<Box<MenuState>>,(Option<[f32;3]>,bool)){
            let bot_prop=&self.bot_prop;
            let bots=&mut self.bots;
            let border=&self.border;
            let mouse_prop=&self.mouse_prop;
            //let graphics=&mut self.graphics;
            let bot_graphics=&mut self.bot_graphics;

            for i in poses.iter(){
                let curr=self.numberthing.get_number();

                //up arrow
                if self.buttons[0].get_dim().contains_vec(i){
                    self.numberthing.update_number(curr+20);
                }
                if self.buttons[1].get_dim().contains_vec(i){
                    self.numberthing.update_number((curr as isize-20).max(1000) as usize); 
                }
                if self.buttons[2].get_dim().contains_vec(i){

                    let (startx,starty)=self.dim;

                    let k=sys::new(curr,startx,starty,self.draw_debug);
                    return (Some(Box::new(GameState(k))),(None,false))
                }

               
            }


            if self.color_clicker.update(self.color_button.get_dim(),poses){
                self.col_counter=(self.col_counter+1) % COLS.len();
            }

            if self.debug_clicker.update(self.debug_button.get_dim(),poses){
                println!("pushed debug button!");
                self.draw_debug=!self.draw_debug;
                self.debug_button.set(self.draw_debug);
            }
 
            let num_bots={
                use dinotree::SweepTrait;
                let mut bb=IteratorCounter::new(bots.iter_mut());
             
                self.numberthing.draw(&mut bb);

                for i in self.buttons.iter(){
                    i.draw(&mut bb);
                }

                self.color_button.draw(&mut bb);
                self.debug_button.draw(&mut bb);

                let steps=bb.steps_taken();
            
                for b in bb{
                    b.val.pos=axgeom::Vec2::new(-100.0,-100.0);
                    b.update_box(&0.0);
                }

                steps
            };

            
            (None,(Some(COLS[self.col_counter]),false))
        }    

        fn num_verticies(&self)->usize{
            BotLibGraphics::get_num_verticies(self.bots.len())
        }
    
        fn get_verticies(&self,verts:&mut [Vert]){
            //let verts=graphics.drawer.get_range_mut(&graphics.bot_handle);
            
            self.bot_graphics.update(&self.bot_prop,&self.bots,verts);
            //self.graphics.drawer.get_all_ranges() 
        }
    }
}



pub struct MenuGame{
	state:Box<MenuState>
}


impl MenuGame{

    ///Returns desired color
	pub fn new(startx:usize,starty:usize)->(MenuGame,[f32;3]){
        let (k,col)=MenuSystem::new(startx,starty);
        
        (MenuGame{state:Box::new(k)},col)
    }


    pub fn get_num_verticies(&self)->usize{
        self.state.num_verticies()
    }

    ///The number of verticies may change!
	pub fn get_verticies(&self,v:&mut [Vert]){
        self.state.get_verticies(v);
	}

    ///Pass it the touch posisitons based.
    ///Returns desired color if different.
	pub fn step(&mut self,poses:&[axgeom::Vec2])->(Option<[f32;3]>,bool){
		let (j,cols)=self.state.step(poses);

        match j{
            Some(x)=>{
                self.state=x; 
            },
            None=>{

            }
        }
        cols
	}
}




