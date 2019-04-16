use amethyst::{
    prelude::*,
    ecs::prelude::{System, Component, DenseVecStorage, Read, WriteStorage, Join},
    core::{
        timing::Time,
        transform::{Transform, TransformBundle},
    },
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage,
        Camera, Projection,
        PngFormat,
        SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
        Texture, TextureMetadata
    },
    input::{is_key_down, Bindings, InputBundle},
    assets::{
        Loader, AssetStorage
    },
    winit::{Event, WindowEvent, ElementState, MouseButton, VirtualKeyCode},
};

// Transformを別に付けられるので位置情報は要らないが、
// 速度など、そのほかの情報は必要。
struct Icon {
    r: f32,
    v: [f32; 2],
}

impl Component for Icon {
    type Storage = DenseVecStorage<Self>;
}

struct ExampleState(u32);

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Icon>();
        initialise_camera(world);
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
                            let sprite_sheet_handle = load_sprite_sheet(world);
                            let sprite_render = SpriteRender {
                                sprite_sheet: sprite_sheet_handle.clone(),
                                sprite_number: 0,
                            };
                            let mut transform = Transform::default();
                            transform.set_xyz(250.0, 250.0, 0.0);
                            world
                                .create_entity()
                                .with(sprite_render)
                                .with(Icon{
                                    r: 25.0,
                                    v: [60.0, 45.0],
                                })
                                .with(transform)
                                .build();

                            self.0 += 1;
                            println!("create! icon amount: {}", self.0);
                            // なんとかタイトルを動的に変更できないものか...
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

impl<'s> System<'s> for ExampleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Icon>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, mut icons, time): Self::SystemData) {
        for (transform, mut icon) in (&mut transforms, &mut icons).join() {
            let translation = transform.translation();
            let (x, y) = (translation.x, translation.y);

            let (width, height) = (500.0, 500.0);
            let r = icon.r;
            if x < r || width - r <= x {
                icon.v[0] *= -1.0;
            }
            if y < r || height - r <= y {
                icon.v[1] *= -1.0;
            }

            transform.translate_xyz(
                icon.v[0] * time.delta_seconds(),
                icon.v[1] * time.delta_seconds(),
                0.0
            );
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
    );
    let config = DisplayConfig::load("./examples/07_bounding_icons/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let transform_bundle = TransformBundle::new();

    let bindings = Bindings::<String, String>::new();
    let input_bundle = InputBundle::new().with_bindings(bindings);

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with(ExampleSystem, "system", &[]);

    let mut game = Application::new(
        "./examples/08_physical_icons/",
        ExampleState(0),
        game_data
    )?;

    game.run();

    Ok(())
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, 500.0, 0.0, 500.0
        )))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "icon.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}
