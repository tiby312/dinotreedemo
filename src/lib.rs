//#![feature(try_from)]
extern crate axgeom;
extern crate ordered_float;
extern crate dinotree_alg;
extern crate dinotree;
extern crate dists;
extern crate num;


mod bot;
//pub use crate::vec::Vec2;
pub use crate::bot::Bot;

mod inner_prelude{
    pub use crate::bot::*;
    //pub use crate::vec::Vec2;
    pub use ordered_float::*;
    pub use axgeom::*;
    pub(crate) use dists;
    pub use dinotree::*;
    pub use dinotree::copy::*;
    //pub(crate) use crate::convert_to_nan;
    //pub(crate) use crate::convert_from_nan;
}

use crate::inner_prelude::*;




type Tree=DinoTree<axgeom::YAXISS,BBox<NotNaN<f32>,Bot>>;



//input:
//a minimum rectangle that must be visible in the game world
//the window dimensions.
//output:
//the game world dimensions
pub fn compute_border(rect:Rect<f32>,window:[f32;2])->Rect<f32>{
    
    println!("game word minimum={:?}",rect);
    println!("window={:?}",window);
    let target_aspect_ratio=window[0]/window[1];


    let ((x1,x2),(y1,y2))=rect.get();
    let w=x2-x1;
    let h=y2-y1;

    let current_aspect_ratio=w/h;


    //let mut xx=0.0;
    //let mut yy=0.0;
    let [xx,yy]=if target_aspect_ratio<current_aspect_ratio{
        //target is thinner
        [0.0,-h+(window[1]*w)/window[0]]

    }else{
        //target is wider
        [window[0]*h/window[1]-w,0.0]
    };

    let xx_half=xx/2.0;
    let yy_half=yy/2.0;

    let xx1=x1-xx_half;
    let xx2=x2+xx_half;

    let yy1=y1-yy_half;
    let yy2=y2+yy_half;

    let r=Rect::new(xx1,xx2,yy1,yy2);
    println!("game world target={:?}",r);
    r
}

pub struct BotSystem {
    mouse_prop:MouseProp,
    bots: Vec<Bot>,
    bot_prop:BotProp,
    //session:Session,
    //session:DinoTreeCache<axgeom::YAXISS,BBox<NotNaN<f32>,Bot>>
}

impl Drop for BotSystem{
    fn drop(&mut self){
        //self.session.finish();
    }
}


impl BotSystem{

    pub fn new(num_bots:usize) -> (BotSystem,Rect<f32>,f32) {
        
        let bot_prop=BotProp{
            radius:Dist::new(12.0),
            collision_drag:0.003,
            collision_push:0.5,
            //collision_push:0.1,
            minimum_dis_sqr:0.0001,
            viscousity_coeff:0.03
        };

        let (bots,mut container_rect) = bot::create_bots(num_bots,&bot_prop).unwrap();
        container_rect.grow(200.0);
        //let session=Session::new();
        //let session=DinoTreeCache::new(axgeom::YAXISS);

        let mouse_prop=MouseProp{
            radius:Dist::new(200.0),
            force:1.0
        };
        let b=BotSystem {
            mouse_prop,
            bots,
            bot_prop,
        };
        (b,container_rect,bot_prop.radius.dis()*0.7)
    }

    pub fn get_bots(&self)->&[Bot]{
        //TODO return Dist prop instead?
        &self.bots
    }

    pub fn step(&mut self, poses: &[Vec2],border:&Rect<f32>) {
        
        let border=convert_to_nan(*border);

        {                
            let bot_prop=&self.bot_prop;
            


            //let sr=bot_prop.radius.dis()*0.2;
            //bot::handle_rigid_body(&mut self.bots,sr,sr*0.2,10);

            let mut tree=DinoTreeBuilder::new(axgeom::YAXISS,&self.bots,|bot|{
                convert_to_nan(bot.create_bbox(bot_prop))
            }).build_par();

            //TODO remove
            assert!(assert_invariants(&tree));
            


            dinotree_alg::colfind::QueryBuilder::new(&mut tree).query_par(|a,b|{
                bot_prop.collide(&mut a.inner,&mut b.inner);
            });
            
            for k in poses{
                let mouse=Mouse::new(*k,&self.mouse_prop);
                 
                let _ = dinotree_alg::multirect::multi_rect_mut(&mut tree).for_all_in_rect_mut(convert_to_nan(*mouse.get_rect()),&mut |a:&mut BBox<NotNaN<f32>,Bot>|{
                    bot_prop.collide_mouse(&mut a.inner,&mouse);
                });
            }

            border_handle(&mut tree,&border);
            

            tree.apply(&mut self.bots,|b,t|*t=b.inner);
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
        let drag=0.5;
        let pos=&mut a.inner.pos.0;
        let vel=&mut a.inner.vel.0;
        if pos[0]<xx.left{
            pos[0]=xx.left;
            vel[0]= -vel[0];
            vel[0]*=drag;
        }
        if pos[0]>xx.right{
            pos[0]=xx.right;
            vel[0]= -vel[0];
            vel[0]*=drag;
        }
        if pos[1]<yy.left{
            pos[1]=yy.left;
            vel[1]= -vel[1];
            vel[1]*=drag;
        }
        if pos[1]>yy.right{
            pos[1]=yy.right;
            vel[1]= -vel[1];
            vel[1]*=drag;
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


