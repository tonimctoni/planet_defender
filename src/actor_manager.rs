// use std;
use std::f64::consts::PI;
use sdl2;
use satellite::Satellite;
use projectile::Projectile;
use meteor::Meteor;
use actor::Actor;
// use constants::{SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH_F64, SHIP_EPSILON};
use constants::{SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH_F64, SCREEN_HEIGHT_F64, SHIP_EPSILON};
use textures::Textures;
use rand::{XorShiftRng, SeedableRng, Rng};
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;


pub struct ActorManager {
    satellite: Satellite,
    projectiles: Vec<Projectile>,
    meteors: Vec<Meteor>,
    rng: XorShiftRng,
}


impl ActorManager {
    pub fn init(textures: &Textures) -> ActorManager{
        let mut satellite=Satellite::new(&textures);
        satellite.set_center((SCREEN_WIDTH/2) as f64, SCREEN_HEIGHT as f64 - 160.);
        ActorManager{satellite: satellite, projectiles: Vec::with_capacity(64), meteors: Vec::with_capacity(64), rng: XorShiftRng::from_seed([100,2,3,4])}
    }

    pub fn shoot_projectile(&mut self, mouse_pos: (f64, f64), textures: &Textures){ // Add another parameter with type of projectile when more are available
        let satellite_pos=self.satellite.get_center();
        if satellite_pos.1 > mouse_pos.1{
            let mut projectile=Projectile::new(&textures);
            let speed=(mouse_pos.0-satellite_pos.0,mouse_pos.1-satellite_pos.1);
            let speed_speed=(speed.0*speed.0+speed.1*speed.1).sqrt();
            let speed=(speed.0/speed_speed, speed.1/speed_speed);
            projectile.set_center(satellite_pos.0+speed.0*self.satellite.get_radius(), satellite_pos.1+speed.1*self.satellite.get_radius());
            projectile.set_speed(speed.0*5., speed.1*5.);
            self.projectiles.push(projectile);
        }
    }

    pub fn drop_meteor(&mut self, textures: &Textures){ // Add another parameter with type of meteor when more are available // Make sure it does not collide
        let mut meteor=Meteor::new(textures);
        let meteor_width=meteor.get_dims().0;
        meteor.set_pos(self.rng.next_f64()*(SCREEN_WIDTH_F64-meteor_width), 0.);
        meteor.set_speed(2.5);
        meteor.set_angle(self.rng.next_f64()*360.);
        self.meteors.push(meteor);
    }

    pub fn step(&mut self, mouse_pos: (f64, f64)){
        // Move satellite horizontally toward mouse pointer
        let satellite_direction=mouse_pos.0-self.satellite.get_x_center();
        if satellite_direction.abs()>SHIP_EPSILON{
            if satellite_direction > 0.{
                self.satellite.move_horizontally_by_speed();
            } else if satellite_direction < 0.{
                self.satellite.move_horizontally_by_minus_speed();
            }
        }

        // Rotate satellite so it points toward mouse pointer
        let satellite_pos=self.satellite.get_center();
        let gradient=-(mouse_pos.0 - satellite_pos.0)/(mouse_pos.1 - satellite_pos.1);
        let angle=gradient.atan()*180./PI;
        self.satellite.set_angle(angle);

        // For now, when a meteor and a projectile collide, the destroy each other
        for projectile in self.projectiles.iter_mut(){
            for meteor in self.meteors.iter_mut(){
                if projectile.is_colliding_with_circle(meteor){
                    projectile.set_pos(SCREEN_WIDTH_F64+10000., SCREEN_HEIGHT_F64+10000.);
                    meteor.set_pos(SCREEN_WIDTH_F64+10000., SCREEN_HEIGHT_F64+10000.);
                }
            }
        }

        // Move each projectile according to speed
        for projectile in self.projectiles.iter_mut(){
            projectile.move_by_speed();
        }

        // Move each meteor according to speed
        for meteor in self.meteors.iter_mut(){
            meteor.move_by_speed();
        }

        // Drop projectiles that flew outside the screen
        self.projectiles.retain(|p| p.is_within_screen());

        // Drop meteors that flew outside the screen
        self.meteors.retain(|m| m.is_within_screen());
    }

    pub fn draw(&self, canvas: &mut Canvas, textures: &Textures){
        self.satellite.draw(canvas, textures);
        self.projectiles.iter().for_each(|projectile| projectile.draw(canvas, textures));
        self.meteors.iter().for_each(|meteor| meteor.draw(canvas, textures));
    }
}