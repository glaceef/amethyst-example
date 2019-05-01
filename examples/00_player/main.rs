use amethyst::{
    prelude::*,
    renderer::{
        Pipeline, Stage, DrawFlat2D,
        DisplayConfig,
        RenderBundle,
    },
    input::is_key_down,
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

        let transform = Transform::default();
            transform.set_xyz(250.0, 250.0, 0.0);
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

// System

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(DrawFlat2D::new())
    );

    let config = DisplayConfig::load("./examples/00_player/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?;
    let mut game = Application::new(
        "./examples/00_player/",
        ExampleState,
        game_data
    )?;

    game.run();

    Ok(())
}
