use amethyst::{
    prelude::*,
    ecs::prelude::{
        System, Component, DenseVecStorage,
        WriteStorage, ReadStorage, Read,
        Join
    },
    core::transform::{
        Transform, TransformBundle
    },
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage,
        SpriteRender
    },
    input::{
        is_key_down, Bindings, InputBundle, InputHandler
    },
    winit::{
        VirtualKeyCode, MouseButton
    },
};

use amethyst_test::*;

struct Icon(bool);

impl Component for Icon {
    type Storage = DenseVecStorage<Self>;
}

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world);
        initialise_mouse(world);
        world.register::<Icon>();
        initialise_icon(world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        // let world = data.world;
        if let StateEvent::Window(e) = event {
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

#[derive(Default)]
struct Mouse {
    x: f32,
    y: f32,
    mx: f32,
    my: f32,
    press: bool,
}

impl Component for Mouse {
    type Storage = DenseVecStorage<Self>;
}

struct MouseSystem;

impl<'s> System<'s> for MouseSystem {
    type SystemData = (
        WriteStorage<'s, Mouse>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut mouse, input): Self::SystemData) {
        let mouse = (&mut mouse).join().next().unwrap();
        mouse.press = input.mouse_button_is_down(MouseButton::Left);
        if let Some(mouse_pos) = input.mouse_position() {
            let (x, y) = (mouse_pos.0 as f32, mouse_pos.1 as f32);
            mouse.mx = x - mouse.x;
            mouse.my = y - mouse.y;
            mouse.x = x;
            mouse.y = y;
        }
    }
}

struct DragSystem;

impl<'s> System<'s> for DragSystem {
    type SystemData = (
        WriteStorage<'s, Icon>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Mouse>
    );

    fn run(&mut self, (mut icons, transforms, mouse): Self::SystemData) {
        let mouse = mouse.join().next().unwrap();
        if mouse.press {
            for (icon, transform) in (&mut icons, &transforms).join() {
                let translation = transform.translation();
                let (x, y) = (translation.x, translation.y);
                icon.0 = dist(x, y, mouse.x, mouse.y) <= 25.0;
            }
        } else {
            for icon in (&mut icons).join() {
                icon.0 = false;
            }
        }
    }
}

struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        ReadStorage<'s, Icon>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Mouse>,
    );

    fn run(&mut self, (icons, mut transforms, mouse): Self::SystemData) {
        let mouse = mouse.join().next().unwrap();
        for (icon, transform) in (&icons, &mut transforms).join() {
            if icon.0 {
                transform.translate_xyz(mouse.mx, -mouse.my, 0.0);
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
    let config = DisplayConfig::load("./examples/09_dragging_icon/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let transform_bundle = TransformBundle::new();

    let bindings = Bindings::<String, String>::new();
    let input_bundle = InputBundle::new().with_bindings(bindings);

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with(MouseSystem, "mouse-system", &[])
        .with(DragSystem, "drag-system", &[])
        .with(MoveSystem, "move-system", &[]);

    let mut game = Application::new(
        "./examples/09_dragging_icon/",
        ExampleState,
        game_data
    )?;

    game.run();

    Ok(())
}

fn initialise_mouse(world: &mut World) {
    world.register::<Mouse>();
    world
        .create_entity()
        .with(Mouse::default())
        .build();
}

fn initialise_icon(world: &mut World) {
    let sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world),
        sprite_number: 0,
    };
    let mut transform = Transform::default();
    transform.set_xyz(250.0, 250.0, 0.0);
    world
        .create_entity()
        .with(sprite_render)
        .with(Icon(false))
        .with(transform)
        .build();
}

fn dist(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    (x1 - x0).hypot(y1 - y0)
}
