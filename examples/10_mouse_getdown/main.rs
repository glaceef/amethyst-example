use amethyst::{
    prelude::*,
    ecs::prelude::{
        System,
        Read
    },
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage,
    },
    input::{
        is_key_down, Bindings, InputBundle
    },
    winit::{
        VirtualKeyCode, MouseButton
    },
};

use amethyst_test::mouse::*;

struct ClickSystem;

impl<'s> System<'s> for ClickSystem {
    type SystemData = Read<'s, Mouse>;

    fn run(&mut self, mouse: Self::SystemData) {
        if mouse.get_down(MouseButton::Left) {
            println!("press!");
        }
        if mouse.get_up(MouseButton::Left) {
            println!("release!");
        }
    }
}

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.add_resource(Mouse::new());
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
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
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
    );
    let config = DisplayConfig::load("./examples/10_mouse_getdown/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let bindings = Bindings::<String, String>::new();
    let input_bundle = InputBundle::new().with_bindings(bindings);

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(input_bundle)?
        .with(MouseSystem, "mouse-system", &[])
        .with(ClickSystem, "click-system", &[]);

    let mut game = Application::new(
        "./examples/10_mouse_getdown/",
        ExampleState,
        game_data
    )?;

    game.run();

    Ok(())
}
