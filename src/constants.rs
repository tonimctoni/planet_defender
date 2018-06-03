
pub const SCREEN_WIDTH: u32 = 1200;
pub const SCREEN_HEIGHT: u32 = 800;

pub const SCREEN_WIDTH_F64: f64 = SCREEN_WIDTH as f64;
pub const SCREEN_HEIGHT_F64: f64 = SCREEN_HEIGHT as f64;

pub const SHIP_EPSILON: f64 = 2.;

pub const SHIP_SPEED: f64 = 4.;

pub const ENERGY_RECOVERY_PER_FRAME: f64 = 0.002;
pub const COOLDOWN_FRAMES: isize = 10;

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