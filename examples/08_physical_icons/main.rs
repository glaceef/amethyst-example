use amethyst::{
    prelude::*,
    ecs::prelude::{System, Component, DenseVecStorage, Read, ReadStorage, WriteStorage, Join},
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

struct Icon {
    r: f32,
    v: [f32; 2],
}

impl Component for Icon {
    type Storage = DenseVecStorage<Self>; // TODO::ここの種類調べる
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
            if is_mouse_down(&e, MouseButton::Left) {
                let sprite_render = SpriteRender {
                    sprite_sheet: load_sprite_sheet(world),
                    sprite_number: 0,
                };
                let mut transform = Transform::default();
                transform.set_xyz(250.0, 250.0, 0.0);
                let v = {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    [
                        rng.gen_range(50.0, 100.0) * if rng.gen::<bool>() { 1.0 } else { -1.0 },
                        rng.gen_range(50.0, 100.0) * if rng.gen::<bool>() { 1.0 } else { -1.0 },
                    ]
                };
                world
                    .create_entity()
                    .with(sprite_render)
                    .with(Icon{
                        r: 25.0,
                        v,
                    })
                    .with(transform)
                    .build();
            }

            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

struct MovingSystem;

impl<'s> System<'s> for MovingSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Icon>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, icons, time): Self::SystemData) {
        for (transform, icon) in (&mut transforms, &icons).join() {
            transform.translate_xyz(
                icon.v[0] * time.delta_seconds(),
                icon.v[1] * time.delta_seconds(),
                0.0
            );
        }
    }
}

struct BoundingSystem;

impl<'s> System<'s> for BoundingSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Icon>,
        ReadStorage<'s, Transform>
    );

    fn run(&mut self, (transforms, mut icons, others): Self::SystemData) {
        let others: Vec<(&Transform,)> = (&others,).join().collect();
        for (transform, icon) in (&transforms, &mut icons).join() {
            let translation = transform.translation();
            let (x, y) = (translation.x, translation.y);

            // 壁に対する跳ね返り
            let (width, height) = (500.0, 500.0);
            let r = icon.r;
            if x < r || width - r <= x {
                icon.v[0] *= -1.0;
            }
            if y < r || height - r <= y {
                icon.v[1] *= -1.0;
            }

            // 他のiconとの接触
            for (t,) in others.iter() {
                if dist(transform, t) <= 50.0 && &transform != t {
                    icon.v[0] *= -1.0;
                    icon.v[1] *= -1.0;
                }
            }
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
    let config = DisplayConfig::load("./examples/08_physical_icons/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let transform_bundle = TransformBundle::new();

    let bindings = Bindings::<String, String>::new();
    let input_bundle = InputBundle::new().with_bindings(bindings);

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with(MovingSystem, "moving_system", &[])
        .with(BoundingSystem, "bounding_system", &[]);

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

fn is_mouse_down(event: &Event, mouse_button: MouseButton) -> bool {
    if let Event::WindowEvent { ref event, .. } = event { // refがないとmoveがおきる
        if let WindowEvent::MouseInput{ state, button, .. } = event {
            match (state, button) {
                (ElementState::Pressed, mouse_button) => {
                    return true
                }
                _ => {}
            }
        }
    }
    false
}

fn dist(t1: &Transform, t2: &Transform) -> f32 {
    let t1 = t1.translation();
    let t2 = t2.translation();
    let dist_x = (t1.x - t2.x).abs();
    let dist_y = (t1.y - t2.y).abs();
    dist_x.hypot(dist_y)
}
