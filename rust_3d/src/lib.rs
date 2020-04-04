// extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

mod gl_setup;
mod programs;
mod shaders;
mod common_funcs;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct DougsClient {
    gl: WebGlRenderingContext,
    program_color_2d: programs::color_2d::Color2D,
}

#[wasm_bindgen]
impl DougsClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();
        Self {
            gl,
            program_color_2d: programs::Color2d::new(&gl)
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        // log("Update was hit");
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT|GL::DEPTH_BUFFER_BIT);
        self.program_color_2d.render(
            &gl,
            0.,
            10.,
            0.,
            10.,
            10.,
            10.,
        )
    }
}
