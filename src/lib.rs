pub use dinotree;
pub use duckduckgeo;
use crate::bot::*;
use dinotree::axgeom::ordered_float::*;
use dinotree::axgeom::*;
use dinotree::axgeom;

use dinotree::prelude::*;
use duckduckgeo::*;
use dinotree_alg::rect::*;


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
            //radius:Dist::new(20.0),
            collision_drag:0.003,
            collision_push:1.3,
            minimum_dis_sqr:0.0001,
            viscousity_coeff:0.03
        };

        let (bots,mut container_rect) = create_bots(num_bots,&bot_prop).unwrap();
        container_rect.grow(10.0);
        //let session=Session::new();
        //let session=DinoTreeCache::new(axgeom::YAXISS);

        let mouse_prop=MouseProp{
            radius:Dist::new(150.0),
            force:20.0//1.0
        };
        let b=BotSystem {
            mouse_prop,
            bots,
            bot_prop,
        };
        (b,container_rect,bot_prop.radius.dis()*0.7)
    }

    
    pub fn get_bots(&self)->&[Bot]{
        &self.bots
    }

    pub fn get_bots_mut(&mut self)->&mut [Bot]{
        &mut self.bots
    }


    pub fn step(&mut self, poses: &[Vec2<f32>],border:&Rect<f32>) {
        
        let border=border.inner_try_into().unwrap();

        {                
            let bot_prop=&self.bot_prop;
            

            let mut bots=create_bbox_mut(&mut self.bots,|bot|{
                bot.create_bbox(bot_prop).inner_try_into().unwrap()
            });

            let mut tree=DinoTreeBuilder::new(axgeom::YAXISS,&mut bots).build_par();

            
            dinotree_alg::colfind::QueryBuilder::new(&mut tree).query_par(|mut a,mut b|{
                bot_prop.collide(a.inner_mut(),b.inner_mut());
            });
            

            for k in poses{
                let mouse=Mouse::new(*k,&self.mouse_prop);
                let mouserect=mouse.get_rect().inner_try_into().unwrap();
                 
                for_all_in_rect_mut(&mut tree,&mouserect,|mut a|{
                    bot_prop.collide_mouse(a.inner_mut(),&mouse);
                });
            }
            
            for_all_not_in_rect_mut(&mut tree,&border,|mut a|{
                duckduckgeo::collide_with_border(a.inner_mut(),border.as_ref(),0.5);
            });

        }



        //update bots
        for bot in self.bots.iter_mut() {
            bot.vel+=bot.acc;    
            bot.pos+=bot.vel;
            bot.acc=vec2(0.0,0.0);
        }        
    }

}


#[derive(Copy,Clone,Debug)]
pub struct NoBots;
pub fn create_bots(num_bot:usize,bot_prop: &BotProp)->Result<(Vec<Bot>,axgeom::Rect<f32>),NoBots>{
    
    //let s=dists::spiral::Spiral::new([0.0,0.0],12.0,1.0);
    

    let s=dists::grid::Grid::new(axgeom::Rect::new(-2000.,2000.,-1300.,1300.),num_bot);
    //let s=dists::grid::Grid::new(axgeom::Rect::new(-30000.,30000.,-20000.,20000.),num_bot);

    let bots:Vec<Bot>=s.take(num_bot).map(|pos|Bot::new(vec2(pos.x as f32,pos.y as f32))).collect();

    let rect=bots.iter().fold(None,|rect:Option<Rect<NotNan<f32>>>,bot|{
        match rect{
            Some(mut rect)=>{
                rect.grow_to_fit(&bot.create_bbox(bot_prop).inner_try_into().unwrap());
                Some(rect)
            },
            None=>{
                Some(bot.create_bbox(bot_prop).inner_try_into().unwrap())
            }
        }
    });



    match rect{
        Some(x)=>{
            let xx=x.inner_into();
            Ok((bots,xx))
        },
        None=>{
            Err(NoBots)
        }
    }
}

