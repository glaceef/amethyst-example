use amethyst::{
    prelude::*,
    core::transform::{
        TransformBundle,
        Transform
    },
    renderer::{
        Pipeline, Stage, DrawFlat2D,
        DisplayConfig,
        RenderBundle,
    },
    input::is_key_down,
    ecs::prelude::{
        Entity,
        Component, DenseVecStorage,
        System,
        ReadExpect, WriteStorage,
    },
    winit::VirtualKeyCode,
};

struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

struct PlayerEntity(Entity);

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Player>();
        let mut transform = Transform::default();
            transform.set_xyz(0.0, 250.0, 0.0);
        let player = world.create_entity()
            .with(Player)
            .with(transform)
            .build();
        world.add_resource(PlayerEntity(player));
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(e) = event {
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        ReadExpect<'s, PlayerEntity>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (player_entity, mut transforms): Self::SystemData) {
        // PlayerEntity構造体に与えたEntityを媒介にしてコンポーネントを選択できる。
        // このため、単一のEntityのためだけにjoin()をする必要がなくなる。
        if let Some(transform) = transforms.get_mut(player_entity.0) {
            transform.move_right(0.1);
            println!("player pos_x: {:.1}", transform.translation().x);
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(DrawFlat2D::new())
    );

    let config = DisplayConfig::load("./examples/14_single_entity/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let transform_bundle = TransformBundle::new();

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?
        .with_bundle(transform_bundle)?
        .with(PlayerSystem, "player-system", &[]);

    Application::new(
        "./examples/14_single_entity/",
        ExampleState,
        game_data
    )?.run();

    Ok(())
}
