use axgeom;
use botlib::graphics::BotLibGraphics;
use dinotree::graphics::GenTreeGraphics;
use dinotree::DinoTree;
use dinotree::multirect::Rects;
use dinotree::*;
use dinotree::tools::par;
use dinotree::median::*;
use dinotree::support::Numf32;
use dinotree;
use dinotree::DynTreeTrait;
use axgeom::Rect;
use wrap_around::WrapAround;
use botlib::mouse::Mouse;
use botlib::bot::Bot;

//use simpdraw;
use std;
use botlib::bot::BotProp;
use botlib::mouse::MouseProp;
use botlib::bot;
use std::marker::PhantomData;
use kenmisc;
//use kenmisc::log::Logger;
use dinotree::support::DefaultDepthLevel;
use axgeom::AxisTrait;
use botlib::bot::BBot;
//use ordered_float::NotNaN;
//use super::mlog;
//use sys::log::LogT;
//use testy;
use Vert;
pub mod log2{

    
    use std::fs::File;
    use std::io::Write;
    


    pub struct Logger{
        file:File,
        counter:usize
    }
    impl Logger{

        pub fn new(str:&'static str)->Logger{
            
            let file = File::create(str).unwrap();
            Logger{file:file,counter:0}
        }

        pub fn with_names(str:&'static str,names:&[&'static str])->Logger{
            
            let mut file = File::create(str).unwrap();
           
            write!(file,"Iteration,").unwrap();
            for k in names{
                write!(file,"{},",k).unwrap();    
            }
            writeln!(file,"").unwrap();
            Logger{file:file,counter:0}
        }

        pub fn write_str(&mut self,strf:&'static str,slice:&[String]){

            write!(self.file,"{},",strf).unwrap();
            for k in slice{
                write!(self.file,"{},",k).unwrap();    
            }
            writeln!(self.file,"").unwrap();
            
        }
        pub fn write_data(&mut self,slice:&[f64]){
            
            write!(self.file,"{},",self.counter).unwrap();
            for k in slice{
                write!(self.file,"{},",k).unwrap();    
            }
            writeln!(self.file,"").unwrap();
            self.counter+=1;
            
        }
    }
}




pub mod log{
    pub enum Typ {
        Rebal,
        Query,
        RebalQuery,
        BotUpdate,
        ContDyn,
        Graphic,
        Total,
    }

    pub trait LogT{
        fn write(&mut self,e:Typ,val:f64);
        fn newline(&mut self);
    }  
}





pub fn compute_tree_height(num_bots: usize, radius: f32) -> usize {
    let _ = radius;
    //we want each node to have space for around 300 bots.
    //there are 2^h nodes.
    //2^h*200>=num_bots.  Solve for h s.t. h is an integer.

    const NUM_PER_NODE: usize = 10; //TODO make this a user option.
    if num_bots <= NUM_PER_NODE {
        return 1;
    } else {
        return (num_bots as f32 / NUM_PER_NODE as f32).log2().ceil() as usize;
    }
}





pub trait TreeDraw{
    fn get_num_verticies(height:usize)->usize;
    fn update<A:AxisTrait>(rect:&Rect<f32>,tree:&TreeCache<A,Numf32>,verts:&mut [Vert]);
}

pub struct TreeDrawReal{
}
impl TreeDraw for TreeDrawReal{
    fn get_num_verticies(height:usize)->usize{
        GenTreeGraphics::get_num_verticies(height)
    }
    fn update<A:AxisTrait>(rect:&Rect<f32>,tree:&TreeCache<A,Numf32>,verts:&mut [Vert]){
        
        #[derive(Clone,Default,Copy)]
        struct Bo(Vert);
        impl dinotree::graphics::Vertex for Bo{
            fn set_pos(&mut self,x:f32,y:f32){
                (self.0).0[0]=x;
                (self.0).0[1]=y;
            }
        }

        let k:&mut [Bo]=unsafe{std::mem::transmute(verts)};
        GenTreeGraphics::update(bot::convert_to_nan(*rect),tree,k,10.0);
    }
}
pub struct TreeNoDraw{
}
impl TreeDraw for TreeNoDraw{
    fn get_num_verticies(_height:usize)->usize{
        0
    }
    fn update<A:AxisTrait>(_rect:&Rect<f32>,_tree:&TreeCache<A,Numf32>,_verts:&mut [Vert]){
    }
}




/*
pub struct BotSysGraphics{
    pub drawer:simpdraw::Drawer<Vert>,
    pub bot_handle:simpdraw::RangeID,
    pub tree_handle:simpdraw::RangeID
}

impl BotSysGraphics{
    pub fn new<TDraw:TreeDraw>(bots:&[BBot],height:usize)->BotSysGraphics{
       

        let mut drawer=simpdraw::Drawer::new();
        let bot_handle=drawer.add(BotLibGraphics::get_num_verticies(bots.len()));
        let tree_handle=drawer.add(TDraw::get_num_verticies(height)); //TODO FIX ME

        BotSysGraphics{drawer:drawer,bot_handle:bot_handle,tree_handle:tree_handle}
    }
}
*/






pub struct BotSystem<A:AxisTrait,TDraw:TreeDraw> {
    //graphics:BotSysGraphics,
    bot_graphics:BotLibGraphics,
    mouse_prop:MouseProp,
    bots: Vec<BBot>,
    bot_prop:BotProp,
    border: axgeom::Rect<f32>,
    treecache:TreeCache<A,Numf32>,
    phantom:PhantomData<TDraw>,
    /*
    general_log:mlog::MLog,
    rebal_log:Logger,
    colfind_log:Logger
    */
}


pub trait BotSysTrait{
    fn get_verticies(&self,a:&mut [Vert]);
    fn get_num_verticies(&self)->usize;
    fn step(&mut self, poses: &[axgeom::Vec2]);
}

impl<A:AxisTrait,TDraw:TreeDraw> BotSysTrait for BotSystem<A,TDraw>{

    fn get_num_verticies(&self)->usize{

        let height = compute_tree_height(self.bots.len(), self.bot_prop.radius.radius());
        

        TDraw::get_num_verticies(height)+BotLibGraphics::get_num_verticies(self.bots.len())
    }
    fn get_verticies(&self,verts:&mut [Vert])  {
        let height = compute_tree_height(self.bots.len(), self.bot_prop.radius.radius());
        
        let (a,b)=verts.split_at_mut(TDraw::get_num_verticies(height));

        TDraw::update(&self.border,&self.treecache,a);

        self.bot_graphics.update(&self.bot_prop,&self.bots,b);
    }

    fn step(&mut self, poses: &[axgeom::Vec2]) {
        //let rebal_log=&mut self.rebal_log;
        //let colfind_log=&mut self.colfind_log;
        //let mlog=&mut self.general_log;


        let _time_all=kenmisc::Timer2::new();
        let bots=&mut self.bots;
        {                
            let border=&self.border;
            let mouse_prop=&self.mouse_prop;
            let bot_prop=&self.bot_prop;
            let treecache=&mut self.treecache;
            //let graphics=&mut self.graphics;
            //let bot_graphics=&mut self.bot_graphics;
            {

                let _rebal=kenmisc::Timer2::new();

                
                

               
                //      |rebal(0)|
                //      |rebal(1)|query(0)|
                //      |rebal(2)|query(1)|
                //      |rebal(3)|query(2)|
                //      |rebal(4)|query(3)|
                //
                //      

                


                {
                    struct B(
                        f32
                    );
                    impl median::DivMoveStrat for B{
                        type N=Numf32;
                        fn move_divider(&self,a:&mut Self::N,total:usize,b:f32){
                            
                            let total=total as f32;

                            //add a little more urgency to dividers near the root.
                            let total=total*0.003+self.0;
                            a.0+=b*total;
                        }
                    }
                    
                    //let bb=MedianRelax::new(B(4.0));
                    let bb=MedianRelax::new(B(self.bot_prop.radius.radius()));
                    
                    let (mut dyntree,_bag)=DinoTree::new::<par::Parallel,DefaultDepthLevel,_,treetimer::TreeTimerEmpty>
                        (bots,treecache,&bb);
                    
                    //rebal_log.write_data(&bag.into_vec());


                    //the dynamic tree made a copy of the bots.
                    //so here we can still use bo.man.
                    //later will add together the copy and the source.
                    
                    {
                        let mut handle_collisions=||{
                            //mlog.write(log::Typ::Rebal,rebal.elapsed());
                                


                            //let query=kenmisc::Timer2::new();

                            let clos=|cc:ColPair<BBot>|{
                                use botlib::bot::BotMovementTrait;

                                Bot::collide(bot_prop,cc.a.1,cc.b.1);
                            };

                            let _v=dyntree.for_every_col_pair::<DefaultDepthLevel,_,treetimer::TreeTimer2>(clos);
                            //colfind_log.write_data(&v.into_vec());
                
                            
                            //mlog.write(log::Typ::Query,query.elapsed());
                            

                            WrapAround::handle(&mut dyntree,border,bot_prop);   

                            

                            for k in poses{
                                let mouse=Mouse::new(k,mouse_prop);
                                handle_mouse(bot_prop,&mut dyntree,&mouse);
                                WrapAround::handle_mouse(bot_prop,&mut dyntree,border,&mouse);
                            }
                        };

                        handle_collisions();
                    }

                    
                    //mlog.write(log::Typ::RebalQuery,rebal.elapsed());

                }
                

                //TDraw::update(border,treecache,graphics.drawer.get_range_mut(&graphics.tree_handle));
        
                {
                    {
                        let _upd=kenmisc::Timer2::new();
                        bot::update(bots,bot_prop,border);
                        //bo.update(&self.border);
                        //mlog.write(log::Typ::BotUpdate,upd.elapsed());
                    }
                   
                    {
                        let _upd=kenmisc::Timer2::new();

                        //let verts=graphics.drawer.get_range_mut(&graphics.bot_handle);
                        
                        //bot_graphics.update(bot_prop,bots,verts);
                        //mlog.write(log::Typ::Graphic,upd.elapsed());
                    }
                }
            
            
                //mlog.write(log::Typ::Total,time_all.elapsed());
                //mlog.newline();
            }

            
            //bots
        }
    
                
    }
}

pub fn new(num_bots:usize,startx:usize,starty:usize,draw_tree:bool)->Box<BotSysTrait>{
    use axgeom::XAXIS_S;
    use axgeom::YAXIS_S;

    
    if startx>=starty{
        if draw_tree{
            let k=BotSystem::<XAXIS_S,TreeDrawReal>::new_inner(num_bots,startx,starty);
            Box::<BotSystem<XAXIS_S,TreeDrawReal>>::new(k)
        }else{
            let k=BotSystem::<XAXIS_S,TreeNoDraw>::new_inner(num_bots,startx,starty);
            Box::<BotSystem<XAXIS_S,TreeNoDraw>>::new(k)
        }
    }else{
        if draw_tree{
            let k=BotSystem::<YAXIS_S,TreeDrawReal>::new_inner(num_bots,startx,starty);
            Box::<BotSystem<YAXIS_S,TreeDrawReal>>::new(k)
        }else{
            let k=BotSystem::<YAXIS_S,TreeNoDraw>::new_inner(num_bots,startx,starty);
            Box::<BotSystem<YAXIS_S,TreeNoDraw>>::new(k)
        }
    }
}

impl<A:AxisTrait,TDraw:TreeDraw> BotSystem<A,TDraw> {

    
    fn new_inner(num_bots:usize,startx:usize,starty:usize) -> BotSystem<A,TDraw> {
        let world= axgeom::Rect::new(0.0,startx as f32,0.0,starty as f32);
    
        let br=bot::compute_bot_radius(num_bots,&world).unwrap();
        
        let unit=bot::get_unit(startx,starty);
        let (bot_prop,mouse_prop)=bot::create_from_radius(br,unit*10.0);



        let mut bots = bot::create_bots(num_bots,&world,&bot_prop);

        //let r=bot_prop.radius.radius2()*2.0;
        //let padded_world:Rect<f32>=*world.clone().grow(r);

        //TODO should it be based on max prop or average prop
        let height = compute_tree_height(bots.len(), bot_prop.radius.radius());
        //println!("fheight={:?}",height);

        let mut treecache=TreeCache::new(height);

        {         
            let k=MedianStrict::<Numf32>::new();
            let (_dyntree,_bag)=DinoTree::new::<par::Parallel,DefaultDepthLevel,_,treetimer::TreeTimerEmpty>
                    (&mut bots,&mut treecache,&k);
        }

        //let graphics=BotSysGraphics::new::<TDraw>(&mut bots,treecache.get_height());
        
        let bot_graphics=BotLibGraphics::new(&bot_prop);
        

        /*
        let mut general_log=mlog::MLog::new("/storage/emulated/0/Download/data.csv");

        let mut rebal_log={
            let mut rebal_log=Logger::new("/storage/emulated/0/Download/rebal.csv");
            let a:Vec<String>=(0..height).map(|a|format!("level {}",a)).collect();
            rebal_log.write_str("Iteration",&a);
            rebal_log
        };

        let mut colfind_log={
            let mut query_log=Logger::new("/storage/emulated/0/Download/query.csv");
            let a:Vec<String>=(0..height).map(|a|format!("level {}",a)).collect();
            query_log.write_str("Iteration",&a);
            query_log
        };
        */

      
        BotSystem {
            bot_graphics:bot_graphics,
            //graphics:graphics,
            mouse_prop:mouse_prop,
            bots,
            bot_prop,
            border: world,
            treecache:treecache,
            phantom:PhantomData,
            //general_log,
            //rebal_log,
            //colfind_log
        }
    }
}



fn handle_mouse<K:DynTreeTrait<T=BBot,Num=Numf32>>(prop:&BotProp,tree:&mut K,mouse:&Mouse){
    

    let mut rect=Rects::new(tree);//tree.create_rects();//Rects::new(tree);
    rect.for_all_in_rect(&bot::convert_to_nan(*mouse.get_rect()),
                           &mut |cc:ColSingle<BBot>| {

                        //println!("collide mouse!");
            use botlib::bot::BotMovementTrait;
            Bot::collide_mouse(cc.1,prop,&mouse);

        });
}



