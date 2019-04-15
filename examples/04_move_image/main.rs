use amethyst::{
    prelude::*,
    core::transform::{
        Transform, TransformBundle
    },
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage,
        Camera, Projection,
        PngFormat,
        SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
        Texture, TextureMetadata
    },
    input::{
        InputBundle, InputHandler
    },
    assets::{
        AssetStorage, Loader
    },
    ecs::prelude::{
        Component, DenseVecStorage, Join, Read, ReadStorage, System, WriteStorage
    }
};

pub struct ImageState;

impl SimpleState for ImageState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Image>();

        initialise_images(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

struct Image {
    width: f32,
    height: f32,
}

// world.register::<Image>()するために必要
impl Component for Image {
    type Storage = DenseVecStorage<Self>;
}

struct ImageSystem;

impl<'s> System<'s> for ImageSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Image>, // これ使ってないが、無いと動かない
        Read<'s, InputHandler<String, String>>,
    );

    // fn run(&mut self, (mut transforms, input): Self::SystemData) {
    //     for (transform) in (&mut transforms).join() {
    fn run(&mut self, (mut transforms, images, input): Self::SystemData) {
        for (image, transform) in (&images, &mut transforms).join() {
            // axis_value(axis_name) -> Option<f64> ; 0.0 or 1.0 or -1.0
            let speed = 3.0;
            let dx = if let Some(rl_movement) = input.axis_value("lr_axis") {
                rl_movement * speed
            } else { 0.0 };
            let dy = if let Some(ud_movement) = input.axis_value("ud_axis") {
                ud_movement * speed
            } else { 0.0 };

            transform
                .move_right(dx as f32)
                .move_down(dy as f32);
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );
    let config = DisplayConfig::load("./examples/04_move_image/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let transform_bundle = TransformBundle::new();

    // let mut bindings = Bindings::<String, String>::new();
    // let _axis = bindings.insert_axis(
    //     "lr_axis",
    //     Axis::Emulated {
    //         pos: Button::Key(VirtualKeyCode::Right),
    //         neg: Button::Key(VirtualKeyCode::Left)
    //     }
    // ).unwrap();
    // let _axis = bindings.insert_axis(
    //     "ud_axis",
    //     Axis::Emulated {
    //         pos: Button::Key(VirtualKeyCode::Up),
    //         neg: Button::Key(VirtualKeyCode::Down)
    //     }
    // ).unwrap();
    // let input_bundle = InputBundle::new().with_bindings(bindings);
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file("./examples/04_move_image/bindings.ron")?;

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())? // リファレンス見てもよーわからん
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with(ImageSystem, "system", &[]);

    let mut game = Application::new("examples/04_move_image/", ImageState, game_data)?;

    game.run();

    Ok(())
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "image.png",
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

fn initialise_images(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut transform = Transform::default();
    transform.set_xyz(250.0, 250.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // spritesheetのspritesのインデックス？
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Image{width: 50.0, height: 50.0})
        .with(transform)
        .build();
}
