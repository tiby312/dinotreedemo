use axgeom;
use axgeom::Rect;
use botlib::bot::BotProp;


use dinotree::*;
use dinotree::Rects;
use dinotree::support::Numf32;
use dinotree::support;

use botlib::bot::BBot;
use botlib::bot;
use axgeom::AxisTrait;

use botlib::mouse::Mouse;


pub struct WrapAround{}

impl WrapAround{

	pub fn handle_mouse(prop:BotProp,tree:&mut DinoTree2<BBot>,rect:&Rect<f32>,mouse:&Mouse){

		let mut mm=*mouse;
		
		let mut flipp=false;

		let mut ff=mm.midpoint;
		
		for axis in axgeom::AxisIter::new(){
		
			if mm.get_rect().get_range(axis).left()<rect.get_range(axis).left(){
				*ff.get_axis_mut(axis)+=rect.get_range(axis).len();
				flipp=true;
			}else if mm.get_rect().get_range(axis).right()>rect.get_range(axis).right(){
				*ff.get_axis_mut(axis)-=rect.get_range(axis).len();
				flipp=true;
			}
		}

		if !flipp{
			return;
		}

		mm.move_to(&ff);    

		tree.rects().for_all_in_rect(&bot::convert_to_nan(*mm.get_rect()),
			&mut |mut a:ColSingle<BBot>|{bot::collide_mouse(&mut a,&prop,mouse);});
	
	}
	pub fn handle(tree:&mut DinoTree2<BBot>,rect:&Rect<f32>,max_prop:BotProp){
		

	    
        //                   world
        //       |                          |
        //      (|  .   )                   |
        //                                 (|  .   )
        //          ^                          ^
        //         bot                    projected_bot
        //For bots intersecting the border, this will project their position to the other side
        
        let width=max_prop.radius.radius2()+max_prop.radius.radius();
        let padding=max_prop.radius.radius2();

        
        //Regardless of the starting axis, we want to handle x and y.
        use axgeom::XAXIS_S;
		use axgeom::YAXIS_S;

		{
        	let mut rects=tree.rects();
			Self::handle2::<XAXIS_S>(&max_prop,&mut rects,width,padding,rect);
        }
        {
        	let mut rects=tree.rects();
        	Self::handle2::<YAXIS_S>(&max_prop,&mut rects,width,padding,rect);
        }
	
	}


	

	fn handle2<'a,A:AxisTrait>(
		prop:&BotProp,
		rects:&mut Rects<DinoTree2<BBot>>,
		width:f32,
		padding:f32,rect:&Rect<f32>){

		let top_d_axis=A::get();
		let left_r_axis=A::Next::get();
		let top_down_range=rect.get_range(top_d_axis);

		let top_down_length=rect.get_range(top_d_axis).end-rect.get_range(top_d_axis).start;

		let left_right_range=*rect.get_range(left_r_axis).clone().grow(width);

		//get top rect
		let rect1={
			let mut rr=rect.clone();
			rr.get_range_mut(top_d_axis).start=top_down_range.start-padding;   
			rr.get_range_mut(top_d_axis).end=top_down_range.start+width;
			*rr.get_range_mut(left_r_axis)=left_right_range;
			rr
		};

		//get bottom rect
		let rect2={
			let mut rr=rect.clone();
			rr.get_range_mut(top_d_axis).start=top_down_range.end-width;
			rr.get_range_mut(top_d_axis).end=top_down_range.end+padding;
			*rr.get_range_mut(left_r_axis)=left_right_range;
			rr
		};

		let rect1=bot::convert_to_nan(rect1);
		let rect2=bot::convert_to_nan(rect2);

		let bo=|cc:ColPair<BBot>|{
			    let mut copy_botstuff=cc.a.1.clone();
			    let mut pos=cc.a.1.pos.clone();
			    *pos.get_axis_mut(top_d_axis)+=top_down_length;
			    copy_botstuff.pos=pos;
			     
			    let cc_copy=ColPair{a:(cc.a.0,&mut copy_botstuff),b:(cc.b.0,cc.b.1)};
			    
			    bot::collide(&prop,cc_copy);
		};
		support::collide_two_rect_parallel::<A::Next,_,_,_,_>(rects,&rect1,&rect2,bo);
		
	}
	
}
