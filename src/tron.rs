use amethyst::{
    assets::{AssetStorage, Handle, Loader, PrefabLoader, RonFormat},
    controls::HideCursor,
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::Entity,
    ecs::prelude::{Component, DenseVecStorage},
    input::{is_key_down, is_mouse_button_down},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    winit::{MouseButton, VirtualKeyCode},
};

use crate::components::Player;
use crate::components::FlyCamera;

#[derive(Default)]
pub struct Tron;

impl SimpleState for Tron {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        Player::init(world);
        FlyCamera::init(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;

        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = false;
            } else if is_mouse_button_down(&event, MouseButton::Left) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = true;
            }
        }
        Trans::None
    }
    
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
