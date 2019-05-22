// examples/06_animation/main.rs

use amethyst::{
    prelude::*,
    renderer::{
        Pipeline, Stage, DrawFlat2D,
        DisplayConfig, RenderBundle,
    },
    input::{
        InputBundle, InputHandler,
        is_key_down,
    },
    utils::application_root_dir,
    ecs::prelude::{
        System, Read
    },
    winit::VirtualKeyCode,
};

use serde::*;

use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
enum Key {
    X_AXIS, Y_AXIS
}

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

struct ExampleSystem;

impl<'s> System<'s> for ExampleSystem {
    type SystemData = Read<'s, InputHandler<Key, String>>;

    fn run(&mut self, input: Self::SystemData) {
        let x_axis = input.axis_value(&Key::X_AXIS);
        let y_axis = input.axis_value(&Key::Y_AXIS);
        match (x_axis, y_axis) {
            (Some(x_axis), Some(y_axis)) => {
                println!("x_axis: {}, y_axis: {}", x_axis, y_axis);
            }
            _ => {}
        }
    }
}

fn main() -> amethyst::Result<()> {
    // amethyst::start_logger(Default::default());

    let app_root = PathBuf::from(application_root_dir()).join("examples/05_keyboard_input/");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(DrawFlat2D::new())
    );
    let config = DisplayConfig::load(app_root.join("config.ron"));
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let input_bundle = InputBundle::<Key, String>::new()
        .with_bindings_from_file(app_root.join("bindings.ron"))?;

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?
        .with_bundle(input_bundle)?
        .with(ExampleSystem, "example-system", &[]);

    Application::new(app_root, ExampleState, game_data)?.run();

    Ok(())
}
