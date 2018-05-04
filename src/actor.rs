use constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
const SCREEN_WIDTH_F64: f64 = SCREEN_WIDTH as f64;
const SCREEN_HEIGHT_F64: f64 = SCREEN_HEIGHT as f64;


fn keep_in_screen_horizontally(pos: &mut (f64, f64), width: f64){
    if pos.0<0.{
        pos.0=0.;
    }

    if pos.0+width>SCREEN_WIDTH_F64-1.{
        pos.0=SCREEN_WIDTH_F64-width-1.;
    }
}

fn keep_in_screen_vertically(pos: &mut (f64, f64), height: f64){
    if pos.1<0.{
        pos.1=0.;
    }

    if pos.1+height>SCREEN_HEIGHT_F64-1.{
        pos.1=SCREEN_HEIGHT_F64-height-1.;
    }
}

fn keep_in_screen(pos: &mut (f64, f64), width: f64, height: f64){
    keep_in_screen_horizontally(pos, width);
    keep_in_screen_vertically(pos, height);
}

// fn keep_in_screen_get_screen_collision(pos: &mut (f64, f64), width: f64, height: f64) -> bool{
//     let mut collided=false;

//     if pos.0<0.{
//         pos.0=0.;
//         collided=true;
//     }

//     if pos.1<0.{
//         pos.1=0.;
//         collided=true;
//     }

//     if pos.0+width>SCREEN_WIDTH_F64-1.{
//         pos.0=SCREEN_WIDTH_F64-width-1.;
//         collided=true;
//     }

//     if pos.1+width>SCREEN_HEIGHT_F64-1.{
//         pos.1=SCREEN_HEIGHT_F64-height-1.;
//         collided=true;
//     }

//     collided
// }

pub trait Actor {
    fn get_mut_pos(&mut self) -> &mut (f64,f64);
    fn get_pos(&self) -> (f64,f64);
    fn get_dims(&self) -> (f64, f64);

    fn get_speed(&self) -> (f64, f64){
        (0.,0.)
    }

    fn get_radius(&self) -> f64{
        let (width,height)=self.get_dims();
        (width+height)/4.
    }

    fn get_x(&self) -> f64{
        self.get_pos().0
    }

    fn get_y(&self) -> f64{
        self.get_pos().1
    }

    fn get_x_center(&self) -> f64{
        self.get_pos().0+self.get_dims().0/2.
    }

    fn get_y_center(&self) -> f64{
        self.get_pos().1+self.get_dims().1/2.
    }

    fn get_center(&self) -> (f64,f64){
        let pos=self.get_pos();
        let dims=self.get_dims();
        (pos.0+dims.0/2., pos.1+dims.1/2.)
    }

    fn set_center(&mut self, x: f64, y: f64){
        let (width,height)=self.get_dims();
        let pos=self.get_mut_pos();

        pos.0=x-width/2.;
        pos.1=y-height/2.;

        keep_in_screen(pos, width, height);
    }

    fn set_x_center(&mut self, x: f64){
        let (width,_)=self.get_dims();
        let pos=self.get_mut_pos();

        pos.0=x-width/2.;

        keep_in_screen_horizontally(pos, width);
    }

    fn set_y_center(&mut self, y: f64){
        let (_,height)=self.get_dims();
        let pos=self.get_mut_pos();

        pos.1=y-height/2.;

        keep_in_screen_vertically(pos, height);
    }

    fn move_by_speed(&mut self){
        let (width,height)=self.get_dims();
        let (dx,dy)=self.get_speed();
        let pos=self.get_mut_pos();

        pos.0+=dx;
        pos.1+=dy;

        keep_in_screen(pos, width, height)
    }

    fn move_horizontally_by_speed(&mut self){
        let (width,_)=self.get_dims();
        let (dx,_)=self.get_speed();
        let pos=self.get_mut_pos();

        pos.0+=dx;

        keep_in_screen_horizontally(pos, width)
    }

    fn move_horizontally_by_minus_speed(&mut self){
        let (width,_)=self.get_dims();
        let (dx,_)=self.get_speed();
        let pos=self.get_mut_pos();

        pos.0-=dx;

        keep_in_screen_horizontally(pos, width)
    }

}