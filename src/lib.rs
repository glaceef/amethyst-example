use amethyst::{
    prelude::*,
    core::transform::Transform,
    renderer::{
        Camera, Projection,
        SpriteSheet, SpriteSheetHandle, SpriteSheetFormat, PngFormat,
        Texture, TextureMetadata,
    },
    assets::{
        Loader, AssetStorage,
    },
    winit::{
        Event, WindowEvent, ElementState, MouseButton, VirtualKeyCode
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

pub fn is_mouse_down(event: &Event, button: MouseButton) -> bool {
    if let Event::WindowEvent { ref event, .. } = event { // refがないとmoveがおきる
        if let WindowEvent::MouseInput{ state, button, .. } = event {
            match (state, button) {
                (ElementState::Pressed, b) if b == button => {
                    return true;
                }
                _ => {}
            }
        }
    }
    false
}
