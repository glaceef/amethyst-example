use amethyst::{
    input::is_key_down,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    winit::VirtualKeyCode,
};

struct Example;

impl SimpleState for Example {
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

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // Pipeline::build() -> PipelineBuilder::new()
    // Pipeline::build()でも、直接PipelineBuilder::new()を呼んでもどっちでもよい。
    // PipelineBuilder::default() もあり。
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0) // (color_val, depth_val)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );
    // with_stage() や with_target() でチェーンしていく。

    let config = DisplayConfig::load("./examples/01_create_window/config.ron");
    // DisplayConfig::default() もしくは ::from(wb: WindowBuilder)

    let bundle = RenderBundle::new(pipe, Some(config));
    let game_data = GameDataBuilder::new()// or default()
        .with_bundle(bundle)?;
    let mut game = Application::new(
        // Path, State, DataInit
        "./", Example, game_data
    )?;
    // ここのExampleは、構造体の生成。ユニット構造体なためこれだけ。
    // Example{ ... }という書き方はしない。
    // DataInitは、() もしくは GameDataBuilder に実装されている。

    game.run();

    Ok(())
}
