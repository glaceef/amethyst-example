// examples/02_image/main.rs

use amethyst::{
    prelude::*,
    core::{
        Transform, TransformBundle
    },
    renderer::{
        DisplayConfig, Pipeline, Stage, DrawFlat2D, ColorMask, ALPHA, DepthMode,
        Camera, Projection,
        Texture, PngFormat, TextureMetadata,
        RenderBundle
    },
    assets::{
        Loader, AssetStorage
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

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let config = DisplayConfig::load("./examples/02_image/config.ron");
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
        .with_bundle(transform_bundle)?;

    let mut game = Application::new("./examples/02_image/", ExampleState, game_data)?;

    game.run();

    Ok(())
}

fn init_camera(world: &mut World) {
    let camera = Camera::from(Projection::orthographic(
        -250.0, 250.0, -250.0, 250.0
        // Projection::perspective(l, r, b, t)   : 遠近感を表現
        // Projection::orthographic(aspect, fov) : 遠近感なし、平面で描画
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
        .with(transform)
        .build();
}
