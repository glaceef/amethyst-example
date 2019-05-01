// examples/02-2_colored_image/main.rs

use amethyst::{
    prelude::*,
    core::{
        Transform, TransformBundle
    },
    renderer::{
        DisplayConfig, Pipeline, Stage, DrawFlat2D, ColorMask, ALPHA, DepthMode,
        Camera, Projection,
        Texture, PngFormat, TextureMetadata,
        RenderBundle,
        Rgba
    },
    assets::{
        Loader, AssetStorage
    },
    ecs::prelude::{
        System,
        WriteStorage,
        Join
    },
};

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        init_camera(world);
        init_image(world);
    }
}

struct TextureSystem(u32);

impl<'s> System<'s> for TextureSystem {
    type SystemData = WriteStorage<'s, Rgba>;

    fn run(&mut self, mut color: Self::SystemData) {
        if let Some(ref mut color) = (&mut color).join().next() {
            let c = (self.0 as f32).to_radians().sin().abs();
            color.1 = c;
            color.2 = c;
        }
        self.0 = if self.0 == 359 { 0 } else { self.0 + 1 };
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let config = DisplayConfig::load("./examples/13_colored_image/config.ron");
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            )),
    );
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let transform_bundle = TransformBundle::new();

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?
        .with_bundle(transform_bundle)?
        .with(TextureSystem(0), "texture-system", &[]);

    let mut game = Application::new("./examples/13_colored_image/", ExampleState, game_data)?;

    game.run();

    Ok(())
}

fn init_camera(world: &mut World) {
    let camera = Camera::from(Projection::orthographic(
        -250.0, 250.0, -250.0, 250.0
    ));
    let mut transform = Transform::default();
    transform.set_xyz(250.0, 250.0, 1.0);
    world
        .create_entity()
        .with(camera)
        .with(transform)
        .build();
}

fn init_image(world: &mut World) {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "logo.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let mut transform = Transform::default();
    transform.set_xyz(250.0, 250.0, 0.0);
    world
        .create_entity()
        .with(texture_handle)
        .with(Rgba::red())
        .with(transform)
        .build();
}
