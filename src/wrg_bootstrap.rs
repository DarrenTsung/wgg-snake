//! HACK: Do not modify this file! It is just used to export classes / types from wasm-rgame.
use wasm_bindgen::prelude::*;

use std::ops::{Deref, DerefMut};
use wasm_rgame;

macro_rules! deref_inner {
    ($outer_type:path, $inner_type:path) => {
        impl Deref for $outer_type {
            type Target = $inner_type;

            fn deref(&self) -> &$inner_type {
                &self.inner
            }
        }

        impl DerefMut for $outer_type {
            fn deref_mut(&mut self) -> &mut $inner_type {
                &mut self.inner
            }
        }
    };
}

#[wasm_bindgen]
pub struct Application {
    inner: wasm_rgame::Application,
}

#[wasm_bindgen]
impl Application {
    pub fn new() -> Application {
        Application { inner: wasm_rgame::Application::new(), }
    }

    pub fn canvas_properties_ptr(&self) -> *const u32 { self.inner.canvas_properties_ptr() }
    pub fn keys_ptr(&self) -> *const wasm_rgame::KeyCodeState { self.inner.keys_ptr() }
    pub fn mouse_events_ptr(&self) -> *const wasm_rgame::MouseEvent { self.inner.mouse_events_ptr() }
    pub fn tick(&mut self, graphics: &mut Graphics, delta_s: f64) { self.inner.tick(graphics, delta_s); }
}

deref_inner!(Application, wasm_rgame::Application);

#[wasm_bindgen]
pub struct Graphics {
    inner: wasm_rgame::Graphics,
}

#[wasm_bindgen]
impl Graphics {
    pub fn new() -> Graphics {
        Graphics { inner: wasm_rgame::Graphics::new() }
    }

    pub fn draw_rects_ptr(&self) -> *const wasm_rgame::DrawRect { self.inner.draw_rects_ptr() }
    pub fn draw_rects_len(&self) -> usize { self.inner.draw_rects_len() }
    pub fn draw_action_colors_ptr(&self) -> *const wasm_rgame::DrawActionColor { self.inner.draw_action_colors_ptr() }
    pub fn draw_action_colors_len(&self) -> usize { self.inner.draw_action_colors_len() }
    pub fn strings_ptr(&self) -> *const u8 { self.inner.strings_ptr() }
    pub fn string_properties_ptr(&self) -> *const wasm_rgame::StringProperties { self.inner.string_properties_ptr() }
    pub fn string_properties_len(&self) -> usize { self.inner.string_properties_len() }

    pub fn reset(&mut self) { self.inner.reset() }
}

deref_inner!(Graphics, wasm_rgame::Graphics);
