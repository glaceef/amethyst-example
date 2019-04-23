use amethyst::{
    prelude::*,
    ecs::prelude::{
        System,
        Write, Read
    },
    renderer::{
        DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage,
    },
    input::{
        is_key_down, Bindings, InputBundle, InputHandler
    },
    winit::{
        VirtualKeyCode, MouseButton
    },
};

use std::{
    collections::{
        HashMap, HashSet
    },
    iter::FromIterator,
};

#[derive(Default)]
struct Mouse {
    state: HashMap<MouseButton, bool>,
    press: HashSet<MouseButton>,
    release: HashSet<MouseButton>,
}
impl Mouse {
    fn get(&self, button: MouseButton) -> bool {
        match self.state.get(&button) {
            Some(true) => { true }
            _ => { false }
        }
    }

    fn get_down(&self, button: MouseButton) -> bool {
        self.press.contains(&button)
    }

    fn get_up(&self, button: MouseButton) -> bool {
        self.release.contains(&button)
    }
}

struct MouseButtonStateSystem;

impl<'s> System<'s> for MouseButtonStateSystem {
    type SystemData = (
        Write<'s, Mouse>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut mouse, input): Self::SystemData) {
        let iter = input.mouse_buttons_that_are_down();
        let down_buttons: HashSet<&MouseButton> = HashSet::from_iter(iter);

        mouse.press.clear();
        let mut iter = down_buttons.iter();
        while let Some(button) = iter.next() {
            match mouse.state.get(button) {
                Some(true) => {}
                _ => {
                    mouse.press.insert(**button);
                }
            }
        }

        let mut vec = vec![]; // 一度配列に退避させないとエラー
        for (button, state) in mouse.state.iter() {
            if *state {
                if !down_buttons.contains(button) {
                    vec.push(*button);
                }
            }
        }
        mouse.release.clear();
        for b in vec {
            mouse.release.insert(b);
        }

        for (_, state) in mouse.state.iter_mut() {
            *state = false;
        }

        let mut iter = down_buttons.iter();
        while let Some(button) = iter.next() {
            *mouse.state.entry(**button).or_insert(true) = true;
        }
    }
}

struct ClickSystem;

impl<'s> System<'s> for ClickSystem {
    type SystemData = Read<'s, Mouse>;

    fn run(&mut self, mouse: Self::SystemData) {
        if mouse.get_down(MouseButton::Left) {
            println!("press!");
        }
        if mouse.get_up(MouseButton::Left) {
            println!("release!");
        }
    }
}

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.add_resource(Mouse::default());
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

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
    );
    let config = DisplayConfig::load("./examples/10_mouse_getdown/config.ron");
    let render_bundle = RenderBundle::new(pipe, Some(config));

    let bindings = Bindings::<String, String>::new();
    let input_bundle = InputBundle::new().with_bindings(bindings);

    let game_data = GameDataBuilder::new()
        .with_bundle(render_bundle.with_sprite_sheet_processor())?
        .with_bundle(input_bundle)?
        .with(MouseButtonStateSystem, "mouse-button-state-system", &[])
        .with(ClickSystem, "click-system", &[]);

    let mut game = Application::new(
        "./examples/10_mouse_getdown/",
        ExampleState,
        game_data
    )?;

    game.run();

    Ok(())
}
