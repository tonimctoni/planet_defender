
use actor_manager::ActorManager;
use textures::Textures;




pub struct GameEvents {
    step: usize
}


impl GameEvents {
    pub fn new() -> GameEvents{
        GameEvents{
            step: 0
        }
    }

    pub fn step(&mut self, actor_manager: &mut ActorManager, textures: &Textures){
        self.step+=1;
        if self.step%(60*3)==0{
            actor_manager.drop_meteor(&textures);
        }
    }
}