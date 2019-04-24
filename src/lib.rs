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

    pub struct Mouse {
        pub x: f32,
        pub y: f32,
        pub mx: f32,
        pub my: f32,

        // ここらへんは公開範囲を限定してpubにする？
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
                ..Default::default
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
    }

    pub struct MouseSystem;

    impl<'s> System<'s> for MouseSystem {
        type SystemData = (
            Write<'s, Mouse>,
            Read<'s, InputHandler<String, String>>
        );

        fn run(&mut self, (mut mouse, input): Self::SystemData) {
            if let Some(mouse_pos) = input.mouse_position() {
                let (x, y) = (mouse_pos.0 as f32, 500.0 - mouse_pos.1 as f32);
                mouse.mx = x - mouse.x;
                mouse.my = y - mouse.y;
                mouse.x = x;
                mouse.y = y;
            }

            // --------------
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
            // -------------------
        }
    }
}
