use crate::{
    components::base::{CameraComponent, TransformComponent},
    ecs::{EntityId, Game, System},
};

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

fn register_systems(game: &mut Game) {
    game.register_system(MessageSystem {});
}

fn register_camera(game: &mut Game) {
    let camera_entity = game.new_entity();
    game.add_component(
        camera_entity,
        TransformComponent {
            pos: (0.0, 0.0, 0.0),
            rot: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
        },
    );
    game.add_component(camera_entity, CameraComponent {});
}

// TODO
fn register_cube(_game: &mut Game) {}

pub fn game_main(game: &mut Game) {
    register_systems(game);
    register_camera(game);

    let my_entity = game.new_entity();
    game.add_component(my_entity, MessageComponent { msg: "Hello!" });

    // TODO: move this into a loop or something
    game.run_systems();
}
