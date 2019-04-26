// examples/01_create_window/main.rs

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

struct ExampleState;

impl SimpleState for ExampleState {
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

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let config = DisplayConfig::load("./examples/01_create_window/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?;
    let mut game = Application::new(
        "./01_create_window/",
        ExampleState,
        game_data
    )?;

    game.run();

    Ok(())
}
