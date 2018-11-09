
use super::*;


use ordered_float::NotNaN;
use botlib::bot::BotProp;

pub fn handle(tree:&mut sys::Tree,rect:&axgeom::Rect<NotNaN<f32>>,max_prop:BotProp){
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


    	//num::clamp(pos.0[0],xx.left,xx.right);
    	//num::clamp(pos.0[1],yy.left,yy.right);
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


