use amethyst::{
    assets::{AssetStorage, Loader},
    core::{Transform, TransformBundle},
    ecs::Entity,
    prelude::*,
    renderer::{
        Camera, ColorMask, DepthMode, DisplayConfig, DrawFlat2D, Pipeline, PngFormat, Projection,
        RenderBundle, Stage, Texture, TextureHandle, TextureMetadata, ALPHA,
    },
};

struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // let texture_handle = load_texture(world, "./examples/image/logo.png");
        let texture_handle = load_texture(world, "logo.png");
        let _image = init_image(world, &texture_handle);

        init_camera(world)
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let config = DisplayConfig::load("./examples/02_image/config.ron");
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            )),
    );

    // let game_data = GameDataBuilder::default()
    let game_data = GameDataBuilder::new()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;

    let mut game = Application::new("./examples/02_image/", Example, game_data)?;
    // ここでアプリケーションの位置を指定すると、logo.pngへの相対パスの書き方が変わる。
    // 上のconfigはアプリケーションが読みに行く前の相対パスなので、プロジェクトルートからになる。
    game.run();

    Ok(())
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -250.0, 250.0, -250.0, 250.0, // 四角いカメラの大きさ。これで500x500。
            // Projection::perspective(l, r, b, t)   : 遠近感を表現
            // Projection::orthographic(aspect, fov) : 遠近感なし、平面で描画
        )))
        .with(transform)
        .build();
}

fn init_image(world: &mut World, texture: &TextureHandle) -> Entity {
    let mut transform = Transform::default(); // new()の場合いくつかパラメータを与える必要がある。
    transform.set_x(0.0);
    transform.set_y(0.0);

    world
        .create_entity()
        .with(transform)
        .with(texture.clone())
        .build()
}

fn load_texture(world: &mut World, path: &str) -> TextureHandle {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        path,
        PngFormat, // JpgFormat や BmpFormat など
        TextureMetadata::srgb_scale(), // いろいろ試すしかない
        (), // いろいろ試すしかない
        &texture_storage,
    )
}
