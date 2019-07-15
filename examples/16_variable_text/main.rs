// examples/16_variable_text/main.rs

use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    window::{
        ScreenDimensions, Window, WindowBundle
    },
    renderer::{
        rendy::{
            factory::Factory,
            graph::{
                render::{
                    RenderGroupDesc, SubpassBuilder
                },
                GraphBuilder,
            },
            hal::{
                format::Format, image
            },
        },
        GraphCreator, RenderingSystem,
        types::DefaultBackend,
    },
    input::{
        StringBindings,
        is_key_down
    },
    ui::{
        UiBundle,
        FontAsset, TtfFormat, UiText, UiTransform, Anchor,
        DrawUiDesc,
    },
    assets::{
        Loader, AssetStorage
    },
    utils::{
        fps_counter::{
            FPSCounterBundle, FPSCounter
        },
        application_dir
    },
    ecs::prelude::{
        Resources,
        Entity,
        System, Read, WriteStorage, ReadExpect,
    },
    shred::SystemData,
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

struct FpsDisplaySystem;

impl<'s> System<'s> for FpsDisplaySystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        ReadExpect<'s, TextEntity>,
        Read<'s, FPSCounter>
    );

    fn run(&mut self, (mut ui_texts, entity, fps_counter): Self::SystemData) {
        if let Some(text) = ui_texts.get_mut(entity.0) {
            text.text = format!("fps: {:.2}", fps_counter.sampled_fps());
        }
    }
}

struct TextEntity(Entity);

fn main() -> amethyst::Result<()> {
    // amethyst::start_logger(Default::default());

    let app_root = application_dir("examples/16_variable_text")?;

    let transform_bundle = TransformBundle::new(); // UiTransformを利用するために必要
    let ui_bundle = UiBundle::<DefaultBackend, StringBindings>::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(app_root.join("config.ron")))?
        .with_bundle(transform_bundle)?
        .with_bundle(ui_bundle)?
        .with_bundle(FPSCounterBundle)?
        .with(FpsDisplaySystem, "fps-display-system", &[])
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            ExampleGraph::default(),
        ));

    Application::new(app_root, ExampleState, game_data)?.run();

    Ok(())
}

fn initialise_text(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "square.ttf",
        TtfFormat,
        (),
        &world.read_resource::<AssetStorage<FontAsset>>(),
    );

    let mut text = UiText::new(
        font,
        String::from(""),
        [0.0, 0.0, 0.0, 1.0],
        24.0
    );
    text.align = Anchor::TopLeft;

    let ui_transform = UiTransform::new(
        String::from("fps"),
        Anchor::TopLeft, // parent(この場合ウィンドウ)のどこを基準にするか
        Anchor::TopLeft, // UiTransformのどの部分を上に合わせるか
        0.0, 0.0, 0.0, // x, y, z
        500.0, 24.0, // width, height
    );

    let entity = world.create_entity()
        .with(text)
        .with(ui_transform)
        .build();

    world.add_resource(TextEntity(entity));
}

#[derive(Default)]
struct ExampleGraph {
    dimensions: Option<ScreenDimensions>,
    surface_format: Option<Format>,
    dirty: bool,
}

impl GraphCreator<DefaultBackend> for ExampleGraph {
    fn rebuild(&mut self, res: &Resources) -> bool {
        // Rebuild when dimensions change, but wait until at least two frames have the same.
        let new_dimensions = res.try_fetch::<ScreenDimensions>();
        use std::ops::Deref;
        if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
            self.dirty = true;
            self.dimensions = new_dimensions.map(|d| d.clone());
            return false;
        }
        return self.dirty;
    }

    fn builder(
        &mut self,
        factory: &mut Factory<DefaultBackend>,
        res: &Resources,
    ) -> GraphBuilder<DefaultBackend, Resources> {
        use amethyst::renderer::rendy::{
            graph::present::PresentNode,
            hal::command::{ClearDepthStencil, ClearValue},
        };

        self.dirty = false;

        let window = <ReadExpect<'_, Window>>::fetch(res);
        let surface = factory.create_surface(&window);
        // cache surface format to speed things up
        let surface_format = *self
            .surface_format
            .get_or_insert_with(|| factory.get_surface_format(&surface));
        let dimensions = self.dimensions.as_ref().unwrap();
        let window_kind =
            image::Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);

        let mut graph_builder = GraphBuilder::new();
        let color = graph_builder.create_image(
            window_kind,
            1,
            surface_format,
            Some(ClearValue::Color([0.34, 0.36, 0.52, 1.0].into())),
        );

        let depth = graph_builder.create_image(
            window_kind,
            1,
            Format::D32Sfloat,
            Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        );

        let ui = graph_builder.add_node(
            SubpassBuilder::new()
                .with_group(DrawUiDesc::new().builder())
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        let _present = graph_builder
            .add_node(PresentNode::builder(factory, surface, color).with_dependency(ui));

        graph_builder
    }
}
