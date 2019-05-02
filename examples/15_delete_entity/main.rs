// examples/15_delete_entity/main.rs

use amethyst::{
    prelude::*,
    renderer::{
        Pipeline, Stage, DrawFlat2D,
        DisplayConfig,
        RenderBundle,
    },
    input::is_key_down,
    ecs::{
        prelude::{
            Component, DenseVecStorage,
            System, ReadStorage, WriteStorage, Join
        },
        Entities
    },
    winit::{
        VirtualKeyCode, MouseButton
    }
};

use amethyst_test::is_mouse_down;

struct Item(u32);

impl Component for Item {
    type Storage = DenseVecStorage<Self>;
}

struct ExampleState(u32);

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Item>();
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(e) = event {
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
            if is_mouse_down(&e, MouseButton::Left) {
                data.world.create_entity()
                    .with(Item(self.0))
                    .build();
                self.0 += 1;
            }
            if is_mouse_down(&e, MouseButton::Right) {
                let (entities, mut items): (Entities, WriteStorage<Item>)
                    = data.world.system_data();
                let mut destroy = None;
                for (entity, item) in (&entities, &mut items).join() {
                    if item.0 == 0 {
                        destroy = Some(entity);
                    } else {
                        item.0 -= 1;
                    }
                }
                if let Some(entity) = destroy {
                    entities.delete(entity).unwrap();
                }
                if self.0 != 0 { self.0 -= 1; }
            }
        }
        Trans::None
    }
}

struct ItemSystem;

impl<'s> System<'s> for ItemSystem {
    type SystemData = ReadStorage<'s, Item>;

    fn run(&mut self, items: Self::SystemData) {
        let size = items.join().count();
        println!("size: {}", size);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(DrawFlat2D::new())
    );

    let config = DisplayConfig::load("./examples/15_delete_entity/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle)?
        .with(ItemSystem, "item-system", &[]);

    Application::new(
        "./examples/15_delete_entity/",
        ExampleState(0),
        game_data
    )?.run();

    Ok(())
}
