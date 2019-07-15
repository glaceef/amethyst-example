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
    input::is_key_down,
    utils::application_root_dir,
    ecs::prelude::{
        Entity,
        System, WriteStorage, ReadExpect
    },
    winit::VirtualKeyCode,
};

use amethyst_test::{
    TransformExt,
    initialise_camera,
    load_sprite_sheet
};

use std::path::PathBuf;

struct CharaEntity(Entity);

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world, [500.0, 500.0]);
        initialise_character(world);
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

struct CharaSpriteSystem(usize);

impl<'s> System<'s> for CharaSpriteSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, CharaEntity>
    );

    fn run(&mut self, (mut sprites, player): Self::SystemData) {
        match sprites.get_mut(player.0) {
            Some(sprite) => {
                sprite.sprite_number = num_extend(self.0, 4, 10);
                self.0 += 1;
            }
            _ => {}
        }

        fn num_extend(n: usize, size: usize, repeat: usize) -> usize {
            n % (size * repeat) / repeat
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

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(transform_bundle)?
        .with(CharaSpriteSystem(0), "chara_sprite_system", &[]);

    Application::new(app_root, ExampleState, game_data)?.run();

    Ok(())
}

fn initialise_character(world: &mut World) {
    let sprite_sheet = load_sprite_sheet(world, "dot_reimu.png", "spritesheet.ron");
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };

    let scale = 2.0;
    let mut transform = Transform::from_xyz(250.0, 250.0, 0.0);
        transform.set_scale(scale, scale, 1.0);

    let entity = world.create_entity()
        .with(sprite_render)
        .with(transform)
        .build();
    world.add_resource(CharaEntity(entity));
}
