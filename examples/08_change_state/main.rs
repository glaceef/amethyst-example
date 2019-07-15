// example/08_change_state/main.rs

use amethyst::{
    prelude::*,
    utils::application_root_dir,
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage,
    },
    input::is_key_down,
    winit::VirtualKeyCode,
};

use std::path::PathBuf;

struct LoadState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_image(world);
    }
}

struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("current state: Main");
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("current state: Main");
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(e) = event {
            if is_key_down(&e, VirtualKeyCode::Return) {
                return Trans::Push(Box::new(SubState));
            }
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

struct SubState;

impl SimpleState for SubState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("current state: Sub");
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(e) = event {
            if is_key_down(&e, VirtualKeyCode::Return) {
                return Trans::Pop;
            }
        }
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    // amethyst::start_logger(Default::default());

    let app_root = PathBuf::from(application_root_dir()).join("examples/08_change_state/");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
    );
    let config = DisplayConfig::load(app_root.join("config.ron"));
    let render_bundle = RenderBundle::new(pipe, Some(config));

    // let input_bundle = InputBundle::new().with_bindings(bindings);

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?;
        // .with_bundle(input_bundle)?

    Application::new(app_root, MainState, game_data)?.run();

    Ok(())
}

fn initialize_image(world: &mut World) {
    let sprite_sheet = load_sprite_sheet(world, "icon.png", "spritesheet.ron");
    let sprite_render = SpriteRender {
        sprite_sheet, sprite_number: 0,
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
