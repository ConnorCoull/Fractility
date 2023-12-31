use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

// #[wasm_bindgen]
// pub fn draw_dot(canvas: &HtmlCanvasElement, x: f64, y: f64) {
//     let context = get_canvas_context(&canvas);

//     context.begin_path();
//     context.arc(x, y, 5.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
//     context.fill();
// }

#[wasm_bindgen]
pub fn draw_fractal(x: f64, y: f64, length: f64, length_scalar: f64, angle: f64, angle_scalar: f64, iterations: u32, canvas: &HtmlCanvasElement, thickness: f64, thickness_scalar: f64) {
    if iterations == 0 {
        return;
    }

    let angle_radians = angle * std::f64::consts::PI / 180.00;

    let x2 = x + (length * length_scalar) * angle_radians.cos();
    let y2 = y + (length * length_scalar) * angle_radians.sin();

    draw_line(&canvas, x, y, x2, y2, thickness);

    draw_fractal(x2, y2, length * length_scalar, length_scalar, -angle*angle_scalar % 360.00, angle_scalar, iterations - 1, &canvas, thickness*thickness_scalar, thickness_scalar);
    draw_fractal(x2, y2, length * length_scalar, length_scalar, angle*angle_scalar % 360.00, angle_scalar, iterations - 1, &canvas, thickness*thickness_scalar, thickness_scalar);
}

#[wasm_bindgen]
pub fn draw_line(canvas: &HtmlCanvasElement, x: f64, y: f64, x2: f64, y2: f64, thickness: f64) {
    // Convert the inputs to coordinates for the line
    // Then use WebGPU to draw the line on the canvas
    let context = get_canvas_context(&canvas);

    context.begin_path();
    context.move_to(x, y);
    context.line_to(x2, y2);
    context.set_line_width(thickness);
    context.set_line_cap("round");
    context.stroke();
}

// #[wasm_bindgen]
// pub fn draw_line_given_one_point(canvas: &HtmlCanvasElement, x: f64, y: f64, angle: f64, length: f64) {
//     let angle_radians = angle * std::f64::consts::PI / 180.00;

//     let x2 = x + length * angle_radians.cos();
//     let y2 = y + length * angle_radians.sin();

//     draw_line(&canvas, x, y, x2, y2, 1.0);
// }

#[wasm_bindgen]
pub fn clear(canvas: &HtmlCanvasElement) {
    let context = get_canvas_context(&canvas);

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    context.clear_rect(0.0, 0.0, width, height);
    context.clear_rect(0.0, 0.0, -width, -height);
    context.clear_rect(0.0, 0.0, -width, height);
    context.clear_rect(0.0, 0.0, width, -height);
}

#[wasm_bindgen]
pub fn get_canvas_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}
