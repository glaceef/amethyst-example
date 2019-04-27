use amethyst::{
    prelude::*,
    core::transform::{
        TransformBundle, Transform
    },
    renderer::{
        Pipeline, Stage, DrawFlat2D, ColorMask, DepthMode, ALPHA,
        DisplayConfig, RenderBundle,
        SpriteRender
    },
    input::{
        InputBundle, InputHandler,
        is_key_down
    },
    ecs::prelude::{
        System,
        Component, DenseVecStorage,
        Read, ReadStorage, WriteStorage,
        Join
    },
    winit::{
        VirtualKeyCode
    },
};

use amethyst_test::{
    TransformExt,
    initialise_camera,
    load_sprite_sheet,
};

#[derive(Debug)]
enum State {
    Idle, Right, Left
}
impl State {
    fn offset(&self) -> usize {
        match self {
            State::Idle => 0,
            State::Right => 3,
            State::Left => 6,
        }
    }
}

struct Player(State);

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world);
        initialise_player(world);
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

struct PlayerTextureSystem(usize);

impl<'s> System<'s> for PlayerTextureSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>
    );

    fn run(&mut self, (player, mut sprite): Self::SystemData) {
        if let Some((player, sprite)) = (&player, &mut sprite).join().next() {
            sprite.sprite_number = func(self.0, 3, 7) + player.0.offset();
            self.0 += 1;
        }

        fn func(n: usize, max_index: usize, repeat: usize) -> usize {
            n % (max_index * repeat) / repeat
        }
    }
}

struct PlayerMoveSystem;

impl<'s> System<'s> for PlayerMoveSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut player, mut transform, input): Self::SystemData) {
        if let Some((player, transform)) = (&mut player, &mut transform).join().next() {
            player.0 = match (
                input.key_is_down(VirtualKeyCode::Left),
                input.key_is_down(VirtualKeyCode::Right),
            ) {
                (true, false) => { State::Left }
                (false, true) => { State::Right }
                _ => { State::Idle }
            };

            let speed = 3.0;
            let dx = input.axis_value("x_axis").unwrap() * speed;
            let dy = input.axis_value("y_axis").unwrap() * speed;
            transform.translate_xyz(dx as f32, dy as f32, 0.0);
        }
    }
}

fn main() -> amethyst::Result<()> {
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite)
            ))
    );
    let config = DisplayConfig::load("./examples/12_animation_and_move/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file("./examples/12_animation_and_move/bindings.ron")?;

    let transform_bundle = TransformBundle::new();

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(input_bundle)?
        .with_bundle(transform_bundle)?
        .with(PlayerStateSystem, "player-state-system", &[])
        .with(PlayerTextureSystem(0), "player-texture-system", &[])
        .with(PlayerMoveSystem, "player-move-system", &[]);


    Application::new("./examples/12_animation_and_move/", ExampleState, game_data)?.run();

    Ok(())
}

fn initialise_player(world: &mut World) {
    world.register::<Player>();
    let transform = Transform::from_xyz(250.0, 250.0, 0.0);
    let sprite_sheet_handle = load_sprite_sheet(world, "Cirno.png", "spritesheet.ron");
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(Player(State::Idle))
        .with(transform)
        .with(sprite_render)
        .build();
}
