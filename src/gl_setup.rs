use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

pub fn initialize_webgl_context() ->  Result<WebGlRenderingContext, JsValue> {
  let window = window().unwrap();
  let document = window.document().unwrap();
  let canvas  = document.get_element_by_id("rustCanvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

  gl.enable(GL::BLEND);
  gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
  gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA
  Ok(gl)
}