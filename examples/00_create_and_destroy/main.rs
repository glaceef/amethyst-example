// examples/04_create_and_destroy/main.rs

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

use amethyst_test::{
    TransformExt,
    initialise_camera,
    load_sprite_sheet,
    is_mouse_down,
};

struct Icon {
    id: u32,
    dx: f32,
    dy: f32,
}
impl Icon {
    fn new(id: u32) -> Self {
        let rng = rand::thread_rng();
        Icon{
            dx: rng::gen_range(-5.0, 5.0),
            dy: rng::gen_range(-5.0, 5.0),
        }
    }
}

impl Component for Icon {
    type Storage = DenseVecStorage<Self>;
}

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world, [500.0, 500.0]);
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

struct CreateDestroySystem(u32);

impl<'s> System<'s> for CreateDestroySystem {
    type SystemData = (
        WriteStorage<'s, Icon>
        Entities<'s>,
        Read<'s, Mouse>
    );

    fn run(&mut self, (icons, entities, mouse): Self::SystemData) {
        // create
        if mouse.get_down(MouseButton::Left) {
            let sprite_sheet = load_sprite_sheet(
                world,
                "icon.png",
                "spritesheet.ron"
            );
            // こいつをResourceにする？
            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet,
                sprite_number: 0,
            };

            let id = self.0;
            let mut transform = Transform::from_xyz(250.0, 250.0, 0.0);
            entities.create()
                .with(Icon::new(id))
                .with(sprite_render)
                .with(transform)
                .build();
            self.0 += 1;
        }

        // destroy
        if mouse.get_down(MouseButton::Right) && self.0 > 0 {
            let id = self.0;
            if let Some(entity) = (||{
                for (entity, icon) in (&entities, &mut icons).join() {
                    if icon.id == id {
                        return Some(entity);
                    }
                }
                None
            })() {
                entities.delete(entity).unwrap();
                self.0 -= 1;
            }
        }
    }
}

struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        ReadStorage<'s, Icon>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (icons, mut transforms): Self::SystemData) {
        for (icon, transform) in (&icons, &mut transforms).join() {
            transform.move_right(icon.dx);
            transform.move_up(icon.dy);
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
    let config = DisplayConfig::load("./examples/04_create_and_destroy/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let input_bundle = InputBundle::<String, String>::new();

    let transform_bundle = TransformBundle::new();

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(input_bundle)?
        .with_bundle(transform_bundle)?
        .with(MoveSystem, "move-system", &[]);

    Application::new(
        "./examples/04_create_and_destroy/",
        ExampleState,
        game_data
    )?.run();

    Ok(())
}

fn initialise_icon(world: &mut World) {
    let sprite_sheet = load_sprite_sheet(
        world,
        "icon.png",
        "spritesheet.ron"
    );

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };

    let mut transform = Transform::from_xyz(250.0, 250.0, 0.0);
    world.create_entity()
        .with(Icon::new())
        .with(sprite_render)
        .with(transform)
        .build();
}
