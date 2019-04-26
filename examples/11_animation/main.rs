use amethyst::{
    prelude::*,
    core::transform::{
        TransformBundle, Transform
    },
    renderer::{
        Pipeline, Stage, DrawFlat2D, ColorMask, DepthMode, ALPHA,
        DisplayConfig, RenderBundle,
        Camera,
        SpriteRender
    },
    input::is_key_down,
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
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite)
            ))
    );
    let config = DisplayConfig::load("./examples/11_animation/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let transform_bundle = TransformBundle::new();

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(transform_bundle)?
        .with(PlayerTextureSystem(0), "player_texture_system", &[]);

    Application::new("./examples/11_animation/", ExampleState, game_data)?.run();

    Ok(())
}
