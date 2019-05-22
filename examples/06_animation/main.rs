// examples/06_animation/main.rs

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
        is_key_down,
    },
    utils::application_root_dir,
    ecs::prelude::{
        Component, DenseVecStorage, Entity,
        System, ReadStorage, WriteStorage, Read, ReadExpect
    },
    winit::VirtualKeyCode,
};

use amethyst_test::{
    TransformExt,
    initialise_camera,
    load_sprite_sheet
};

use std::path::PathBuf;

enum State {
    Idle, Right, Left
}
impl State {
    fn offset(&self) -> usize {
        match self {
            State::Idle => 0,
            State::Right => 4,
            State::Left => 8,
        }
    }
}

struct Player(State);

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

struct PlayerEntity(Entity);

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world, [500.0, 500.0]);
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

struct PlayerSpriteSystem(usize);

impl<'s> System<'s> for PlayerSpriteSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, PlayerEntity>
    );

    fn run(&mut self, (players, mut sprites, player): Self::SystemData) {
        match (players.get(player.0), sprites.get_mut(player.0)) {
            (Some(player), Some(sprite)) => {
                sprite.sprite_number = num_extend(self.0, 4, 7) + player.0.offset();
                self.0 += 1;
            }
            _ => {}
        }

        fn num_extend(n: usize, size: usize, repeat: usize) -> usize {
            n % (size * repeat) / repeat
        }
    }
}

struct PlayerMoveSystem;

impl<'s> System<'s> for PlayerMoveSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, PlayerEntity>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut players, mut transforms, player, input): Self::SystemData) {
        let dx = input.axis_value("x_axis").unwrap() as f32;
        let dy = input.axis_value("y_axis").unwrap() as f32;
        let speed = 3.0;
        if let Some(transform) = transforms.get_mut(player.0) {
            transform.translate_xyz(dx * speed, dy * speed, 0.0);
        }

        if let Some(player) = players.get_mut(player.0) {
            player.0 = match dx as i32 {
                 0 => State::Idle,
                 1 => State::Right,
                -1 => State::Left,
                _ => panic!(),
            };
        }
    }
}

fn main() -> amethyst::Result<()> {
    // amethyst::start_logger(Default::default());

    let app_root = PathBuf::from(application_root_dir()).join("examples/06_animation/");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.001, 0.0, 0.02, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite)
            ))
    );
    let config = DisplayConfig::load(app_root.join("config.ron"));
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let transform_bundle = TransformBundle::new();

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(app_root.join("bindings.ron"))?;

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with(PlayerSpriteSystem(0), "player_sprite_system", &[])
        .with(PlayerMoveSystem, "player-move-system", &[]);

    Application::new(app_root, ExampleState, game_data)?.run();

    Ok(())
}

fn initialise_player(world: &mut World) {
    world.register::<Player>();
    let sprite_sheet = load_sprite_sheet(world, "dot_reimu.png", "spritesheet.ron");
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };
    let transform = Transform::from_xyz(250.0, 250.0, 0.0);

    let entity = world.create_entity()
        .with(Player(State::Idle))
        .with(sprite_render)
        .with(transform)
        .build();
    world.add_resource(PlayerEntity(entity));
}
