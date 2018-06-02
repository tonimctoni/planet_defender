use constants::{SCREEN_WIDTH_F64, SCREEN_HEIGHT_F64};
// use constants::{SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH_F64, SCREEN_HEIGHT_F64};


// fn keep_in_screen_horizontally(pos: &mut (f64, f64), width: f64){
//     if pos.0<0.{
//         pos.0=0.;
//     }

//     if pos.0+width>SCREEN_WIDTH_F64-1.{
//         pos.0=SCREEN_WIDTH_F64-width-1.;
//     }
// }

// fn keep_in_screen_vertically(pos: &mut (f64, f64), height: f64){
//     if pos.1<0.{
//         pos.1=0.;
//     }

//     if pos.1+height>SCREEN_HEIGHT_F64-1.{
//         pos.1=SCREEN_HEIGHT_F64-height-1.;
//     }
// }

// fn keep_in_screen(pos: &mut (f64, f64), width: f64, height: f64){
//     keep_in_screen_horizontally(pos, width);
//     keep_in_screen_vertically(pos, height);
// }

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

    fn is_within_screen(&self) -> bool{
        let pos=self.get_pos();
        let dims=self.get_dims();

        if pos.0+dims.0<1. { // because it gets rounded down for drawing on screen
            false
        } else if pos.0>=SCREEN_WIDTH_F64 {
            false
        } else if pos.1+dims.1<1. {
            false
        } else if pos.1>=SCREEN_HEIGHT_F64 {
            false
        } else {
            true
        }
    }

    fn is_colliding_with_circle<T: Actor>(&self, other: &T) -> bool{
    // fn is_colliding_with_circle(&self, other: &Actor) -> bool{
        let (self_center_x, self_center_y)=self.get_center();
        let (other_center_x, other_center_y)=other.get_center();

        let x_distance=self_center_x-other_center_x;//may be negative
        let y_distance=self_center_y-other_center_y;//may be negative
        let added_radii=self.get_radius()+other.get_radius();

        let distance_squared=x_distance*x_distance+y_distance*y_distance;
        let darii_squared=added_radii*added_radii;

        distance_squared < darii_squared
    }

    fn set_pos(&mut self, x: f64, y: f64){
        let pos=self.get_mut_pos();

        pos.0=x;
        pos.1=y;
    }

    fn set_center(&mut self, x: f64, y: f64){
        let (width,height)=self.get_dims();
        let pos=self.get_mut_pos();

        pos.0=x-width/2.;
        pos.1=y-height/2.;
    }

    fn set_x_center(&mut self, x: f64){
        let (width,_)=self.get_dims();
        let pos=self.get_mut_pos();

        pos.0=x-width/2.;
    }

    fn set_y_center(&mut self, y: f64){
        let (_,height)=self.get_dims();
        let pos=self.get_mut_pos();

        pos.1=y-height/2.;
    }

    fn move_by_speed(&mut self){
        let (dx,dy)=self.get_speed();
        let pos=self.get_mut_pos();

        pos.0+=dx;
        pos.1+=dy;
    }

    fn move_horizontally_by_speed(&mut self){
        let (dx,_)=self.get_speed();
        let pos=self.get_mut_pos();

        pos.0+=dx;
    }

    fn move_horizontally_by_minus_speed(&mut self){
        let (dx,_)=self.get_speed();
        let pos=self.get_mut_pos();

        pos.0-=dx;
    }

    fn move_vertically_by_speed(&mut self){
        let (_,dy)=self.get_speed();
        let pos=self.get_mut_pos();

        pos.1+=dy;
    }
}