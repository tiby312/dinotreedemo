use axgeom;
use botlib::graphics::BotLibGraphics;
use dinotree::graphics::GenTreeGraphics;
use dinotree::DinoTree;
use dinotree::multirect::Rects;
use dinotree::*;
use dinotree::tools::par;
use dinotree::median::*;
use dinotree::median::relax::*;
use dinotree::median::strict::*;
use dinotree::support::Numf32;
use dinotree;
use dinotree::DynTreeTrait;
use axgeom::Rect;
use wrap_around::WrapAround;
use botlib::mouse::Mouse;
//use botlib::bot::Bot;

//use simpdraw;
use std;
use botlib::bot::BotProp;
use botlib::mouse::MouseProp;
use botlib::bot;
use std::marker::PhantomData;
use kenmisc;
use dinotree::support::DefaultDepthLevel;
use axgeom::AxisTrait;
use botlib::bot::BBot;

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



use sys::log::LogT;
use kenmisc::log::Logger;
use mlog;
pub struct LogSystem{
    pub general_log:mlog::MLog,
    pub rebal_log:Logger,
    pub colfind_log:Logger
}
impl LogSystem{
    pub fn new(height:usize)->LogSystem{

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

        LogSystem{general_log,rebal_log,colfind_log}
    }
}



pub struct BotSystem<A:AxisTrait,TDraw:TreeDraw> {
    bot_graphics:BotLibGraphics,
    mouse_prop:MouseProp,
    bots: Vec<BBot>,
    bot_prop:BotProp,
    border: axgeom::Rect<f32>,
    treecache:TreeCache<A,Numf32>,
    phantom:PhantomData<TDraw>,
    logsys:LogSystem
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

        let _time_all=kenmisc::Timer2::new();
        let bots=&mut self.bots;
        {                
            let border=&self.border;
            let mouse_prop=&self.mouse_prop;
            let bot_prop=&self.bot_prop;
            let treecache=&mut self.treecache;

            {
                let _rebal=kenmisc::Timer2::new();

                {
                    struct B(
                        f32
                    );
                    impl DivMoveStrat for B{
                        type N=Numf32;
                        fn move_divider(&self,a:&mut Self::N,total:usize,b:f32){
                            
                            let total=total as f32;

                            //add a little more urgency to dividers near the root.
                            let total=total*0.003+self.0;
                            a.0+=b*total;
                        }
                    }
                    
                    let bb=MedianRelax::new(B(self.bot_prop.radius.radius()));
                    
                    let (mut dyntree,_bag)=DinoTree::new::<par::Parallel,DefaultDepthLevel,_,treetimer::TreeTimer2>
                        (bots,treecache,&bb);
                    
                    self.logsys.rebal_log.write_data(&_bag.into_vec());


                    //the dynamic tree made a copy of the bots.
                    //so here we can still use bo.man.
                    //later will add together the copy and the source.
                    
                    {
                        self.logsys.general_log.write(log::Typ::Rebal,_rebal.elapsed());
                            


                        let query=kenmisc::Timer2::new();

                        let clos=|cc:ColPair<BBot>|{
                            //use botlib::bot::BotMovementTrait;

                            bot::collide(bot_prop,cc);
                        };

                        let _v=dyntree.for_every_col_pair::<DefaultDepthLevel,_,treetimer::TreeTimer2>(clos);
                        self.logsys.colfind_log.write_data(&_v.into_vec());
            
                        
                        self.logsys.general_log.write(log::Typ::Query,query.elapsed());
                        

                        WrapAround::handle(&mut dyntree,border,bot_prop);   

                        

                        for k in poses{
                            let mouse=Mouse::new(k,mouse_prop);
                            handle_mouse(bot_prop,&mut dyntree,&mouse);
                            WrapAround::handle_mouse(bot_prop,&mut dyntree,border,&mouse);
                        }
                        
                    }

                    
                    self.logsys.general_log.write(log::Typ::RebalQuery,_rebal.elapsed());

                }
        
                {
                    let _upd=kenmisc::Timer2::new();
                    bot::update(bots,bot_prop,border);
                    self.logsys.general_log.write(log::Typ::BotUpdate,_upd.elapsed());
                }
            
            
                self.logsys.general_log.write(log::Typ::Total,_time_all.elapsed());
                self.logsys.general_log.newline();
            }
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

        //TODO should it be based on max prop or average prop
        let height = compute_tree_height(bots.len(), bot_prop.radius.radius());

        let mut treecache=TreeCache::new(height);

        {         
            let k=MedianStrict::<Numf32>::new();
            let (_dyntree,_bag)=DinoTree::new::<par::Parallel,DefaultDepthLevel,_,treetimer::TreeTimerEmpty>
                    (&mut bots,&mut treecache,&k);
        }
        
        let bot_graphics=BotLibGraphics::new(&bot_prop);
        
        let logsys=LogSystem::new(height);
        
        

      
        BotSystem {
            bot_graphics:bot_graphics,
            mouse_prop:mouse_prop,
            bots,
            bot_prop,
            border: world,
            treecache:treecache,
            phantom:PhantomData,
            logsys
        }
    }
}



fn handle_mouse<K:DynTreeTrait<T=BBot,Num=Numf32>>(prop:&BotProp,tree:&mut K,mouse:&Mouse){
    

    let mut rect=Rects::new(tree);//tree.create_rects();//Rects::new(tree);
    rect.for_all_in_rect(&bot::convert_to_nan(*mouse.get_rect()),
                           &mut |mut cc:ColSingle<BBot>| {

                        //println!("collide mouse!");
            //use botlib::bot::BotMovementTrait;
            bot::collide_mouse(&mut cc,prop,&mouse);

        });
}



