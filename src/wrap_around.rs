
use axgeom;
use botlib::bot::BotTrait;
//use botlib::bot::Bot;
use axgeom::Rect;
use botlib::bot::BotProp;


use dinotree::*;
use dinotree::multirect::Rects;
use dinotree::support::Numf32;

use botlib::bot::BBot;
use botlib::bot;
use axgeom::AxisTrait;

use botlib::bot::BotStuff;
use botlib::bot::BotAcc;
//use dinotree::multirect::MultiRectTrait;

struct BotWrapper<'a>{
	bot_stuff:&'a BotStuff,
	acc:&'a mut BotAcc,
	pos:axgeom::Vec2
}

impl<'a> BotTrait for BotWrapper<'a>{

	fn apply_force(&mut self,vec:&axgeom::Vec2){
		self.acc.acc+=*vec;//.apply_force(vec);
	}
	fn pos(&self)->&axgeom::Vec2{
		&self.pos
	}

	fn vel(&self)->&axgeom::Vec2{
		&self.bot_stuff.vel
	}
	fn get_acc(&self)->&axgeom::Vec2{
		&self.acc.acc
	}
}
use botlib::mouse::Mouse;




pub struct WrapAround{
}

impl WrapAround{

	//pub fn handle_mouse<K:ColTrait<T=BBot>>(prop:&BotProp,tree:&mut K,rect:&Rect<f32>,mouse:&Mouse){
	pub fn handle_mouse<K:DynTreeTrait<T=BBot,Num=Numf32>>(prop:&BotProp,tree:&mut K,rect:&Rect<f32>,mouse:&Mouse){

		let mut rects:Rects<K>=Rects::new(tree);//tree.create_rects();


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

        rects.for_all_in_rect(
        			&bot::convert_to_nan(*mm.get_rect()),
                    &mut |mut cc:ColSingle<BBot>| {
                //use botlib::bot::BotMovementTrait;
                bot::collide_mouse(&mut cc,prop,&mm);
		    });
		
	}
	pub fn handle<K:DynTreeTrait<T=BBot,Num=Numf32>>(tree:&mut K,rect:&Rect<f32>,max_prop:&BotProp){
		

	    
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
        	let mut rects=Rects::new(tree);//tree.create_rects();//Rects::new(tree);
			Self::handle2::<XAXIS_S,_>(max_prop,&mut rects,width,padding,rect);
        }
        {
        	let mut rects=Rects::new(tree);//tree.create_rects();//Rects::new(tree);
        	Self::handle2::<YAXIS_S,_>(max_prop,&mut rects,width,padding,rect);
        }
		/*
        for axis in axgeom::AxisIter::new(){
			let mut rects=Rects::new(tree);//tree.create_rects();//Rects::new(tree);
			
			Self::handle2(max_prop,&mut rects,width,padding,rect);
        }
        */


	    
	}


	

	fn handle2<'a,A:AxisTrait,K:DynTreeTrait<T=BBot,Num=Numf32>>(
		prop:&BotProp,
		
		rects:&mut Rects<K>,
		width:f32,
		padding:f32,rect:&Rect<f32>){
		//println!("Rect={:?}",rect);
		

		let top_d_axis=A::get();//axis;
		let left_r_axis=A::Next::get();//axis.next();
		//println!("{:?}",(top_d_axis,left_r_axis));

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
		//println!("rect111={:?}",rect1);
		//get bottom rect
		let rect2={
			let mut rr=rect.clone();
			rr.get_range_mut(top_d_axis).start=top_down_range.end-width;
			rr.get_range_mut(top_d_axis).end=top_down_range.end+padding;
			*rr.get_range_mut(left_r_axis)=left_right_range;
			rr
		};

		//println!("rect222={:?}",rect2);

		let mut func=|cc:ColPair<BBot>|{
			let top_down_length=top_down_length;
			let top_d_axis=top_d_axis;

		    //let mut pos=BotTrait::pos(bots_i.0).clone();
		    let mut pos=cc.a.0.pos.clone();
		    
		    *pos.get_axis_mut(top_d_axis)+=top_down_length;
		      
		    let mut bots={		        
		        //Change position to wrap around
		        let x=BotWrapper{bot_stuff:cc.a.0,acc:cc.a.1,pos:pos};
		        //let pp=*bots_i.1.pos();
		        let y=BotWrapper{bot_stuff:cc.b.0,acc:cc.b.1,pos:cc.b.0.pos};
		        (x,y)
		    };

		    //TODO fix

		    //TODO NEED TO FIXXX

		    //bot::collide(prop,&mut bots.0,&mut bots.1);
			
			
		};
		println!("wraparound fix");

		let rect1=bot::convert_to_nan(rect1);
		let rect2=bot::convert_to_nan(rect2);
		//println!("Rect1={:?}",rect1);
		//println!("Rect2={:?}",rect2);
		use dinotree::multirect;
		multirect::collide_two_rect_parallel::<A::Next,_,_>(rects,&rect1,&rect2,&mut func);
	}
	
}
