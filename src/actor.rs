use constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
const SCREEN_WIDTH_F64: f64 = SCREEN_WIDTH as f64;
const SCREEN_HEIGHT_F64: f64 = SCREEN_HEIGHT as f64;


fn keep_in_screen(pos: &mut (f64, f64), width: f64, height: f64){
    if pos.0<0.{
        pos.0=0.;
    }

    if pos.1<0.{
        pos.1=0.;
    }

    if pos.0+width>SCREEN_WIDTH_F64-1.{
        pos.0=SCREEN_WIDTH_F64-width-1.;
    }

    if pos.1+width>SCREEN_HEIGHT_F64-1.{
        pos.1=SCREEN_HEIGHT_F64-height-1.;
    }
}

fn keep_in_screen_get_screen_collision(pos: &mut (f64, f64), width: f64, height: f64) -> bool{
    let mut collided=false;

    if pos.0<0.{
        pos.0=0.;
        collided=true;
    }

    if pos.1<0.{
        pos.1=0.;
        collided=true;
    }

    if pos.0+width>SCREEN_WIDTH_F64-1.{
        pos.0=SCREEN_WIDTH_F64-width-1.;
        collided=true;
    }

    if pos.1+width>SCREEN_HEIGHT_F64-1.{
        pos.1=SCREEN_HEIGHT_F64-height-1.;
        collided=true;
    }

    collided
}

pub trait Actor {
    fn get_mut_pos(&mut self) -> &mut (f64,f64);
    fn get_dims(&self) -> (f64, f64);

    fn get_speed(&self) -> (f64, f64){
        (0.,0.)
    }

    fn set_center(&mut self, x: f64, y: f64){
        let (width,height)=self.get_dims();
        let pos=self.get_mut_pos();

        pos.0=x-width/2.;
        pos.1=y-height/2.;

        keep_in_screen(pos, width, height);
    }

    // fn move_by(&mut self, dx: f64, dy: f64) -> bool{
    //     let (width,height)=self.get_dims();
    //     let pos=self.get_mut_pos();

    //     pos.0+=dx;
    //     pos.1+=dy;

    //     keep_in_screen_get_screen_collision(pos, width, height)
    // }

    fn move_by_speed(&mut self) -> bool{
        let (width,height)=self.get_dims();
        let (dx,dy)=self.get_speed();
        let pos=self.get_mut_pos();

        pos.0+=dx;
        pos.1+=dy;

        keep_in_screen_get_screen_collision(pos, width, height)
    }

    fn move_by_minus_speed(&mut self) -> bool{
        let (width,height)=self.get_dims();
        let (dx,dy)=self.get_speed();
        let pos=self.get_mut_pos();

        pos.0-=dx;
        pos.1-=dy;

        keep_in_screen_get_screen_collision(pos, width, height)
    }

}