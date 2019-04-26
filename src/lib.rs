use amethyst::{
    prelude::*,
    ecs::prelude::{
        System,
        Write, Read,
    },
    core::transform::Transform,
    renderer::{
        Camera, Projection,
        SpriteSheet, SpriteSheetHandle, SpriteSheetFormat, PngFormat,
        Texture, TextureMetadata,
    },
    input::{
        InputHandler,
    },
    assets::{
        Loader, AssetStorage,
    },
    winit::{
        Event, WindowEvent, ElementState, MouseButton
    },
};

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, 500.0, 0.0, 500.0
        )))
        .with(transform)
        .build();
}

pub fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "icon.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_storage,
    )
}

pub fn is_mouse_down(event: &Event, mouse_button: MouseButton) -> bool {
    if let Event::WindowEvent { ref event, .. } = event { // refがないとmoveがおきる
        if let WindowEvent::MouseInput{ state, button, .. } = event {
            match (state, button) {
                (ElementState::Pressed, b) if *b == mouse_button => {
                    return true;
                }
                _ => {}
            }
        }
    }
    false
}

pub mod mouse {
    use super::*;
    use std::collections::{
        HashMap, HashSet
    };

    #[derive(Default)]
    pub struct Mouse {
        pub x: f32,
        pub y: f32,
        pub mx: f32,
        pub my: f32,

        state: HashMap<MouseButton, bool>,
        press: HashSet<MouseButton>,
        release: HashSet<MouseButton>,
    }

    impl Mouse {
        pub fn new() -> Self {
            let mut state = HashMap::new();
                state.insert(MouseButton::Left,   false);
                state.insert(MouseButton::Right,  false);
                state.insert(MouseButton::Middle, false);
            Mouse {
                state,
                ..Default::default()
            }
        }

        pub fn get(&self, button: MouseButton) -> bool {
            match self.state.get(&button) {
                Some(true) => { true }
                _ => { false }
            }
        }

        pub fn get_down(&self, button: MouseButton) -> bool {
            self.press.contains(&button)
        }

        pub fn get_up(&self, button: MouseButton) -> bool {
            self.release.contains(&button)
        }

        fn position_update(&mut self, input: &InputHandler<String, String>) {
            if let Some(mouse_pos) = input.mouse_position() {
                let (x, y) = (mouse_pos.0 as f32, 500.0 - mouse_pos.1 as f32);
                self.mx = x - self.x;
                self.my = y - self.y;
                self.x = x;
                self.y = y;
            }
        }

        fn state_update(&mut self, input: &InputHandler<String, String>) {
            let down_buttons: HashSet<&MouseButton>
                = input.mouse_buttons_that_are_down().collect();
            self.press.clear();
            self.release.clear();
            // カスタムボタンは未実装
            for button in &[MouseButton::Left, MouseButton::Middle, MouseButton::Right] {
                match (self.state[button], down_buttons.contains(button)) {
                    (false, true) => {
                        self.press.insert(*button);
                        *self.state.get_mut(button).unwrap() = true;
                    }
                    (true, false) => {
                        self.release.insert(*button);
                        *self.state.get_mut(button).unwrap() = false;
                    }
                    _ => {}
                }
            }
        }
    }

    pub struct MouseSystem;

    impl<'s> System<'s> for MouseSystem {
        type SystemData = (
            Write<'s, Mouse>,
            Read<'s, InputHandler<String, String>>
        );

        fn run(&mut self, (mut mouse, input): Self::SystemData) {
            mouse.position_update(&input);
            mouse.state_update(&input);
        }
    }
}
