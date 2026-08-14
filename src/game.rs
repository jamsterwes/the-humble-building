use crate::ecs::{EntityId, Game, System};

struct MessageComponent {
    pub msg: &'static str,
}

struct MessageSystem {}
impl System for MessageSystem {
    type Component = MessageComponent;

    fn run(&self, eid: EntityId, component: &mut MessageComponent) {
        println!("Entity {} says {}", eid, component.msg);
    }
}

pub fn game_main(game: &mut Game) {
    game.register_system(MessageSystem {});

    let my_entity = game.new_entity();
    game.add_component(my_entity, MessageComponent { msg: "Hello!" });

    game.run_systems();
}
