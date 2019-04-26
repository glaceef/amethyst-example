use amethyst::{
    prelude::*,
    core::transform::{
        TransformBundle, Transform
    },
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, Camera,
        SpriteRender
    },
    input::{
        is_key_down, Bindings, InputBundle
    },
    ecs::prelude::{
        System,
        Component, DenseVecStorage,
        ReadStorage, WriteStorage,
        Join
    },
    winit::VirtualKeyCode,
};

use amethyst_test::{
    initialise_camera,
    load_sprite_sheet
};

/*
[everpazzle/src/systems/block_system.rs]
sprites.get_mut(stack[i]).unwrap().sprite_number =
    b.kind as usize * 8 + b.anim_offset as usize;
*/

struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Camera>();
        initialise_camera(world);

        world.register::<Player>();
        let mut transform = Transform::default();
        transform.set_xyz(250.0, 250.0, 0.0);
        let sprite_sheet_handle = load_sprite_sheet(world, "Cirno.png", "spritesheet.ron");
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
        };
        world
            .create_entity()
            .with(Player)
            .with(transform)
            .with(sprite_render)
            .build();
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
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

struct PlayerSystem(usize);

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>
    );

    fn run(&mut self, (player, mut sprite): Self::SystemData) {
        if let Some((_player, sprite)) = (&player, &mut sprite).join().next() {
            sprite.sprite_number = func(self.0, 3, 7);
            self.0 += 1;
        }

        fn func(n: usize, max_index: usize, repeat: usize) -> usize {
            n % (max_index * repeat) / repeat
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(DrawFlat2D::new())
    );
    let config = DisplayConfig::load("./examples/11_animation/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let bindings = Bindings::<String, String>::new();
    let input_bundle = InputBundle::new().with_bindings(bindings);

    let transform_bundle = TransformBundle::new();

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(input_bundle)?
        .with_bundle(transform_bundle)?
        .with(PlayerSystem(0), "player_system", &[]);

    Application::new("./examples/11_animation/", ExampleState, game_data)?.run();

    Ok(())
}
