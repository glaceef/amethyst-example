use amethyst::{
    prelude::*,
    ecs::prelude::{System, Component, DenseVecStorage, Read, ReadStorage, WriteStorage, Join},
    core::transform::{Transform, TransformBundle},
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    input::{is_key_down, Bindings, InputBundle, InputHandler},
    winit::{Event, WindowEvent, ElementState, MouseButton, VirtualKeyCode},
};

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Item>();
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let world = data.world;
        if let StateEvent::Window(e) = event {
            if let Event::WindowEvent { ref event, .. } = e { // refがないとmoveがおきる
                if let WindowEvent::MouseInput{ state, button, .. } = event {
                    match (state, button) {
                        (ElementState::Pressed, MouseButton::Left) => {
                            // TransformBundleなども必要なければ用意しなくて良い。
                            world
                                .create_entity()
                                .with(Item)
                                .build();
                            println!("create!");
                        }
                        _ => {}
                    }
                }
            }

            if is_key_down(&e, VirtualKeyCode::Return) {
                // 要素１のタプルとして扱わないといけない。
                let (items,): (ReadStorage<Item>,) = world.system_data();
                let mut cnt = 0;
                for _ in (&items,).join() {
                    cnt += 1;
                }
                println!("item amount: {}", cnt);
            }

            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

struct Item;

impl Component for Item {
    type Storage = DenseVecStorage<Self>;
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(DrawFlat2D::new())
    );
    let config = DisplayConfig::load("./examples/06_create_item/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let bindings = Bindings::<String, String>::new();
    let input_bundle = InputBundle::new().with_bindings(bindings);

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?
        .with_bundle(input_bundle)?;

    let mut game = Application::new("./examples/06_create_item/", ExampleState, game_data)?;

    game.run();

    Ok(())
}
