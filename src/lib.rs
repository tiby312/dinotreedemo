use crate::bot::*;
use ordered_float::*;
use axgeom::*;
use dinotree::*;
use dinotree::copy::*;
use duckduckgeo::*;

use cgmath::prelude::*;
use cgmath::Vector2;
use cgmath::vec2;


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
    bot_prop:BotProp
}


impl BotSystem{

    pub fn new(num_bots:usize) -> (BotSystem,Rect<f32>,f32) {
        
        let bot_prop=BotProp{
            radius:Dist::new(12.0),
            collision_drag:0.003,
            collision_push:0.5,
            minimum_dis_sqr:0.0001,
            viscousity_coeff:0.03
        };

        let (bots,mut container_rect) = create_bots(num_bots,&bot_prop).unwrap();
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

    pub fn step(&mut self, poses: &[Vector2<f32>],border:&Rect<f32>) {
        
        let border=border.cast().unwrap();

        {                
            let bot_prop=&self.bot_prop;
            

            let mut tree=DinoTreeBuilder::new(axgeom::YAXISS,&self.bots,|bot|{
                bot.create_bbox(bot_prop).cast().unwrap()
            }).build_par();

            //assert!(assert_invariants(&tree));
            
            dinotree_alg::colfind::QueryBuilder::new(&mut tree).query_par(|a,b|{
                bot_prop.collide(&mut a.inner,&mut b.inner);
            });
        
            for k in poses{
                let mouse=Mouse::new(*k,&self.mouse_prop);
                let mouserect=mouse.get_rect().cast().unwrap();
                 
                let _ = dinotree_alg::multirect::multi_rect_mut(&mut tree).for_all_in_rect_mut(mouserect,&mut |a:&mut BBox<NotNan<f32>,Bot>|{
                    bot_prop.collide_mouse(&mut a.inner,&mouse);
                });
            }
            
            let rect2=border.cast().unwrap();
            dinotree_alg::rect::for_all_not_in_rect_mut(&mut tree,&border,|a|{
                duckduckgeo::collide_with_border(&mut a.inner,&rect2,0.5);
            });
        
            tree.apply(&mut self.bots,|b,t|*t=b.inner);
        }

        //update bots
        for bot in self.bots.iter_mut() {
            bot.vel+=bot.acc;    
            bot.pos+=bot.vel;
            bot.acc=Vector2::zero();
        }        
    }

}


#[derive(Copy,Clone,Debug)]
pub struct NoBots;
pub fn create_bots(num_bot:usize,bot_prop: &BotProp)->Result<(Vec<Bot>,axgeom::Rect<f32>),NoBots>{
    
    let s=dists::spiral::Spiral::new([0.0,0.0],12.0,1.0);

    let bots:Vec<Bot>=s.take(num_bot).map(|pos|Bot::new(vec2(pos[0] as f32,pos[1] as f32))).collect();

    let rect=bots.iter().fold(None,|rect:Option<Rect<NotNan<f32>>>,bot|{
        match rect{
            Some(mut rect)=>{
                rect.grow_to_fit(&bot.create_bbox(bot_prop).cast().unwrap());
                Some(rect)
            },
            None=>{
                Some(bot.create_bbox(bot_prop).cast().unwrap())
            }
        }
    });



    match rect{
        Some(x)=>{
            let xx=x.cast().unwrap();
            Ok((bots,xx))
        },
        None=>{
            Err(NoBots)
        }
    }
}

