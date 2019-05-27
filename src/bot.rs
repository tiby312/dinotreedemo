use crate::inner_prelude::*;


pub use duckduckgeo::bot::*;
pub use duckduckgeo::*;
pub use duckduckgeo::vec2f32::*;
/*
pub fn handle_rigid_body(bodies:&mut [Bot],radius:f32,max_move_every_iteration:f32,max_num_iteration:usize){
    
    for body in bodies.iter_mut(){
        body.acc.set_zero();
    }

    let ball_size=radius;
    let push_rate=max_move_every_iteration;//push_unit / (num_iteration as f64);

    for i in 0..max_num_iteration{        
        let mut tree=DinoTreeBuilder::new(axgeom::YAXISS,bodies,|a|a.create_bbox(ball_size+push_rate)).build_par();

        //let mut counter=0;
        dinotree_alg::colfind::QueryBuilder::new(tree.as_ref_mut()).query_par(|a,b|{
            a.inner.push_away(&mut b.inner,ball_size,push_rate);
            //counter+=1;
        });    
        /*
        if counter==0{
            println!("exiting early at iteration={:?}",i);
            break;
        }
        */

        tree.apply(bodies,|a,b|*b=a.inner);

        for body in bodies.iter_mut(){
            if body.acc.dis()>0.0000001{
                body.acc.truncate(push_rate);
                body.pos+=body.acc;
                body.acc.set_zero();
            }
        }
    }
}
*/





#[derive(Copy,Clone,Debug)]
pub struct NoBots;
pub fn create_bots(num_bot:usize,bot_prop: &BotProp)->Result<(Vec<Bot>,axgeom::Rect<f32>),NoBots>{
    
    let s=dists::spiral::Spiral::new([0.0,0.0],12.0,1.0);

    let bots:Vec<Bot>=s.take(num_bot).map(|pos|Bot::new(Vec2::new(pos[0] as f32,pos[1] as f32))).collect();

    let rect=bots.iter().fold(None,|rect:Option<Rect<NotNaN<f32>>>,bot|{
        match rect{
            Some(mut rect)=>{
                rect.grow_to_fit(&convert_to_nan(bot.create_bbox(bot_prop)));
                Some(rect)
            },
            None=>{
                Some(convert_to_nan(bot.create_bbox(bot_prop)))
            }
        }
    });



    match rect{
        Some(x)=>{
            let xx=convert_from_nan(x);
            Ok((bots,xx))
        },
        None=>{
            Err(NoBots)
        }
    }
}

