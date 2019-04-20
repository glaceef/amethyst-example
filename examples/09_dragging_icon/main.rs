use amethyst::{
    prelude::*,
    ecs::prelude::{
        System, Component, DenseVecStorage,
        Read, WriteStorage,
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
        is_key_down, Bindings, InputBundle
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
        world.register::<Mouse>();
        initialise_mouse(world);
        world.register::<Icon>();
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
                world
                    .create_entity()
                    .with(sprite_render)
                    .with(Icon(false))
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

#[derive(default)]
struct Mouse {
    x: f32,
    y: f32,
    state: bool,
}

impl Component for Mouse {
    type Storage = DenseVecStorage<Self>;
}

struct MouseSystem;

impl<'s> System<'s> for MouseSystem {
    type SystemData = (
        WriteStorage<'s, Mouse>,
        Read<'s, InputHundler>
    );

    fn run(&mut self, (mut mouse, input): Self::SystemData) {
        if let Some((mouse, input)) = (&mut mouse, &input).join().next() {
            if input.mouse_button_is_down(MouseButton::Left) {
                mouse.state = true;
                let mouse_pos = input.mouse_position().unwrap();
                mouse.x = mouse_pos[0];
                mouse.y = mouse_pos[1];
            } else {
                mouse.state = false;
            }
        }
    }
}

struct DragSystem;

impl<'s> System<'s> for DragSystem {
    type SystemData = (
        WriteStorage<'s, Icon>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler>
    );

    fn run(&mut self, (mut icons, transforms, input): Self::SystemData) {
        if input.mouse_button_is_down(MouseButton::Left) {
            let (mx, my) = input.mouse_position().expext("error");
            for (icon, transform) in (&mut icons, &transforms).join() {
                let (x, y) = {
                    let translation = transform.translation();
                    (translation.x, translation.y)
                };
                Icon.0 = dist(x, y, mx, my) <= 25.0;
            }
        } else {
            for (icon,) in (&mut icons,).join() {
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
    );

    fn run(&mut self, (icons, transforms): Self::SystemData) {
        for (icon, transform) in (&icons, &mut transforms).join() {
            if icon.0 {
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
    let config = DisplayConfig::load("./examples/09_dragging_item/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let transform_bundle = TransformBundle::new();

    let bindings = Bindings::<String, String>::new();
    let input_bundle = InputBundle::new().with_bindings(bindings);

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with(DragSystem, "drag-system", &[]);

    let mut game = Application::new(
        "./examples/09_dragging_item/",
        ExampleState,
        game_data
    )?;

    game.run();

    Ok(())
}

fn initialise_mouse(world: &mut World) {
    world
        .create_entity()
        .with(Mouse::default())
        .build()
}
