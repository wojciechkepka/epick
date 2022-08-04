use crate::{
    app::{App, FrameCtx},
    save_to_clipboard,
};

use std::collections::HashMap;

pub type KeyBindingFunc = Box<dyn Fn(&mut App, &mut FrameCtx<'_>) + Send + Sync + 'static>;

pub struct KeyBinding {
    description: &'static str,
    str_key: &'static str,
    key: egui::Key,
    binding: KeyBindingFunc,
}

impl KeyBinding {
    pub fn str_key(&self) -> &'static str {
        self.str_key
    }

    pub fn description(&self) -> &'static str {
        self.description
    }

    pub fn key(&self) -> egui::Key {
        self.key
    }

    pub fn binding(&self) -> &KeyBindingFunc {
        &self.binding
    }
}

pub struct KeyBindings(HashMap<egui::Key, KeyBinding>);
impl KeyBindings {
    pub fn new(bindings: HashMap<egui::Key, KeyBinding>) -> Self {
        Self(bindings)
    }

    pub fn iter(&self) -> impl Iterator<Item = &KeyBinding> {
        self.0.values()
    }
}

pub fn default_keybindings() -> KeyBindings {
    KeyBindings(
        [
            (
                egui::Key::H,
                KeyBinding {
                    description: "toggle the side panel",
                    str_key: "h",
                    key: egui::Key::H,
                    binding: Box::new(|_, mut ctx| {
                        ctx.app.sidepanel.show = !ctx.app.sidepanel.show;
                    }),
                },
            ),
            (
                egui::Key::P,
                KeyBinding {
                    description: "pick a color from under the cursor",
                    str_key: "p",
                    key: egui::Key::P,
                    binding: Box::new(|app, ctx| {
                        app.picker.set_cur_color(ctx.app.cursor_pick_color);
                        if ctx.app.settings.auto_copy_picked_color {
                            let color = ctx.app.cursor_pick_color;
                            let _ = save_to_clipboard(app.clipboard_color(ctx, &color));
                        }
                    }),
                },
            ),
            (
                egui::Key::S,
                KeyBinding {
                    description: "save a color from under the cursor",
                    str_key: "s",
                    key: egui::Key::S,
                    binding: Box::new(|_, ctx| {
                        ctx.app
                            .palettes
                            .current_mut()
                            .palette
                            .add(ctx.app.cursor_pick_color);
                    }),
                },
            ),
        ]
        .into(),
    )
}