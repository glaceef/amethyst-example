// 画像の選択の仕方はパス名だけなのか？
// 画像変数から読み込むことはできないのか？
// -> spritesheetから設定する？

use amethyst::{
    prelude::*,
    core::transform::{
        TransformBundle, Transform
    },
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage
    },
    input::{
        is_key_down, Bindings, InputBundle, InputHandler
    },
    ecs::prelude::{
        System,
        Component, DenseVecStorage,
        Read, ReadStorage, WriteStorage,
        Join
    },
    winit::{
        Event, WindowEvent, ElementState, MouseButton, VirtualKeyCode
    },
};

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
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

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?
        .with_bundle(input_bundle)?;

    Application::new("./examples/11_animation/", ExampleState, game_data)?.run();

    Ok(())
}
