use axgeom;
use botlib::graphics::BotLibGraphics;
use dinotree_alg;
use axgeom::Rect;
use wrap_around::WrapAround;
use botlib::mouse::Mouse;
use std;
use botlib::bot::BotProp;
use botlib::mouse::MouseProp;
use botlib::bot;
use std::marker::PhantomData;
use kenmisc;
use dinotree_inner::BBox;
use Vert;
use compt;
use vec::Vec2;
use dinotree_inner;

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


pub type Tree=dinotree_inner::DynTree<axgeom::YAXISS,(),dinotree_inner::BBox<NotNaN<f32>,Bot>>;

use botlib::bot::Bot;
use ordered_float::*;
pub trait TreeDraw{
    fn get_num_verticies(height:usize)->usize;
    fn update(rect:&Rect<NotNaN<f32>>,tree:&Tree,verts:&mut [Vert]);
}

pub struct TreeDrawReal{
}
impl TreeDraw for TreeDrawReal{
    fn get_num_verticies(height:usize)->usize{
        let num_nodes=compt::compute_num_nodes(height);
        (num_nodes / 2) * 6

        //dinotree::graphics::get_num_verticies(height)
    }
    fn update(rect:&Rect<NotNaN<f32>>,tree:&Tree,verts:&mut [Vert]){
        /*
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
        */
        let height=tree.get_height();
        let width=1 as f32;
        dinotree_alg::graphics::draw(tree,&mut Bo{verts,height,width},rect);

        struct Bo<'a>{
            verts:&'a mut [Vert],
            height:usize,
            width:f32
        };

        impl<'a> dinotree_alg::graphics::DividerDrawer for Bo<'a>{
            type N=NotNaN<f32>;

            fn draw_divider<A:axgeom::AxisTrait>(&mut self,axis:A,div:Self::N,_cont:[Self::N;2],length:[Self::N;2],depth:usize){
                  let height=self.height;
                  let width=self.width;

                  let verts=std::mem::replace(&mut self.verts,&mut []);
                  //let verts=self.verts;

                  let div=div.into_inner();
                  let length=[length[0].into_inner(),length[1].into_inner()];
                  let width = (((height - depth) + 1) as f32) / (height as f32) * width;

                  let vv=if axis.is_xaxis(){
                    (Vec2::new(div,length[0]),Vec2::new(div,length[1]))
                  }else{
                    (Vec2::new(length[0],div),Vec2::new(length[1],div))
                  };

                  let (f1,f2)=verts.split_at_mut(6);
                  std::mem::replace(&mut self.verts,f2);

                  use std::convert::TryInto;
                  fn foo(x: &mut [Vert]) -> &mut [Vert;6] { x.try_into().unwrap() }

                  draw_line(foo(f1),&vv.0,&vv.1,width);
            }
        }

        fn draw_line(verticies: &mut [Vert;6], p1: &Vec2, p2: &Vec2, width: f32) {
            //debug_assert!(verticies.len() == 6);

            let (p1, p2) = (*p1, *p2);

            let offset = p2 - p1;
            let len_sqr = offset.len_sqr();
            let norm = if len_sqr > 0.0001 {
                offset / len_sqr.sqrt()
            } else {
                Vec2::new(1.0, 0.0)
            };

            let norm90 = norm.rotate90();

            let xxx = norm90 * width;
            let yyy = norm90 * -width;
            let topleft = p1 + xxx;
            let topright = p1 + yyy;
            let bottomleft = p2 + xxx;
            let bottomright = p2 + yyy;

            
            let topleft = topleft.0;
            let topright = topright.0;
            let bottomleft = bottomleft.0;
            let bottomright = bottomright.0;
            /*
            let topleft=[*topleft.0,*topleft.1];
            let topright=[*topright.0,*topright.1];
            let bottomleft=[*bottomleft.0,*bottomleft.1];
            let bottomright=[*bottomright.0,*bottomright.1];
            */

                verticies[0].0=topleft;
                

                verticies[1].0=topright;

                verticies[2].0=bottomleft;

                verticies[3].0=bottomright;

                verticies[4].0=bottomleft;

                verticies[5].0=topright;
                /*
                verticies
                    .get_unchecked_mut(0)
                    .set_pos(*topleft.0, *topleft.1);
                verticies
                    .get_unchecked_mut(1)
                    .set_pos(*topright.0, *topright.1);
                verticies
                    .get_unchecked_mut(2)
                    .set_pos(*bottomleft.0, *bottomleft.1);
                verticies
                    .get_unchecked_mut(3)
                    .set_pos(*bottomright.0, *bottomright.1);
                verticies
                    .get_unchecked_mut(4)
                    .set_pos(*bottomleft.0, *bottomleft.1);
                verticies
                    .get_unchecked_mut(5)
                    .set_pos(*topright.0, *topright.1);
                */
            
        }

    }
}
pub struct TreeNoDraw{
}
impl TreeDraw for TreeNoDraw{
    fn get_num_verticies(_height:usize)->usize{
        0
    }
    fn update(_rect:&Rect<NotNaN<f32>>,_tree:&Tree,_verts:&mut [Vert]){
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

        let general_log=mlog::MLog::new("/storage/emulated/0/Download/data.csv");

        let rebal_log={
            let mut rebal_log=Logger::new("/storage/emulated/0/Download/rebal.csv");
            let a:Vec<String>=(0..height).map(|a|format!("level {}",a)).collect();
            rebal_log.write_str("Iteration",&a);
            rebal_log
        };

        let colfind_log={
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
    bots: Vec<Bot>,
    bot_prop:BotProp,
    border: axgeom::Rect<NotNaN<f32>>,
    phantom:PhantomData<TDraw>,
    logsys:LogSystem
}


pub trait BotSysTrait{
    fn get_num_verticies(&self)->usize;
    fn step(&mut self, poses: &[Vec2],a:&mut [Vert]);
}

impl<TDraw:TreeDraw> BotSysTrait for BotSystem<TDraw>{

    fn get_num_verticies(&self)->usize{
        let height = dinotree_inner::compute_tree_height_heuristic(self.bots.len());
        TDraw::get_num_verticies(height)+BotLibGraphics::get_num_verticies(self.bots.len())
    }

    fn step(&mut self, poses: &[Vec2],verts:&mut [Vert]) {
        //println!("stepping");
        let height = dinotree_inner::compute_tree_height_heuristic(self.bots.len());
        
        let (tree_verts,bot_verts)=verts.split_at_mut(TDraw::get_num_verticies(height));
        

        let _time_all=kenmisc::Timer2::new();
        let bots=&mut self.bots;
        {                
            let border=&self.border;
            let mouse_prop=&self.mouse_prop;
            let bot_prop=self.bot_prop;

            {
                let _rebal=kenmisc::Timer2::new();

                {

                    let (mut dyntree,_bag)=dinotree_inner::DynTree::with_debug(axgeom::YAXISS,(),&bots,|bot|{
                        bot.create_bbox(bot_prop.radius.radius())
                    },dinotree_inner::DefaultHeightHeur);


                    //println!("tree health={:?}",dyntree.compute_tree_health());
                    self.logsys.rebal_log.write_data(_bag);


                    //the dynamic tree made a copy of the bots.
                    //so here we can still use bo.man.
                    //later will add together the copy and the source.
                    
                    {
                        self.logsys.general_log.write(log::Typ::Rebal,_rebal.elapsed());
                            
                        let query=kenmisc::Timer2::new();
                        
                        let _v=dinotree_alg::colfind::query_debug_mut(&mut dyntree,|a,b|{
                            bot::collide(&bot_prop,&mut a.inner,&mut b.inner);
                        });

                        self.logsys.colfind_log.write_data(_v);

                        self.logsys.general_log.write(log::Typ::Query,query.elapsed());
                        

                        WrapAround::handle(&mut dyntree,border,bot_prop);   

                        

                        for k in poses{
                            let mouse=Mouse::new(k,mouse_prop);
                             
                            let _ = dinotree_alg::multirect::multi_rect_mut(&mut dyntree).for_all_in_rect_mut(bot::convert_to_nan(*mouse.get_rect()),&mut |a:&mut BBox<NotNaN<f32>,Bot>|{
                                bot::collide_mouse(&mut a.inner,&bot_prop,&mouse);
                            });
                            WrapAround::handle_mouse(bot_prop,&mut dyntree,border,&mouse);
                        }


                        TDraw::update(&self.border,&dyntree,tree_verts);
                        
                    }

                    
                    self.logsys.general_log.write(log::Typ::RebalQuery,_rebal.elapsed());
                    dyntree.apply_orig_order(bots,|b,t|*t=b.inner);

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
        let world= axgeom::Rect::new(NotNaN::new(0.0).unwrap(),NotNaN::new(startx as f32).unwrap(),NotNaN::new(0.0).unwrap(),NotNaN::new(starty as f32).unwrap());
    
        let br=bot::compute_bot_radius(num_bots,&world).unwrap();
        
        let unit=bot::get_unit(startx,starty);
        let (bot_prop,mouse_prop)=bot::create_from_radius(br,unit*10.0);

        let bots = bot::create_bots(num_bots,&world,&bot_prop);

        let height = dinotree_inner::compute_tree_height_heuristic(bots.len());

        let bot_graphics=BotLibGraphics::new(&bot_prop);
        
        let logsys=LogSystem::new(height);
        
        BotSystem {
            bot_graphics:bot_graphics,
            mouse_prop:mouse_prop,
            bots,
            bot_prop,
            border: world,
            phantom:PhantomData,
            logsys
        }
    }
}
