use crate::bot::*;


use axgeom::*;
use axgeom;
use dinotree_alg::prelude::*;
use duckduckgeo::*;



pub struct BotSystem {
    mouse_prop:MouseProp,
    bots: Vec<Bot>,
    bot_prop:BotProp
}


impl BotSystem{

    pub fn new(aspect_ratio:f64,num_bots:usize) -> (BotSystem,Rect<f32>,f32) {
        dbg!(aspect_ratio);

        let bot_prop=BotProp{
            radius:Dist::new(12.0),
            collision_drag:0.003,
            collision_push:0.2,
            minimum_dis_sqr:0.0001,
            viscousity_coeff:0.03
        };

        let (bots,mut container_rect) = create_bots(aspect_ratio,num_bots,&bot_prop);
        dbg!(container_rect);
        
        
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


    pub fn step(&mut self, poses: &[Vec2<f32>],border:&Vec2<f32>) {
        let border=rect(0.0,border.x,0.0,border.y);
        let border=border.inner_try_into().unwrap();

        {                
            let bot_prop=&self.bot_prop;
            

            let mut bots=bbox_helper::create_bbox_mut(&mut self.bots,|bot|{
                bot.create_bbox(bot_prop).inner_try_into().unwrap()
            });

            let mut tree=DinoTree::new_par(&mut bots);

            
            tree.find_collisions_mut_par(|mut a,mut b|{
                bot_prop.collide(a.inner_mut(),b.inner_mut());
            });
            

            for k in poses{
                let mouse=Mouse::new(*k,self.mouse_prop);
                let mouserect=mouse.get_rect().inner_try_into().unwrap();
                 
                tree.for_all_in_rect_mut(&mouserect,|mut a|{
                    bot_prop.collide_mouse(a.inner_mut(),&mouse);
                });
            }
            
            tree.for_all_not_in_rect_mut(&border,|mut a|{
                duckduckgeo::collide_with_border(a.inner_mut(),border.as_ref(),0.5);
            });

        }



        //update bots
        for bot in self.bots.iter_mut() {
            bot.update();
        }        
    }

}


#[derive(Copy,Clone,Debug)]
pub struct NoBots;
pub fn create_bots(aspect_ratio:f64,num_bot:usize,bot_prop: &BotProp)->(Vec<Bot>,axgeom::Rect<f32>){
    
    
    let mut bots=Vec::with_capacity(num_bot);
    
    let end:Vec2<f32>=dists::grid::from_top_start(vec2(0.0,0.0),aspect_ratio as f32,10.0,num_bot,|pos|bots.push(Bot::new(pos)));
    
    (bots,rect(0.0,end.x,0.0,end.y))
    /*
    let bots=bots;
    
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


    dbg!(rect);

    match rect{
        Some(x)=>{
            let xx=x.inner_into();
            Ok((bots,xx))
        },
        None=>{
            Err(NoBots)
        }
    }
    */
}

