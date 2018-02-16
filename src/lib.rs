extern crate axgeom;
//extern crate simpdraw;
extern crate kenmisc;
extern crate ordered_float;
//extern crate testy;
extern crate ascii_num;
extern crate dinotree;

mod botlib;
mod sys;
mod menusys;
mod wrap_around;
mod menu_primitives;


pub use menusys::MenuGame;

#[derive(Copy,Clone,Debug,Default)]
pub struct Vert(pub [f32;2]);


mod mlog{
    
    use sys;
    use kenmisc::log;
    pub struct MLog{
        logger:log::Logger,
        arr:[f64;7]
    }

    impl MLog{
        pub fn new(str:&'static str)->MLog{
            MLog{logger:log::Logger::with_names(str,&["rebalance","query","rebal_query","bot update","cont dyn","graphic","total"]),arr:[0.0;7]}
        }
    }

    impl sys::log::LogT for MLog{
        
        fn write(&mut self,e:sys::log::Typ,val:f64){
            self.arr[e as usize]=val;
        }

        fn newline(&mut self){
            self.logger.write_data(&self.arr);
        }
    }


    pub struct MLogDummy{
    }

    impl sys::log::LogT for MLogDummy{
        
        fn write(&mut self,_e:sys::log::Typ,_val:f64){
        }

        fn newline(&mut self){
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
