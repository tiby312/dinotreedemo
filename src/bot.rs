use crate::inner_prelude::*;


pub use duckduckgeo::bot::*;
pub use duckduckgeo::*;
pub use duckduckgeo::vec2f32::*;





#[derive(Copy,Clone,Debug)]
pub struct NoBots;
pub fn create_bots(num_bot:usize,bot_prop: &BotProp)->Result<(Vec<Bot>,axgeom::Rect<f32>),NoBots>{
    
    let s=dists::spiral::Spiral::new([0.0,0.0],12.0,1.0);

    let bots:Vec<Bot>=s.take(num_bot).map(|pos|Bot::new(Vec2::new(pos[0] as f32,pos[1] as f32))).collect();

    let rect=bots.iter().fold(None,|rect:Option<Rect<NotNaN<f32>>>,bot|{
        match rect{
            Some(mut rect)=>{
                rect.grow_to_fit(&bot.create_bbox(bot_prop).into_notnan().unwrap());
                Some(rect)
            },
            None=>{
                Some(bot.create_bbox(bot_prop).into_notnan().unwrap())
            }
        }
    });



    match rect{
        Some(x)=>{
            let xx=x.into_inner();
            Ok((bots,xx))
        },
        None=>{
            Err(NoBots)
        }
    }
}

