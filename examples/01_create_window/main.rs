// examples/01_create_window/main.rs

// cargo run --example 01 --features vulkan

use amethyst::{
    prelude::*,
    window::WindowBundle,
    input::is_key_down,
    utils::application_dir,
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
    // amethyst::start_logger(Default::default());

    let app_root = application_dir("examples/01_create_window/")?;

    let game_data = GameDataBuilder::new()
        .with_bundle(WindowBundle::from_config_path(app_root.join("config.ron")))?;

    Application::new(app_root, ExampleState, game_data)?.run();

    Ok(())
}
