use axgeom;
use axgeom::Rect;
use botlib::bot::BotProp;
use dinotree;
use botlib::bot::Bot;
use botlib::bot;
use axgeom::AxisTrait;
use ordered_float::NotNaN;
use botlib::mouse::Mouse;
use sys;


pub struct WrapAround{}

impl WrapAround{

	pub fn handle_mouse(prop:BotProp,tree:&mut sys::Tree,rect:&Rect<NotNaN<f32>>,mouse:&Mouse){

		let mut mm=*mouse;
		
		let mut flipp=false;

		let mut ff=mm.midpoint;
		
		//let b=axgeom::Range{start:b.start.into_inner(),end:b.end.into_inner()};
    
		macro_rules! bla{
			($axis:ident)=>{
				let axis=$axis;
				let a=rect.get_range(axis);
				let aa=axgeom::Range{left:a.left.into_inner(),right:a.right.into_inner()};
	    	
				if mm.get_rect().get_range(axis).left<aa.left{
					*ff.get_axis_mut(axis)+=aa.len();
					flipp=true;
				}else if mm.get_rect().get_range(axis).right>aa.right{
					*ff.get_axis_mut(axis)-=aa.len();
					flipp=true;
				}
			}
		}
		use axgeom::XAXISS;
		use axgeom::YAXISS;
		bla!(XAXISS);
		bla!(YAXISS);

		if !flipp{
			return;
		}

		mm.move_to(&ff);    

		let _ =dinotree::multirect::multi_rect_mut(tree).for_all_in_rect_mut(bot::convert_to_nan(*mm.get_rect()),
			&mut |a:&mut dinotree::support::BBox<NotNaN<f32>,Bot>|{bot::collide_mouse(&mut a.inner,&prop,mouse);});
	
	}
	pub fn handle(tree:&mut sys::Tree,rect:&Rect<NotNaN<f32>>,max_prop:BotProp){
		

	    
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
		Self::handle2(axgeom::XAXISS,&max_prop,tree,width,padding,rect);
        Self::handle2(axgeom::YAXISS,&max_prop,tree,width,padding,rect);
	}


	

	fn handle2<A:AxisTrait>(
		axis:A,
		prop:&BotProp,
		tree:&mut sys::Tree,
		width:f32,
		padding:f32,rect:&Rect<NotNaN<f32>>){

		let a=rect.get_range(axgeom::XAXISS);
		let b=rect.get_range(axgeom::YAXISS);
		let rect=Rect::new(a.left.into_inner(),a.right.into_inner(),b.left.into_inner(),b.right.into_inner());



		let top_d_axis=axis;
		let left_r_axis=axis.next();
		let top_down_range=rect.get_range(top_d_axis);

		let top_down_length=rect.get_range(top_d_axis).right-rect.get_range(top_d_axis).left;


		let left_right_range=*rect.get_range(left_r_axis).clone().grow(width);

		//get top rect
		let rect1={
			let mut rr=rect.clone();
			rr.get_range_mut(top_d_axis).left=top_down_range.left-padding;   
			rr.get_range_mut(top_d_axis).right=top_down_range.left+width;
			*rr.get_range_mut(left_r_axis)=left_right_range;
			rr
		};

		//get bottom rect
		let rect2={
			let mut rr=rect.clone();
			rr.get_range_mut(top_d_axis).left=top_down_range.right-width;
			rr.get_range_mut(top_d_axis).right=top_down_range.right+padding;
			*rr.get_range_mut(left_r_axis)=left_right_range;
			rr
		};

		let rect1=bot::convert_to_nan(rect1);
		let rect2=bot::convert_to_nan(rect2);

		let bo=|aa:&mut dinotree::support::BBox<NotNaN<f32>,Bot>,bb:&mut dinotree::support::BBox<NotNaN<f32>,Bot>|{

			    let mut copy_botstuff=aa.inner.clone();
			    let mut pos=aa.inner.pos.clone();
			    *pos.get_axis_mut(top_d_axis)+=top_down_length;
			    copy_botstuff.pos=pos;
			     

			    //let cca=ColSingle{rect:a.rect,inner:&mut copy_botstuff};
			    //let ccb=ColSingle{rect:b.rect,inner:b.inner};
			    
			    //bot::collide(&prop,cca,ccb);
			    bot::collide(&prop,&mut copy_botstuff,&mut bb.inner);
		
		};
		let mut m=dinotree::multirect::multi_rect_mut(tree);
		let _ = dinotree::multirect::collide_two_rect_parallel(&mut m,axis.next(),&rect1,&rect2,bo);
		
	}
	
}
