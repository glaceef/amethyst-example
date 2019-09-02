// examples/02_image/main.rs

use amethyst::{
    prelude::*,
    core::{
        Transform, TransformBundle
    },
    renderer::{
        RenderingBundle,
        plugins::{
            RenderToWindow, RenderFlat2D,
        },
        types::DefaultBackend,
        Camera,
        Texture, ImageFormat,
    },
    assets::{
        Loader, AssetStorage
    },
    utils::application_dir,
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

    let app_root = application_dir("./examples/02_image/")?;

    let render_bundle = RenderingBundle::<DefaultBackend>::new().with_plugin(
        RenderToWindow::from_config_path(app_root.join("display_config.ron"))
            .with_clear([0.0, 0.0, 0.0, 1.0]),
    ).with_plugin(RenderFlat2D::default());

    let transform_bundle = TransformBundle::new();

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?
        .with_bundle(transform_bundle)?;

    let mut game = Application::new("./examples/02_image/", ExampleState, game_data)?;

    game.run();

    Ok(())
}

fn init_camera(world: &mut World) {
    let camera = Camera::standard_2d(500.0, 500.0);
    let mut transform = Transform::default();
    transform.set_translation_xyz(250.0, 250.0, 1.0);
    world
        .create_entity()
        .with(transform)
        .with(camera)
        .build();
}

fn init_image(world: &mut World) {
    let texture = world.read_resource::<Loader>().load(
        "logo.png",
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(250.0, 250.0, 0.0);
    world
        .create_entity()
        .with(transform)
        .with(texture)
        .build();
}
