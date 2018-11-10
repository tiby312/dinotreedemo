#![feature(try_from)]
extern crate axgeom;
extern crate compt;
extern crate ordered_float;
extern crate dinotree_alg;
extern crate dinotree;
extern crate dinotree_measure;
extern crate dists;
extern crate num;


mod vec;
mod bot;
pub use vec::Vec2;
pub use bot::Bot;

mod inner_prelude{
    pub use bot::*;
    pub use vec::Vec2;
    pub use ordered_float::*;
    pub use axgeom::*;
    pub use dinotree_measure::*;
    pub(crate) use dists;
    pub use dinotree::*;
    pub(crate) use convert_to_nan;
    pub(crate) use convert_from_nan;
}

use inner_prelude::*;


fn convert_to_nan(r:Rect<f32>)->Rect<NotNaN<f32>>{

    let a=r.get_range(axgeom::XAXISS);
    let b=r.get_range(axgeom::YAXISS);
    
    let rect=Rect::new(
        NotNaN::new(a.left).unwrap(),
        NotNaN::new(a.right).unwrap(),
        NotNaN::new(b.left).unwrap(),
        NotNaN::new(b.right).unwrap()
        );
    rect
}
fn convert_from_nan(r:Rect<NotNaN<f32>>)->Rect<f32>{
    unsafe{std::mem::transmute(r)}
}



type Tree=dinotree::DinoTree<axgeom::YAXISS,(),dinotree::BBox<NotNaN<f32>,Bot>>;



pub struct BotSystem {
    mouse_prop:MouseProp,
    bots: Vec<Bot>,
    bot_prop:BotProp,
    session:Session,
}

impl Drop for BotSystem{
    fn drop(&mut self){
        self.session.finish();
    }
}


impl BotSystem{

    pub fn new(num_bots:usize) -> (BotSystem,Rect<f32>) {
        
        let bot_prop=BotProp{
            radius:Dist::new(12.0),
            collision_drag:0.01,
            collision_push:1.0,
            minimum_dis_sqr:0.0001,
            viscousity_coeff:0.06
        };

        let (bots,container_rect) = bot::create_bots(num_bots,&bot_prop).unwrap();

        let session=Session::new();


        let mouse_prop=MouseProp{
            radius:Dist::new(100.0),
            force:2.0
        };
        let b=BotSystem {
            mouse_prop:mouse_prop,
            bots,
            bot_prop,
            session
        };
        (b,container_rect)
    }

    pub fn get_bots(&self)->(f32,&[Bot]){
        //TODO return Dist prop instead?
        (self.bot_prop.radius.dis(),&self.bots)
    }

    pub fn step(&mut self, poses: &[Vec2],border:&Rect<f32>) {
        
        let border=convert_to_nan(*border);

        {                
            let bot_prop=&self.bot_prop;

            let mut tree = DinoTreeMeasure::new(axgeom::YAXISS,&self.bots,|bot|{
                bot.create_bbox(bot_prop.radius.dis())
            });

            tree.query_mut(&mut self.session,|a,b|{
                bot_prop.collide(&mut a.inner,&mut b.inner);
            });

            for k in poses{
                let mouse=Mouse::new(k,&self.mouse_prop);
                 
                let _ = dinotree_alg::multirect::multi_rect_mut(tree.get_inner()).for_all_in_rect_mut(convert_to_nan(*mouse.get_rect()),&mut |a:&mut BBox<NotNaN<f32>,Bot>|{
                    bot_prop.collide_mouse(&mut a.inner,&mouse);
                });
            }

            border_handle(tree.get_inner(),&border);
            
            tree.get_inner().apply(&mut self.bots,|b,t|*t=b.inner);
        }

        //update bots
        for bot in self.bots.iter_mut() {
            bot.vel+=bot.acc;    
            bot.pos+=bot.vel;
            bot.acc.0=[0.0;2];
        }        
    }

}



fn border_handle(tree:&mut Tree,rect:&axgeom::Rect<NotNaN<f32>>){

    let a=rect.get_range(axgeom::XAXISS);
    let b=rect.get_range(axgeom::YAXISS);
    let rect2=axgeom::Rect::new(a.left.into_inner(),a.right.into_inner(),b.left.into_inner(),b.right.into_inner());
    
    let xx=rect2.get_range(axgeom::XAXISS);
    let yy=rect2.get_range(axgeom::YAXISS);

    dinotree_alg::rect::for_all_not_in_rect_mut(tree,rect,|a|{
        //TODO improve this
        let pos=&mut a.inner.pos.0;
        let vel=&mut a.inner.vel.0;
        if pos[0]<xx.left{
            pos[0]=xx.left;
            vel[0]=-vel[0];
        }
        if pos[0]>xx.right{
            pos[0]=xx.right;
            vel[0]=-vel[0];
        }
        if pos[1]<yy.left{
            pos[1]=yy.left;
            vel[1]=-vel[1];
        }
        if pos[1]>yy.right{
            pos[1]=yy.right;
            vel[1]=-vel[1];
        }
    })

    /*
        macro_rules! bla{
            ($axis:ident)=>{
                let j=$axis;
                let a=rect.get_range(j).left.into_inner();
                let b=rect.get_range(j).right.into_inner();

                let mut new_pos=bot.pos.clone();

                if *bot.pos.get_axis(j) < a {
                    *new_pos.get_axis_mut(j) = b;
                }
                if *bot.pos.get_axis(j) > b {
                    *new_pos.get_axis_mut(j) = a;
                }
                bot.pos=new_pos;
            }
        }
        use axgeom::XAXISS;
        use axgeom::YAXISS;
        bla!(XAXISS);
        bla!(YAXISS);
        */

}


