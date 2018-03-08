use axgeom;
use botlib::graphics::BotLibGraphics;

use dinotree;
use dinotree::*;
use dinotree::support::Numf32;

use axgeom::Rect;
use wrap_around::WrapAround;
use botlib::mouse::Mouse;

use std;
use botlib::bot::BotProp;
use botlib::mouse::MouseProp;
use botlib::bot;
use std::marker::PhantomData;
use kenmisc;
use dinotree::support::DefaultDepthLevel;
use axgeom::AxisTrait;
use botlib::bot::BBot;
use botlib::bot::Bot;
use Vert;
use botlib::bot::convert_aabbox;


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







pub trait TreeDraw{
    fn get_num_verticies(height:usize)->usize;
    fn update(rect:&Rect<f32>,tree:&DinoTree<BBot>,verts:&mut [Vert]);
}

pub struct TreeDrawReal{
}
impl TreeDraw for TreeDrawReal{
    fn get_num_verticies(height:usize)->usize{
        dinotree::graphics::get_num_verticies(height)
    }
    fn update(rect:&Rect<f32>,tree:&DinoTree<BBot>,verts:&mut [Vert]){
        
        #[derive(Clone,Default,Copy)]
        struct Bo(Vert);
        impl dinotree::graphics::Vertex for Bo{
            fn set_pos(&mut self,x:f32,y:f32){
                (self.0).0[0]=x;
                (self.0).0[1]=y;
            }
        }

        let k:&mut [Bo]=unsafe{std::mem::transmute(verts)};
        dinotree::graphics::update(bot::convert_to_nan(*rect),tree,k,10.0);
    }
}
pub struct TreeNoDraw{
}
impl TreeDraw for TreeNoDraw{
    fn get_num_verticies(_height:usize)->usize{
        0
    }
    fn update(_rect:&Rect<f32>,_tree:&DinoTree<BBot>,_verts:&mut [Vert]){
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



pub struct BotSystem<TDraw:TreeDraw> {
    bot_graphics:BotLibGraphics,
    mouse_prop:MouseProp,
    bots: Vec<BBot>,
    bot_prop:BotProp,
    border: axgeom::Rect<f32>,
    //treecache:TreeCache2<Numf32>,
    axis:axgeom::Axis,
    phantom:PhantomData<TDraw>,
    logsys:LogSystem
}


pub trait BotSysTrait{
    //fn get_verticies(&self,a:&mut [Vert]);
    fn get_num_verticies(&self)->usize;
    fn step(&mut self, poses: &[axgeom::Vec2],a:&mut [Vert]);
}

impl<TDraw:TreeDraw> BotSysTrait for BotSystem<TDraw>{

    fn get_num_verticies(&self)->usize{
        use dinotree::graphics::compute_tree_height;
        let height = compute_tree_height(self.bots.len());
        TDraw::get_num_verticies(height)+BotLibGraphics::get_num_verticies(self.bots.len())
    }
    /*
    fn get_verticies(&self,verts:&mut [Vert])  {
        let height = compute_tree_height(self.bots.len(), self.bot_prop.radius.radius());
        
        let (a,b)=verts.split_at_mut(TDraw::get_num_verticies(height));
        TDraw::update(&self.border,&self.treecache,a);
        self.bot_graphics.update(&self.bot_prop,&self.bots,b);
    }
    */

    fn step(&mut self, poses: &[axgeom::Vec2],verts:&mut [Vert]) {
        use dinotree::graphics::compute_tree_height;
        let height = compute_tree_height(self.bots.len());
        
        let (tree_verts,bot_verts)=verts.split_at_mut(TDraw::get_num_verticies(height));
        

        let _time_all=kenmisc::Timer2::new();
        let bots=&mut self.bots;
        {                
            let border=&self.border;
            let mouse_prop=&self.mouse_prop;
            let bot_prop=self.bot_prop;
            //let treecache=&mut self.treecache;

            {
                let _rebal=kenmisc::Timer2::new();

                {

                    /*
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
                    */
                    //let bb=MedianStrict::new();                                                                                  //TreeTimer2 ot TreeTimerEmpty
                    //let (mut dyntree,_bag)=treecache.new_tree::<_,par::Parallel,DefaultDepthLevel,_,treetimer::TreeTimer2>
                    //    (bots,&bb);
                    let (mut dyntree,_bag)=DinoTree::new_debug(bots,self.axis==axgeom::XAXIS);

                    self.logsys.rebal_log.write_data(&_bag.into_vec());


                    //the dynamic tree made a copy of the bots.
                    //so here we can still use bo.man.
                    //later will add together the copy and the source.
                    
                    {
                        self.logsys.general_log.write(log::Typ::Rebal,_rebal.elapsed());
                            


                        let query=kenmisc::Timer2::new();


                        let a=|a:ColSingle<BBot>,b:ColSingle<BBot>|{
                            bot::collide(&bot_prop,a,b);
                        };
                        
                        let _v=dyntree.intersect_every_pair_debug(a);
                        
                        /*
                        let a=AABBox::new((Numf32::from_f32(0.0),Numf32::from_f32(100.0)),(Numf32::from_f32(0.0),Numf32::from_f32(100.0)));
                        dyntree.for_all_in_rect(&a,|a:ColSingle<BBot>|{
                            a.1.vel=axgeom::Vec2::new(0.0,0.0);
                        });
                        */

                        self.logsys.colfind_log.write_data(&_v.into_vec());

                        self.logsys.general_log.write(log::Typ::Query,query.elapsed());
                        

                        WrapAround::handle(&mut dyntree,border,bot_prop);   

                        

                        for k in poses{
                            let mouse=Mouse::new(k,mouse_prop);
                            handle_mouse(bot_prop,&mut dyntree,&mouse);
                            WrapAround::handle_mouse(bot_prop,&mut dyntree,border,&mouse);
                        }

                        TDraw::update(&self.border,&dyntree,tree_verts);
                        
                    }

                    
                    self.logsys.general_log.write(log::Typ::RebalQuery,_rebal.elapsed());

                }
        
                {
                    let _upd=kenmisc::Timer2::new();
                    bot::update(bots,bot_prop,border);
                    self.bot_graphics.update(&self.bot_prop,bots,bot_verts);
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

    if draw_tree{
        let k=BotSystem::<TreeDrawReal>::new_inner(num_bots,startx,starty);
        Box::new(k)
    }else{
        let k=BotSystem::<TreeNoDraw>::new_inner(num_bots,startx,starty);
        Box::new(k)
    }
}

impl<TDraw:TreeDraw> BotSystem<TDraw> {

    
    fn new_inner(num_bots:usize,startx:usize,starty:usize) -> BotSystem<TDraw> {
        let world= axgeom::Rect::new(0.0,startx as f32,0.0,starty as f32);
    
        let br=bot::compute_bot_radius(num_bots,&world).unwrap();
        
        let unit=bot::get_unit(startx,starty);
        let (bot_prop,mouse_prop)=bot::create_from_radius(br,unit*10.0);

        let mut bots = bot::create_bots(num_bots,&world,&bot_prop);

        //TODO should it be based on max prop or average prop
        use dinotree::graphics::compute_tree_height;
        let height = compute_tree_height(bots.len());

        let axis=if startx>starty{
            axgeom::XAXIS
        }else{
            axgeom::YAXIS
        };
        //let mut treecache=TreeCache2::new(axis,height);

        
        /*
        {         
            let k=MedianStrict::<Numf32>::new();
            let (_dyntree,_bag)=treecache.new_tree::<_,par::Parallel,DefaultDepthLevel,_,treetimer::TreeTimerEmpty>
                    (&mut bots,&k);
        }
        */
        
        
        let bot_graphics=BotLibGraphics::new(&bot_prop);
        
        let logsys=LogSystem::new(height);
        
        

      
        BotSystem {
            bot_graphics:bot_graphics,
            mouse_prop:mouse_prop,
            bots,
            bot_prop,
            border: world,
            axis,
            //treecache:treecache,
            phantom:PhantomData,
            logsys
        }
    }
}



fn handle_mouse(prop:BotProp,tree:&mut DinoTree<BBot>,mouse:&Mouse){
      
    //let mut rect=Rects::new(tree);
    //let rect=tree.rects();

    tree.rects().for_all_in_rect(&convert_aabbox(bot::convert_to_nan(*mouse.get_rect())),&mut |mut a:ColSingle<BBot>|{
        bot::collide_mouse(&mut a,&prop,mouse);
    });

}



