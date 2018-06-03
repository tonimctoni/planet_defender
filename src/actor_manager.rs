// use std;
use std::f64::consts::PI;
use sdl2;
use satellite::Satellite;
use projectile::Projectile;
use meteor::Meteor;
use actor::Actor;
use constants::ProjectileKind;
use textures::Textures;
use rand::{XorShiftRng, SeedableRng, Rng};
use constants::{SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH_F64, SCREEN_HEIGHT_F64, SHIP_EPSILON};
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;


pub struct ActorManager {
    satellite: Satellite,
    projectiles: Vec<Projectile>,
    meteors: Vec<Meteor>,
    rng: XorShiftRng,
    triple_shot: bool,
}


impl ActorManager {
    pub fn init(textures: &Textures) -> ActorManager{
        let mut satellite=Satellite::new(&textures);
        satellite.set_center((SCREEN_WIDTH/2) as f64, SCREEN_HEIGHT as f64 - 160.);
        ActorManager{
            satellite: satellite,
            projectiles: Vec::with_capacity(64),
            meteors: Vec::with_capacity(64),
            rng: XorShiftRng::from_seed([100,2,3,4]),
            triple_shot: false
        }
    }

    pub fn set_triple_shot(&mut self, triple_shot: bool){
        self.triple_shot=triple_shot;
    }

    pub fn shoot_projectile(&mut self, mouse_pos: (f64, f64), textures: &Textures, projectile_kind: ProjectileKind){
        let satellite_pos=self.satellite.get_center();
        if satellite_pos.1 > mouse_pos.1{
            let velocity=(mouse_pos.0-satellite_pos.0,mouse_pos.1-satellite_pos.1);
            let magnitude=(velocity.0*velocity.0+velocity.1*velocity.1).sqrt();
            let velocity=(velocity.0/magnitude, velocity.1/magnitude);


            let mut projectile=Projectile::new(&textures, projectile_kind, velocity);
            projectile.set_center(satellite_pos.0+velocity.0*self.satellite.get_radius(), satellite_pos.1+velocity.1*self.satellite.get_radius());
            self.projectiles.push(projectile);

            if self.triple_shot{
                for r in [
                ((0.98480775301220802, -0.17364817766693033), (0.17364817766693033, 0.98480775301220802)),
                ((0.98480775301220802, 0.17364817766693033), (-0.17364817766693033, 0.98480775301220802))
                ].iter(){
                    let velocity=(velocity.0*(r.0).0+velocity.1*(r.0).1, velocity.0*(r.1).0+velocity.1*(r.1).1);
                    let mut projectile=Projectile::new(&textures, projectile_kind, velocity);
                    projectile.set_center(satellite_pos.0+velocity.0*self.satellite.get_radius(), satellite_pos.1+velocity.1*self.satellite.get_radius());
                    self.projectiles.push(projectile);
                }
            }
        }
    }

    pub fn drop_meteor(&mut self, textures: &Textures){ // Add another parameter with type of meteor when more are available // Make sure it does not collide
        let mut meteor=Meteor::new(textures);
        let meteor_width=meteor.get_dims().0;
        meteor.set_pos(self.rng.next_f64()*(SCREEN_WIDTH_F64-meteor_width), 0.);
        for o_meteor in self.meteors.iter(){
            if o_meteor.is_colliding_with_circle(&meteor){
                return
            }
        }
        meteor.set_speed(1.);
        meteor.set_angle(self.rng.next_f64()*360.);
        self.meteors.push(meteor);
    }

    pub fn step(&mut self, mouse_pos: (f64, f64), textures: &Textures){
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
        let mut pellets: Vec<Projectile>=Vec::new();
        for projectile in self.projectiles.iter_mut(){
            for meteor in self.meteors.iter_mut(){
                if projectile.is_colliding_with_circle(meteor){
                    if meteor.damage(projectile.get_damage()){
                        meteor.set_pos(SCREEN_WIDTH_F64+10000., SCREEN_HEIGHT_F64+10000.);
                        if projectile.is_kind(ProjectileKind::P02){
                            let projectile_center=projectile.get_center();
                            for velocity in [
                                (-0.64278760968653936, -0.76604444311897801),
                                (-0.49999999999999978, -0.86602540378443871),
                                (-0.34202014332566871, -0.93969262078590843),
                                (-0.1736481776669303, -0.98480775301220802),
                                (6.123233995736766e-17, -1.0),
                                (0.17364817766693041, -0.98480775301220802),
                                (0.34202014332566882, -0.93969262078590832),
                                (0.50000000000000011, -0.8660254037844386),
                                (0.64278760968653936, -0.76604444311897801),
                            ].iter(){
                                let mut projectile=Projectile::new(&textures, ProjectileKind::P03, *velocity);
                                projectile.set_center(
                                    projectile_center.0,
                                    projectile_center.1
                                );
                                pellets.push(projectile);
                            }
                        }
                    }
                    projectile.set_pos(SCREEN_WIDTH_F64+10000., SCREEN_HEIGHT_F64+10000.);
                }
            }
        }
        self.projectiles.append(&mut pellets);

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