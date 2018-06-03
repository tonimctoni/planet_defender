
pub const FPS: u32 = 60;

pub const SCREEN_WIDTH: u32 = 1200;
pub const SCREEN_HEIGHT: u32 = 800;

pub const SCREEN_WIDTH_F64: f64 = SCREEN_WIDTH as f64;
pub const SCREEN_HEIGHT_F64: f64 = SCREEN_HEIGHT as f64;

pub const OUT_OF_SCREEN_X_F64: f64 = SCREEN_WIDTH_F64+10000.;
pub const OUT_OF_SCREEN_Y_F64: f64 = SCREEN_HEIGHT_F64+10000.;

pub const SHIP_EPSILON: f64 = 2.;

pub const SHIP_SPEED: f64 = 4.;

pub const ENERGY_RECOVERY_PER_FRAME: f64 = 0.002;
pub const COOLDOWN_FRAMES: isize = 10;

pub const TRIPLE_SHOT_DURATION_FRAMES: isize = (FPS as isize)*12;
pub const SHIELD_DURATION_FRAMES: isize = (FPS as isize)*5;

pub const SHIELD_ENERGY_COST: f64 = 0.5;
pub const TRIPLE_SHOT_ENERGY_COST: f64 = 1.0;


pub static PELLET_VELOCITY: [(f64,f64);9] = [
                                (-0.64278760968653936, -0.76604444311897801),
                                (-0.49999999999999978, -0.86602540378443871),
                                (-0.34202014332566871, -0.93969262078590843),
                                (-0.1736481776669303, -0.98480775301220802),
                                (6.123233995736766e-17, -1.0),
                                (0.17364817766693041, -0.98480775301220802),
                                (0.34202014332566882, -0.93969262078590832),
                                (0.50000000000000011, -0.8660254037844386),
                                (0.64278760968653936, -0.76604444311897801),
                            ];

pub static TRIPLE_SHOT_ROTATIONS: [((f64,f64),(f64,f64));2] = [
                ((0.98480775301220802, -0.17364817766693033), (0.17364817766693033, 0.98480775301220802)),
                ((0.98480775301220802, 0.17364817766693033), (-0.17364817766693033, 0.98480775301220802))
                ];

#[derive(PartialEq, Clone, Copy)]
pub enum ProjectileKind {
    P01,
    P02,
    P03,
}

impl ProjectileKind {
    pub fn get_ptexture_i(&self) -> usize{
        match self {
            &ProjectileKind::P01 => 0,
            &ProjectileKind::P02 => 1,
            &ProjectileKind::P03 => 2,
        }
    }

    pub fn get_speed(&self) -> f64{
        match self {
            &ProjectileKind::P01 => 6.,
            &ProjectileKind::P02 => 2.5,
            &ProjectileKind::P03 => 5.,
        }
    }

    pub fn get_damage(&self) -> f64{
        match self {
            &ProjectileKind::P01 => 10.,
            &ProjectileKind::P02 => 30.,
            &ProjectileKind::P03 => 20.,
        }
    }

    pub fn get_energy_cost(&self) -> f64{
        match self {
            &ProjectileKind::P01 => 0.05,
            &ProjectileKind::P02 => 0.25,
            &ProjectileKind::P03 => panic!(),
        }
    }
}