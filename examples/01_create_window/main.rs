// examples/01_create_window/main.rs

use amethyst::{
    prelude::*,
    renderer::{
        RenderingBundle,
        types::DefaultBackend,
        plugins::{RenderToWindow, RenderFlat2D},
    },
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
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_dir("examples/01_create_window/")?;

    let display_config_path = app_root.join("display_config.ron");
    let render_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)
                .with_clear([1.0; 4]),
        )
        .with_plugin(RenderFlat2D::default());

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?;

    Application::new(app_root, ExampleState, game_data)?.run();

    Ok(())
}
