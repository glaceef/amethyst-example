use amethyst::{
    prelude::*,
    ecs::prelude::{System, Read},
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    input::{is_key_down, Bindings, InputBundle, InputHandler},
    winit::{Event, WindowEvent, ElementState, MouseButton, VirtualKeyCode},
};

struct Example;

impl SimpleState for Example {
    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(e) = event {
            if let Event::WindowEvent { ref event, .. } = e {　// refがないとmoveがおきる
                if let WindowEvent::MouseInput{ state, button, .. } = event {
                    match (state, button) {
                        (ElementState::Pressed, MouseButton::Left) => {
                            println!("click!");
                        }
                        _ => {}
                    }
                }
            }

            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

struct ExampleSystem;

impl<'a> System<'a> for ExampleSystem {
    type SystemData = (
        Read<'a, InputHandler<String, String>>,
    );

    fn run(&mut self, (input,): Self::SystemData) {
        if input.key_is_down(VirtualKeyCode::Return) { // Enter
            println!("Enter key pressed.");
        }
        if input.key_is_down(VirtualKeyCode::Space) {
            println!("Space key pressed.");
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
    let config = DisplayConfig::load("./examples/05_click/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let bindings = Bindings::<String, String>::new();
    let input_bundle = InputBundle::new().with_bindings(bindings);

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?
        .with_bundle(input_bundle)?
        .with(ExampleSystem, "system", &[]);

    let mut game = Application::new("./examples/05_click/", Example, game_data)?;

    game.run();

    Ok(())
}
