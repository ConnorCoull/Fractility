use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

// create a struct that can be used by wasm
#[wasm_bindgen]
pub struct Point {
    x: f64,
    y: f64
}

#[wasm_bindgen]
pub fn greet(name: String) -> String {
    // match case on lower case name
    match name.to_lowercase().as_str(){
        "" => "Hello, World!".to_string(),
        _ => format!("Hello, {}!", name)
    }
}

#[wasm_bindgen]
pub fn add(a: i32, b: String) -> i32 {
    match b.to_lowercase().as_str(){
        "" => a,
        _ => a + b.parse::<i32>().unwrap_or(0)
    }
}

#[wasm_bindgen]
pub fn draw_dot(canvas: HtmlCanvasElement, x: f64, y: f64) {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();
    context.arc(x, y, 5.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
    context.fill();
}

#[wasm_bindgen]
pub fn calculate_next_point(start_point: Point, length: u32, angle: f64) -> Point {
    let angle_in_radians = angle * std::f64::consts::PI / 180.0;
    Point {
        x: start_point.x + f64::from(length) * angle_in_radians.cos(),
        y: start_point.y + f64::from(length) * angle_in_radians.sin()
    }
}

// #[wasm_bindgen]
// pub fn drawLine(canvas: HtmlCanvasElement, input1: &str, input2: &str) {
//   // Convert the inputs to coordinates for the line
//   // Then use WebGPU to draw the line on the canvas
// }
