use botlib::prop::RadiusProp;
use axgeom;
use botlib::mouse::MouseProp;
use super::mouse::Mouse;
use ordered_float::NotNaN;
use axgeom::Rect;
use vec::Vec2;



#[derive(Copy,Clone,Debug)]
pub struct BotProp {
    pub radius: RadiusProp,
    pub collision_push: f32,
    pub collision_drag: f32,
    pub minimum_dis_sqr: f32,
}


#[derive(Copy,Clone,Debug)]
pub struct Bot{
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}
impl Bot{
    pub fn create_bbox(&self,radius:f32)->Rect<NotNaN<f32>>{
        let p=self.pos.0;
        let r=radius;
        let r=Rect::new(p[0]-r,p[0]+r,p[1]-r,p[1]+r);
        convert_to_nan(r)
    }
    pub fn new(a:Vec2)->Bot{
        let pos=a;
        let vel=Vec2([0.0;2]);
        let acc=vel;
        Bot{pos,vel,acc}
    }
}



pub fn convert_to_nan(r:Rect<f32>)->Rect<NotNaN<f32>>{

    let a=r.get_range(axgeom::XAXISS);
    let b=r.get_range(axgeom::YAXISS);
    
    let rect=Rect::new(
        NotNaN::new(a.left).unwrap(),
        NotNaN::new(a.right).unwrap(),
        NotNaN::new(b.left).unwrap(),
        NotNaN::new(b.right).unwrap()
        );
    rect
}



//Takes a values between 0 and 1, and returns a value between 0 and 1.
//The input is the distance from not touching.
//So if the bots are just barely touching, the input will be 0.
//If they are right on top of each other, the input will be 1.

//The output is the normalized force with which to handle.
//A value of 0 is no force.
//A value of 1 is full force.
pub fn handle_repel(input:f32)->f32{
    let a=3.0*input*input;
    a.min(1.0)
}



pub fn collide(prop:&BotProp,a:&mut Bot,b:&mut Bot){
    let bots=[a,b];

    let offset = bots[0].pos - bots[1].pos;

    let dis_sqr = offset.len_sqr();
    
    if dis_sqr >= prop.radius.radius2_squared() {
        //They not touching (bots are circular).
        return ;
    }

    //At this point, we know they collide!!!!



    let sum_rad = prop.radius.radius2();

    let dis = dis_sqr.sqrt();

    //TODO #[cold]
    if dis<0.0001{
        let vec=Vec2::new(prop.collision_push,0.0); //TODO dont hardcode. and test
        bots[0].acc+=vec;
        bots[1].acc+=-vec;
        return;
    }
    
    //0 if barely touching (not touching)
    //1 if directly on top of each other
    let dd=(sum_rad-dis)/sum_rad;

    let ammount_touching=handle_repel(dd);

    let push_mag={
        ammount_touching*prop.collision_push
    };
    
    let [drag1,drag2]={
        
        //center of mass velocity (treating mass of each bot as one s.t. their sum mass is 2)
        let cvel = (bots[0].vel + bots[1].vel ) / 2.0;

        //take the component of velocity (in the reference of center of mass) along the offset, and use to calculate drag
        let mag = [
                    -(bots[0].vel - cvel).inner_product(&offset),
                    -(bots[1].vel - cvel).inner_product(&offset)
                ];

        let k=prop.collision_drag*ammount_touching;///dis_sqr; //why does dis_sqr scale better than dis???
        
        [mag[0]*k,mag[1]*k]        
    };
    
    let push1=drag1+push_mag;
    let push2=drag2-push_mag;

    let push_force1=offset*(push1/dis);
    let push_force2=offset*(push2/dis);

    //TODO hardcoded value
    let viscous=(bots[0].vel-bots[1].vel)*-0.03*ammount_touching;

    bots[0].acc+=push_force1;
    bots[0].acc+=viscous;

    bots[1].acc+=push_force2;
    bots[1].acc+=viscous;
}



pub fn collide_mouse(bot:&mut Bot,prop:&BotProp,mouse:&Mouse){

    let offset = *mouse.get_midpoint() - bot.pos;
    let dis_sqr = offset.len_sqr();
    
    let sum_rad=mouse.get_radius() + prop.radius.radius();
    if dis_sqr < sum_rad*sum_rad {

        let dis = dis_sqr.sqrt();

        if dis<0.0001{
            return;
        }

        let vv=(sum_rad-dis)/sum_rad;
        let vv=vv*vv;
        let vv2=(5.0*vv).min(1.0);


        let push_mag=vv2*mouse.mouse_prop.force;
        let push_force=offset*(push_mag/dis);

        bot.acc+=-push_force;
    }
}

pub fn compute_bot_radius(num_bots: usize, world: &axgeom::Rect<NotNaN<f32>>) -> Option<f32> {
    let a=world.get_range(axgeom::XAXISS);
    let b=world.get_range(axgeom::YAXISS);
    let a=axgeom::Range{left:a.left.into_inner(),right:a.right.into_inner()};
    let b=axgeom::Range{left:b.left.into_inner(),right:b.right.into_inner()};
    //println!("{:?}",(a,b));
    let width=a.right-a.left;
    let height=b.right-b.left;
    //println!("width:height={:?}",(width,height));
    let aspect_ratio = width / height;


    let mut rows = (num_bots as f32 / aspect_ratio).sqrt().ceil() as usize;
    let mut columns = (aspect_ratio * rows as f32).ceil() as usize;
    rows+=1;
    columns+=1;
    //println!("rows={} columns={}", rows, columns);

    let radius = if rows > columns {
        height / (rows * 2) as f32
    } else {
        width / (columns * 2) as f32
    };

    if radius < 0.00001 {
        return None; //the radius required to have the spacing we need and everything is either impossible for very very small.
    }

    let radius=radius*1.7 ;
    //Some(10.0)
    Some(radius)
}



pub fn get_unit(startx:usize,starty:usize)->f32{
    (startx.min(starty) as f32)/100.0
}
pub fn create_from_radius(bot_radius:f32,mouse_radius:f32)->(BotProp,MouseProp){

    //In order to take advantage of constant folding, keep
    //these are hardcoded values.
    let bot_prop = BotProp {
        radius: RadiusProp::create(bot_radius),
        collision_push: bot_radius*0.2,
        collision_drag: bot_radius*0.01,
        minimum_dis_sqr: 0.000001,
        //max_acc:20.0//bot_radius*0.3
    }; 


    //let a=world.get_range(axgeom::XAXIS);
    //let b=world.get_range(axgeom::YAXIS);
    //let area=a.len()*b.len();    
    let mouse_prop = MouseProp {
        radius: RadiusProp::create(mouse_radius),
        force: mouse_radius*0.03,
    };

    (bot_prop,mouse_prop)
}


pub fn create_bots_spaced<X,Y:Fn(&Vec2)->X>(world:&axgeom::Rect<NotNaN<f32>>,num_bot:usize,spacing:f32,func:Y)->Vec<X>{

    let a=world.get_range(axgeom::XAXISS);
    let b=world.get_range(axgeom::YAXISS);
    let a=axgeom::Range{left:a.left.into_inner(),right:a.right.into_inner()};
    let b=axgeom::Range{left:b.left.into_inner(),right:b.right.into_inner()};
    let start = Vec2::new(a.left,b.left) + Vec2::new(spacing, spacing);
    //let spacing = bot_prop.radius.radius2();

    let mut cursor = start.clone();
    
    let mut man=Vec::with_capacity(num_bot);
    for _ in 0..num_bot{
        let bb=func(&cursor);
        cursor += Vec2::new(spacing, 0.0);
        if cursor.0[0] > a.right {
            cursor.0[0] = start.0[0];
            cursor += Vec2::new(0.0, spacing);
            if cursor.0[1]>b.right{
                cursor=start;
            }
        }

        man.push(bb);
    }
    man    
}



pub fn update(bots:&mut [Bot],prop:BotProp,rect:&axgeom::Rect<NotNaN<f32>>) {
    for bot in bots.iter_mut() {
        bot.vel+=bot.acc;    
        bot.pos+=bot.vel;
        bot.acc.0=[0.0;2];
    }
}



pub fn create_bots(num_bot:usize, world:&axgeom::Rect<NotNaN<f32>>, bot_prop: &BotProp)->Vec<Bot>{
    let man={
        //let pp=&bot_prop;
        create_bots_spaced(world,num_bot,bot_prop.radius.radius2(),|vec:&Vec2|{
        
            Bot::new(*vec)
            //BBot{val:b,rect:convert_to_nan(r)}
            //BBot::new(b,r)
        })
    };
    man
}

/*
pub struct BotMan{
    pub max_prop:BotProp,
    pub man:Vec<BBot>,
}
impl BotMan{
    pub fn new(num_bot:usize, world:&axgeom::Rect<f32>, bot_prop: BotProp ) -> BotMan {
        
        let man={
            let pp=&bot_prop;
            create_bots_spaced(world,num_bot,bot_prop.radius.radius2(),|vec:&axgeom::Vec2|{
                let b=Bot::new(vec);
                let r=axgeom::Rect::from_pos_and_radius(vec,pp.radius.radius());   
                BBot{val:b,rect:convert_to_nan(r)}
                //BBot::new(b,r)
            })
        };

        BotMan {
            max_prop:bot_prop,
            man:man,
        }
    }


}
*/



/*
pub mod pipeline{

    use super::*;
    use collie::collide::kdtree::serialize::Serializer;
    //      |rebal(0)|
    //      |rebal(1)|query(0)|
    //      |rebal(2)|query(1)|
    //      |rebal(3)|query(2)|
    //      |rebal(4)|query(3)|
    //
    //    
    pub struct Pipeline{
        counter:usize,
        bots:[Vec<Bot>;2]
    }


    pub struct Serial{
        fa:Option<Box<Serializer<Bot>>>
    }

    impl Serial{
        pub fn new()->Serial{
            Serial{fa:None}
        }
        pub fn get(&mut self)->Option<Box<Serializer<Bot>>>{
            self.fa.take()
        }
        pub fn put(&mut self,s:Box<Serializer<Bot>>){
            self.fa=Some(s);
        }
    }

    impl Pipeline{
        pub fn new(bot:Vec<Bot>)->Pipeline{
            let bot2=bot.clone();
            Pipeline{counter:0,bots:[bot,bot2]}
        }


        //Returns current, and then previous if it exists
        pub fn get<'a>(&self)->(&[Bot],&[Bot]){
            let (a,b)=self.get_current();
            (a,b)
        }

        //Returns current, and then previous if it exists
        pub fn get_mut<'a>(&'a mut self)->(&'a mut [Bot],&'a mut [Bot]){
            let (a,b)=self.get_current_mut();
            (a,b)
        }


        fn get_current(&self)->(&[Bot],&[Bot]){
            //let (a,b)=self.bots.split_at_mut(1);
            if self.counter==0{
                (&self.bots[0],&self.bots[1])
            }else{
                (&self.bots[1],&self.bots[0])
            }
        }

        fn get_current_mut(&mut self)->(&mut [Bot],&mut [Bot]){
            let (a,b)=self.bots.split_at_mut(1);
            if self.counter==0{
                (&mut a[0],&mut b[0])
            }else{
                (&mut b[0],&mut a[0])
            }
        }


        pub fn finish(&mut self){
            {
                //let (a,b)=self.get_current_mut();
                //b.copy_from_slice(a);
            }
            self.counter=1-self.counter;
        }

    }
}*/

