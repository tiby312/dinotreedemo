use botlib::prop::RadiusProp;
use axgeom;
use botlib::mouse::MouseProp;
use super::mouse::Mouse;
use ordered_float::NotNaN;
use axgeom::Rect;


use dinotree::*;
use dinotree::support::Numf32;



#[derive(Copy,Clone,Debug)]
pub struct BotProp {
    pub radius: RadiusProp,
    pub collision_push: f32,
    pub collision_drag: f32,
    pub minimum_dis_sqr: f32,
    pub max_acc:f32
}




#[derive(Copy,Clone,Debug)]
pub struct BBot{
    pub rect:AABBox<Numf32>,
    pub inner:Bot
}

#[derive(Copy,Clone,Debug)]
pub struct Bot{
    pub pos: axgeom::Vec2,
    pub vel: axgeom::Vec2,
    pub acc: axgeom::Vec2
}

impl BBot{
    fn new(posa:&axgeom::Vec2,rect:AABBox<Numf32>)->BBot{
        let pos=*posa;
        let vel=axgeom::Vec2::new(0.0,0.0);
        let acc=vel;
        BBot{inner:Bot{pos,vel,acc},rect}
    }
    fn pos(&self)->&axgeom::Vec2{
        &self.inner.pos
    }
    fn vel(&self)->&axgeom::Vec2{
        &self.inner.vel
    }
    fn get_acc(&self)->&axgeom::Vec2{
        &self.inner.acc
    }
    pub fn update_box(&mut self,radius:&f32){
        let r:Rect<f32>=Rect::from_pos_and_radius(&self.inner.pos,*radius);
        
        self.rect=convert_aabbox(convert_to_nan(r));
    }
}

pub fn convert_aabbox(r:Rect<Numf32>)->AABBox<Numf32>{

    let a=r.get_range2::<axgeom::XAXIS_S>();
    let b=r.get_range2::<axgeom::YAXIS_S>();
    
    AABBox::new((a.start,a.end),(b.start,b.end))
}
pub fn convert_to_nan(r:Rect<f32>)->Rect<Numf32>{

    let a=r.get_range2::<axgeom::XAXIS_S>();
    let b=r.get_range2::<axgeom::YAXIS_S>();
    
    let rect=Rect::new(
        Numf32(NotNaN::new(a.start).unwrap()),
        Numf32(NotNaN::new(a.end).unwrap()),
        Numf32(NotNaN::new(b.start).unwrap()),
        Numf32(NotNaN::new(b.end).unwrap())
        );
    rect
}

impl SweepTrait for BBot{
    type Inner=Bot;
    type Num=Numf32;

    ///Destructure into the bounding box and mutable parts.
    fn get_mut<'a>(&'a mut self)->(&'a AABBox<Numf32>,&'a mut Self::Inner){
        (&self.rect,&mut self.inner)
    }

    ///Destructue into the bounding box and inner part.
    fn get<'a>(&'a self)->(&'a AABBox<Numf32>,&'a Self::Inner){
        (&self.rect,&self.inner)
    }
}

//Exists so that wraparound can implement a custom bot
//TODO move to wrap around?
/*
pub trait BotTrait{
    fn pos(&self)->&axgeom::Vec2;
    fn vel(&self)->&axgeom::Vec2;
    fn apply_force(&mut self,vec:&axgeom::Vec2);
    fn get_acc(&self)->&axgeom::Vec2;
}
*/
/*
impl BotTrait for BBot{
    fn pos(&self)->&axgeom::Vec2{
        &self.stuff.pos
    }
    fn vel(&self)->&axgeom::Vec2{
        &self.stuff.vel
    }
    fn apply_force(&mut self,vec:&axgeom::Vec2){
        self.val.acc+=vec;
    }
    fn get_acc(&self)->&axgeom::Vec2{
        &self.val.acc
    }
}
*/

/*
//Exists so that multple collision handling strategies can exist
pub trait BotMovementTrait{
    type Prop;
    fn collide<X:BotTrait>(&Self::Prop,&mut X,&mut X);
    fn collide_mouse<X:BotTrait>(&mut X,&Self::Prop,&Mouse);
    
    fn update(bots:&mut BBot,prop:&BotProp,rect:&axgeom::Rect<f32>);   
}
*/


pub fn collide(prop:&BotProp,a:ColSingle<BBot>,b:ColSingle<BBot>){
   
    let bots=[a.1,b.1];



    //let vals=[cc.a.1,cc.b.1];



    let offset = bots[0].pos - bots[1].pos;

    let dis_sqr = offset.len_sqr();

    
    if dis_sqr >= prop.radius.radius2_squared() {
        return ;
    }

    //At this point, we know they collide!!!!

    //if directly ontop of each other
    if dis_sqr < prop.minimum_dis_sqr {
        let vec=axgeom::Vec2::new(prop.max_acc,0.0); //TODO dont hardcode. and test
        bots[0].acc+=vec;//apply_force(&vec);
        bots[1].acc+=-vec;//.apply_force(&-vec);
        return;
    }

    let sum_rad = prop.radius.radius2();

    let dis = dis_sqr.sqrt();

    

    //let input=(sum_rad-dis)/sum_rad;
    //let push_mag_norm=(2.0*(input*input)).min(1.0);
    

    let sum_rad_sqr=prop.radius.radius2_squared();
    let input=sum_rad-dis;
    //(a-b)(a-b)=a*a-2ab+b*b
    let push_mag_norm=(3.0*(input*input)/sum_rad_sqr).min(1.0);
    

    //assert!(push_mag_norm<1.1);
    let push_mag=push_mag_norm*prop.collision_push;
    //let r=prop.radius.radius2_squared();
    //let push_mag=(((r-dis_sqr)/r))*prop.collision_push;




    let push_force=offset*(push_mag/dis);
    
    //center of mass velocity (treating mass of each bot as one s.t. their sum mass is 2)
    let cvel = (bots[0].vel + bots[1].vel ) / 2.0;

    //take the component of velocity (in the reference of center of mass) along the offset, and use to calculate drag
    let mag = [
                -(bots[0].vel - cvel).inner_product(&offset),
                -(bots[1].vel - cvel).inner_product(&offset)
            ];

    let k=prop.collision_drag/dis_sqr;
    let drag_force=[
                offset * (mag[0] * k),
                offset * (mag[1] * k)
                ];

    let acc=[
        (drag_force[0] + push_force),
        (drag_force[1] - push_force)
    ];
    
    bots[0].acc+=acc[0];//.apply_force(&acc[0]);
    bots[1].acc+=acc[1];//bots[1].apply_force(&acc[1]);
}


pub fn update_bot(bot:&mut BBot,prop:&BotProp,rect:&axgeom::Rect<f32>) {
    {
        //let bot=&mut bota.val;

        for j in axgeom::AxisIter::new() {

            let a=rect.get_range(j).start;
            let b=rect.get_range(j).end;

            let mut new_pos=bot.inner.pos.clone();

            if *bot.inner.pos.get_axis(j) < a {
                *new_pos.get_axis_mut(j) = b;
            }
            if *bot.inner.pos.get_axis(j) > b {
                *new_pos.get_axis_mut(j) = a;
            }
            bot.inner.pos=new_pos;
        }

        //if velocity is nan, just set it to zero and conitnue. so we dont pollute the position to also be nan.
        /*
        if bot.acc.is_nan() {
            bot.acc = axgeom::Vec2::new(0.0, 0.0);
        }
        */

        let acc_sqr=bot.inner.acc.len_sqr();
        if acc_sqr>=prop.max_acc.powi(2){
            bot.inner.acc=bot.inner.acc*(prop.max_acc/acc_sqr);
        }
        
        {
            let mut kk=bot.inner.vel;
            kk+=bot.inner.acc;
            bot.inner.vel=kk;    
        }
        {
            let mut kk=bot.inner.pos;
            kk+=bot.inner.vel;
            bot.inner.pos=kk;
        }

        bot.inner.acc.set(0.0,0.0);
   
    }
    //TODO inefficient?
    //let pos=bota.val.pos;
    bot.update_box(&prop.radius.radius());   
    

}
pub fn collide_mouse(bot:&mut ColSingle<BBot>,prop:&BotProp,mouse:&Mouse){
    //let stuff=bot.0;
    //let val=bot.1;

    let offset = *mouse.get_midpoint() - bot.1.pos;
    let dis_sqr = offset.len_sqr();
    
    if dis_sqr < (mouse.get_radius() + prop.radius.radius()).powi(2) {

        let dis = dis_sqr.sqrt();

        //let offset_norm = offset / dis;

        let mag = -(1.0 - (dis / mouse.get_radius())) * mouse.mouse_prop.force;

        let blap = offset * ( (  mag/dis) ) ;

        let acc = blap;// / prop.mass;
        bot.1.acc+=acc;
        //bot.apply_force(&acc);
    }
}

pub fn compute_bot_radius(num_bots: usize, world: &axgeom::Rect<f32>) -> Option<f32> {
    let a=world.get_range(axgeom::XAXIS);
    let b=world.get_range(axgeom::YAXIS);
    //println!("{:?}",(a,b));
    let width=a.end-a.start;
    let height=b.end-b.start;
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
    Some(radius)
}


/*
pub fn create_props(num_bots:usize,world:&axgeom::Rect<f32>)->(BotProp,MouseProp){

    


    let rad = compute_bot_radius(num_bots, &world).unwrap();


    //In order to take advantage of constant folding, keep
    //these are hardcoded values.
    let bot_prop = BotProp {
        radius: RadiusProp::create(rad),
        collision_push: rad*0.2,
        collision_drag: rad*0.02,
        minimum_dis_sqr: 0.000001,
        max_acc:rad*0.3
    }; 


    let a=world.get_range(axgeom::XAXIS);
    let b=world.get_range(axgeom::YAXIS);
    let area=a.len()*b.len();    
    let mouse_prop = MouseProp {
        radius: RadiusProp::create(area*0.0001),
        force: rad*0.3,
    };

    (bot_prop,mouse_prop)
}
*/

pub fn get_unit(startx:usize,starty:usize)->f32{
    (startx.min(starty) as f32)/100.0
}
pub fn create_from_radius(bot_radius:f32,mouse_radius:f32)->(BotProp,MouseProp){

    //In order to take advantage of constant folding, keep
    //these are hardcoded values.
    let bot_prop = BotProp {
        radius: RadiusProp::create(bot_radius),
        collision_push: bot_radius*0.2,
        collision_drag: bot_radius*0.02,
        minimum_dis_sqr: 0.000001,
        max_acc:bot_radius*0.3
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


pub fn create_bots_spaced<X,Y:Fn(&axgeom::Vec2)->X>(world:&axgeom::Rect<f32>,num_bot:usize,spacing:f32,func:Y)->Vec<X>{

    let a=world.get_range(axgeom::XAXIS);
    let b=world.get_range(axgeom::YAXIS);

    let start = axgeom::Vec2::new(a.start,b.start) + axgeom::Vec2::new(spacing, spacing);
    //let spacing = bot_prop.radius.radius2();

    let mut cursor = start.clone();
    
    let mut man=Vec::with_capacity(num_bot);
    for _ in 0..num_bot{
        let b=func(&cursor);//Bot::new(&cursor);
        cursor += axgeom::Vec2::new(spacing, 0.0);
        if *cursor.get().0 > a.end {
            *cursor.get_mut().0 = *start.get().0;
            cursor += axgeom::Vec2::new(0.0, spacing);
        }
        man.push(b);
    }
    man    
}


pub fn update(bots:&mut [BBot],prop:BotProp,rect:&axgeom::Rect<f32>) {
    //self.last_man.clone_from_slice(&self.man);

    for bot in bots.iter_mut() {
        self::update_bot(bot,&prop,rect);
    }
}



pub fn create_bots(num_bot:usize, world:&axgeom::Rect<f32>, bot_prop: &BotProp)->Vec<BBot>{
    let man={
        let pp=&bot_prop;
        create_bots_spaced(world,num_bot,bot_prop.radius.radius2(),|vec:&axgeom::Vec2|{
            
            let r=axgeom::Rect::from_pos_and_radius(vec,pp.radius.radius());   
            BBot::new(vec,convert_aabbox(convert_to_nan(r)))
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

