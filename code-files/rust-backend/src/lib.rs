use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub fn draw_alternate_fractal(x: f32, y: f32, angle1: f32, angle2: f32, iterations: u8, branches: u8, start_length: f32, length_multiplier: f32, start_width: f32, width_multiplier: f32, canvas: &HtmlCanvasElement, color: &str) {
    if iterations == 0 {
        return;
    }
    
    let mut endpoints: Vec<(f32, f32)> = Vec::with_capacity(branches as usize);

    let angle1_radians = angle1 * std::f32::consts::PI / 180.00;
    let angle2_radians = angle2 * std::f32::consts::PI / 180.00;

    for i in 0..branches {
        let angle = angle1_radians + (angle2_radians * i as f32);

        let x2 = x + start_length * angle.cos();
        let y2 = y + start_length * angle.sin();

        draw_line(&canvas, x, y, x2, y2, start_width, color);

        endpoints.push((x2, y2));
    }

    for &(x, y) in &endpoints {
        draw_alternate_fractal(x, y, angle1, angle2, iterations - 1, branches, start_length * length_multiplier, length_multiplier, start_width * width_multiplier, width_multiplier, &canvas, &get_next_color(color));
    }
}


#[wasm_bindgen]
pub fn draw_line(canvas: &HtmlCanvasElement, x: f32, y: f32, x2: f32, y2: f32, thickness: f32, color: &str) {
    let context = get_canvas_context(&canvas);

    context.begin_path();
    context.move_to(x as f64, y as f64);
    context.line_to(x2 as f64, y2 as f64);
    context.set_line_width(thickness as f64);
    context.set_line_cap("round");
    context.set_stroke_style(&JsValue::from_str(color));
    context.stroke();
}

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

#[wasm_bindgen]
pub fn to_lower_case(string: String) -> String {
    string.to_lowercase().replace(" ", "_")
}

#[wasm_bindgen]
pub fn to_float(string: String) -> f32 {
    string.parse::<f32>().unwrap()
}

#[wasm_bindgen]
pub fn get_next_color(color: &str) -> String {
    let colors = vec!["#DE493E", "#F7A13E", "#F7E13E", "#A1F73E", "#3EF7C8", "#3E9BF7", "#A13EF7", "#F73EF7", "#F73E9B", "#F73E3E"]; // 10 is enough, are they balanced though?
    let index = colors.iter().position(|&c| c == color).unwrap();
    let next_index = (index + 1) % colors.len();
    colors[next_index].to_string()
}

#[wasm_bindgen]
pub fn get_canvas_height_up(canvas: &HtmlCanvasElement, percent: f32) -> f32 {
    canvas.height() as f32 * percent
}
