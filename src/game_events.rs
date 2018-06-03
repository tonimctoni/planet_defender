
use actor_manager::ActorManager;
use energy_meter::EnergyMeter;
use textures::Textures;




pub struct GameEvents {
    step: usize,
    triple_shot: isize,
}


impl GameEvents {
    pub fn new() -> GameEvents{
        GameEvents{
            step: 0,
            triple_shot: 0,
        }
    }

    pub fn sched_disable_triple_shot(&mut self){
        self.triple_shot=60*12;
    }

    pub fn step(&mut self, actor_manager: &mut ActorManager, energy_meter: &mut EnergyMeter, textures: &Textures){
        self.step+=1;
        if self.step%(60*2)==0{
            actor_manager.drop_meteor(&textures);
        }

        if self.triple_shot!=0{
            self.triple_shot-=1;
            if self.triple_shot==0{
                actor_manager.set_triple_shot(false);
                energy_meter.set_triple_shot(false);
            }
        }
    }
}