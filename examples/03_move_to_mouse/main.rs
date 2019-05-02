use amethyst::{
    prelude::*,
    core::transform::{
        TransformBundle, Transform
    },
    renderer::{
        Pipeline, Stage, DrawFlat2D, ColorMask, ALPHA, DepthMode, DisplayConfig, RenderBundle,
        Camera, Projection,
        Texture, PngFormat, TextureMetadata,
        SpriteSheet, SpriteSheetFormat, SpriteRender
    },
    input::{
        InputBundle, InputHandler, Button,
        is_key_down
    },
    assets::{
        Loader, AssetStorage
    },
    ecs::prelude::{
        Component, DenseVecStorage,
        System, Read, ReadStorage, WriteStorage, Join
    },
    winit::{
        VirtualKeyCode, MouseButton
    }
};

struct Icon;

impl Component for Icon {
    type Storage = DenseVecStorage<Self>;
}

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world);
        world.register::<Icon>();
        initialise_icon(world);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(e) = event {
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        ReadStorage<'s, Icon>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (icons, mut transforms, input): Self::SystemData) {
        if input.button_is_down(Button::Mouse(MouseButton::Left)) {
            if let Some((x, y)) = input.mouse_position() {
                for (_icon, transform) in (&icons, &mut transforms).join() {
                    transform.set_xyz(x as f32, 500.0 - y as f32, 0.0);
                }
            }
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite)
            ))
    );
    let config = DisplayConfig::load("./examples/03_move_to_mouse/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let input_bundle = InputBundle::<String, String>::new();

    let transform_bundle = TransformBundle::new();

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(input_bundle)?
        .with_bundle(transform_bundle)?
        .with(MoveSystem, "move-system", &[]);

    Application::new(
        "./examples/03_move_to_mouse/",
        ExampleState,
        game_data
    )?.run();

    Ok(())
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
        transform.set_xyz(250.0, 250.0, 1.0);
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            -250.0, 250.0, -250.0, 250.0
        )))
        .with(transform)
        .build();
}

fn initialise_icon(world: &mut World) {
    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_handle = {
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "icon.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &texture_storage,
            )
        };

        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "spritesheet.ron",
            SpriteSheetFormat,
            texture_handle,
            (),
            &sprite_sheet_store,
        )
    };

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let mut transform = Transform::default();
        transform.set_xyz(250.0, 250.0, 0.0);
    world.create_entity()
        .with(Icon)
        .with(sprite_render)
        .with(transform)
        .build();
}
