use axgeom;
use botlib::bot::BotTrait;
use botlib::bot::BotProp;

//use simpdraw::Vertex;
use botlib::bot::BBot;
use botlib::bot::Bot;
use Vert;

pub struct BotLibGraphics{
    p1:axgeom::Vec2,
    p2:axgeom::Vec2,
    p3:axgeom::Vec2
}


impl BotLibGraphics{
    pub fn new(prop:&BotProp)->BotLibGraphics{
        let r = prop.radius.radius();
        //let z = prop.radius.radius_x_root_2_inv();    

        //let p1 = axgeom::Vec2::new(r, 0.0);
        //let p2 = axgeom::Vec2::new(-z, -z);
        //let p3 = axgeom::Vec2::new(-z, z);

        let mut p1=axgeom::Vec2::new(3.0,2.0);
        p1*=r/p1.len();

        let mut p2=axgeom::Vec2::new(-3.0,2.0);
        p2*=r/p2.len();

        let mut p3=axgeom::Vec2::new(0.0,-1.0);
        p3*=r/p3.len();

        BotLibGraphics{p1:p1,p2:p2,p3:p3}
    }
    pub fn get_num_verticies(num_bot:usize)->usize{
        num_bot*3
    }

    pub fn update(&self,prop:&BotProp,bots:&[BBot],verticies:&mut [Vert]){
        assert!(Self::get_num_verticies(bots.len())<=verticies.len());
		for (a,b) in bots.iter().enumerate()
		{	
            self.update_triangles(prop,a, &b.val,verticies);
        }
    }

    fn update_triangles(&self,_prop:&BotProp,bot_ind: usize, bot: &Bot, verticies: &mut [Vert]) {

        let i = bot_ind as usize * 3;

        /*
        let vel_len=bot.vel().len();
        //let velnorm=bot.vel_norm();
        let velnorm=if vel_len > 0.0001 {
            //todo optimize
            *bot.vel() / vel_len
        } else {
            axgeom::Vec2::new(1.0, 0.0)
        };
        */
                
        
        let p1 = *bot.pos() + self.p1;//.rotate_by(velnorm);
        let p2 = *bot.pos() + self.p2;//.rotate_by(velnorm);
        let p3 = *bot.pos() + self.p3;//.rotate_by(velnorm);

        //let hue_multiplier=vel_len/1.0;//TODO whatever the max vel is
        //let acc_mag=bot.get_acc().len()*0.25;
        //let vel_mag=1.0;//vel_len*0.25;
        //let hue_multiplier=1.0;
        //let alpha_multiplier = vel_len / bot_prop.max_vel.val() * 10.0;
        //let alpha_multiplier=0.2+vel_mag+acc_mag;

        //let sat_multiplier=vel_len / bot_prop.max_vel.val() * 10.0;
        //let sat_multiplier=1.0;
        let p1=p1.get();
        let p2=p2.get();
        let p3=p3.get();
        unsafe {
            *verticies.get_unchecked_mut(i+0)=Vert([*p1.0,*p1.1]);
            *verticies.get_unchecked_mut(i+1)=Vert([*p2.0,*p2.1]);
            *verticies.get_unchecked_mut(i+2)=Vert([*p3.0,*p3.1]);
            

            /*
            verticies.get_unchecked_mut(i + 0).set_pos(*p1.0,*p1.1);
            verticies.get_unchecked_mut(i + 1).set_pos(*p2.0,*p2.1);
            verticies.get_unchecked_mut(i + 2).set_pos(*p3.0,*p3.1);
            */
            //change alpha
            /*
            verticies.get_unchecked_mut(i + 0).set_alpha(alpha_multiplier);
            verticies.get_unchecked_mut(i + 1).set_alpha(alpha_multiplier);
            verticies.get_unchecked_mut(i + 2).set_alpha(alpha_multiplier);
            */
        }
    }
}
