// examples/04_create_and_destroy/main.rs

use amethyst::{
    prelude::*,
    core::transform::{
        TransformBundle, Transform
    },
    renderer::{
        Pipeline, Stage, DrawFlat2D, ColorMask, ALPHA, DepthMode,
        DisplayConfig, RenderBundle,
        SpriteRender
    },
    input::{
        InputBundle,
        is_key_down
    },
    ecs::prelude::{
        System, SystemData, Resources,
        Component, DenseVecStorage,
        Entities, Read, WriteStorage, ReadExpect, Join
    },
    winit::{
        VirtualKeyCode, MouseButton
    }
};

use amethyst_test::{
    TransformExt,
    initialise_camera,
    load_sprite_sheet,
    mouse::*
};

struct Icon {
    id: u32,
    dx: f32,
    dy: f32,
}
impl Icon {
    fn new(id: u32) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Icon{
            id,
            dx: rng.gen_range(-3.0, 3.0),
            dy: rng.gen_range(-3.0, 3.0),
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
        // initialise_mouse(world);
        world.register::<Icon>();

        let sprite_sheet = load_sprite_sheet(
            world,
            "icon.png",
            "spritesheet.ron"
        );
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet,
            sprite_number: 0,
        };
        world.add_resource(sprite_render);
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
        Entities<'s>,
        WriteStorage<'s, Icon>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, SpriteRender>,
        Read<'s, Mouse>
    );

    fn run(
        &mut self,
        (entities,
         mut icons, mut sprite_renders, mut transforms,
         sr, mouse): Self::SystemData
    ) {
        // create
        if mouse.get_down(MouseButton::Left) {
            let id = { self.0 += 1; self.0 };
            let transform = Transform::from_xyz(250.0, 250.0, 0.0);
            entities.build_entity()
                .with(Icon::new(id), &mut icons)
                .with(sr.clone(), &mut sprite_renders)
                .with(transform, &mut transforms)
                .build();
        }

        // destroy
        if mouse.get_down(MouseButton::Right) && self.0 > 0 {
            let search_id = self.0;
            for (entity, icon) in (&entities, &icons).join() {
                if icon.id == search_id {
                    entities.delete(entity).unwrap();
                    self.0 -= 1;
                    break;
                }
            }
        }
    }
}

struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Icon>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut icons, mut transforms): Self::SystemData) {
        let mut amount = 0;
        for (icon, transform) in (&mut icons, &mut transforms).join() {
            let (x, y) = {
                let translation = transform.translation();
                (translation.x, translation.y)
            };
            let (next_x, next_y) = (x + icon.dx, y + icon.dy);
            if next_x < 25.0 || 475.0 <= next_x {
                icon.dx = -icon.dx;
            }
            if next_y < 25.0 || 475.0 <= next_y {
                icon.dy = -icon.dy;
            }
            transform.translate_xyz(icon.dx, icon.dy, 0.0);
            amount += 1;
        }
        print!("\ritem amount: {}", amount);
    }
}

fn main() -> amethyst::Result<()> {
    // amethyst::start_logger(Default::default());

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

    let transform_bundle = TransformBundle::new();

    let input_bundle = InputBundle::<String, String>::new();
    let mouse_bundle = MouseBundle::new();

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with_bundle(mouse_bundle)?
        .with(CreateDestroySystem(0), "create-destroy-system", &[])
        .with(MoveSystem, "move-system", &[]);

    Application::new(
        "./examples/04_create_and_destroy/",
        ExampleState,
        game_data
    )?.run();

    Ok(())
}
