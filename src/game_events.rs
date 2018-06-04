
use actor_manager::ActorManager;
use energy_meter::EnergyMeter;
use textures::Textures;
use constants::MeteorKind;
use constants::{TRIPLE_SHOT_DURATION_FRAMES, SHIELD_DURATION_FRAMES};



pub struct GameEvents {
    step: usize,
    triple_shot: isize,
    shield: isize,
}


impl GameEvents {
    pub fn new() -> GameEvents{
        GameEvents{
            step: 0,
            triple_shot: 0,
            shield: 0,
        }
    }

    pub fn sched_disable_triple_shot(&mut self){
        self.triple_shot=TRIPLE_SHOT_DURATION_FRAMES;
    }

    pub fn sched_disable_shield(&mut self){
        self.shield=SHIELD_DURATION_FRAMES;
    }

    pub fn step(&mut self, actor_manager: &mut ActorManager, energy_meter: &mut EnergyMeter, textures: &Textures){
        self.step+=1;
        if self.step%(60*13)==0{
            actor_manager.drop_meteor(&textures, MeteorKind::M03);
        } else if self.step%(60*5)==0{
            actor_manager.drop_meteor(&textures, MeteorKind::M02);
        } else if self.step%(60*2)==0{
            actor_manager.drop_meteor(&textures, MeteorKind::M01);
        }

        if self.triple_shot!=0{
            self.triple_shot-=1;
            if self.triple_shot==0{
                actor_manager.set_triple_shot(false);
                energy_meter.set_triple_shot(false);
            }
        }

        if self.shield!=0{
            self.shield-=1;
            if self.shield==0{
                actor_manager.set_shield(false);
            }
        }
    }
}