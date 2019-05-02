use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    renderer::{
        Pipeline, Stage, DrawFlat2D, DisplayConfig, RenderBundle,
    },
    input::is_key_down,
    ui::{
        DrawUi, UiBundle,
        FontAsset, TtfFormat, UiText, UiTransform, Anchor
    },
    assets::{
        Loader, AssetStorage
    },
    winit::VirtualKeyCode,
};

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_text(world);
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

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new())
    );
    let config = DisplayConfig::load("./examples/13_text/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    // UiTransformを利用するために必要
    let transform_bundle = TransformBundle::new();

    let ui_bundle = UiBundle::<String, String>::new();

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?
        // .with_bundle(transform_bundle)?
        .with_bundle(ui_bundle)?;

    Application::new(
        "./examples/13_text/",
        ExampleState,
        game_data
    )?.run();

    Ok(())
}

fn initialise_text(world: &mut World) {
    let font = {
        let loader = world.read_resource::<Loader>();
        let font_asset = world.read_resource::<AssetStorage<FontAsset>>();
        loader.load(
            "square.ttf",
            TtfFormat,
            Default::default(),
            (),
            &font_asset,
        )
    };

    let text = UiText::new(
        font,
        String::from("Sample Text"),
        [0.1, 0.3, 1.0, 1.0],
        30.0
    );

    let ui_transform = UiTransform::new(
        String::from("id"),
        Anchor::TopLeft,
        250.0, -250.0, 0.0, // x, y, z
        200.0, 30.0, // width, height
        0 // Tabキーでtab_orderの次に高い(もしくは同じで作成順次の)Uiにフォーカスを移動
    );

    world
        .create_entity()
        .with(text)
        .with(ui_transform)
        .build();
}
