use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub fn draw_fractal(x: f64, y: f64, length: f64, length_scalar: f64, angle: f64, angle_scalar: f64, iterations: u32, canvas: &HtmlCanvasElement, thickness: f64, thickness_scalar: f64, color: &str) {
    if iterations == 0 {
        return;
    }

    let angle_radians = angle * std::f64::consts::PI / 180.00;

    let x2 = x + (length * length_scalar) * angle_radians.cos();
    let y2 = y + (length * length_scalar) * angle_radians.sin();

    draw_line(&canvas, x, y, x2, y2, thickness, color);

    let next_color = get_next_color(color);

    draw_fractal(x2, y2, length * length_scalar, length_scalar, -angle*angle_scalar % 360.00, angle_scalar, iterations - 1, &canvas, thickness*thickness_scalar, thickness_scalar, &next_color);
    draw_fractal(x2, y2, length * length_scalar, length_scalar, angle*angle_scalar % 360.00, angle_scalar, iterations - 1, &canvas, thickness*thickness_scalar, thickness_scalar, &next_color);
}

#[wasm_bindgen]
pub fn draw_alternate_fractal(x: f64, y: f64, angle1: f64, angle2: f64, iterations: u32, branches: u32, start_length: f64, length_multiplier: f64, start_width: f64, width_multiplier: f64, canvas: &HtmlCanvasElement, color: &str) {
    if iterations == 0 {
        return;
    }
    
    let mut endpoints: Vec<(f64, f64)> = Vec::new();

    let angle1_radians = angle1 * std::f64::consts::PI / 180.00;

    let x2 = x + start_length * angle1_radians.cos();
    let y2 = y + start_length * angle1_radians.sin();

    draw_line(&canvas, x, y, x2, y2, start_width, color);

    for i in 0..branches {
        let angle = (angle1 + angle2 * i as f64) % 360.00;
        let angle_radians = angle * std::f64::consts::PI / 180.00;

        let x2 = x + start_length * angle_radians.cos();
        let y2 = y + start_length * angle_radians.sin();

        draw_line(&canvas, x, y, x2, y2, start_width, color);

        endpoints.push((x2, y2));
    }

    for i in 0..endpoints.len() {
        let (x, y) = endpoints[i];
        draw_alternate_fractal(x, y, angle1, angle2, iterations - 1, branches, start_length * length_multiplier, length_multiplier, start_width * width_multiplier, width_multiplier, &canvas, &get_next_color(color));
    }
}


#[wasm_bindgen]
pub fn draw_line(canvas: &HtmlCanvasElement, x: f64, y: f64, x2: f64, y2: f64, thickness: f64, color: &str) {
    let context = get_canvas_context(&canvas);

    context.begin_path();
    context.move_to(x, y);
    context.line_to(x2, y2);
    context.set_line_width(thickness);
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
pub fn to_float(string: String) -> f64 {
    string.parse::<f64>().unwrap()
}

#[wasm_bindgen]
pub fn get_next_color(color: &str) -> String {
    let colors = vec!["#DE493E", "#F7A13E", "#F7E13E", "#A1F73E", "#3EF7C8", "#3E9BF7", "#A13EF7", "#F73EF7", "#F73E9B", "#F73E3E"]; // 10 is enough, are they balanced though?
    let index = colors.iter().position(|&c| c == color).unwrap();
    let next_index = (index + 1) % colors.len();
    colors[next_index].to_string()
}

#[wasm_bindgen]
pub fn get_canvas_height_up(canvas: &HtmlCanvasElement, percent: f64) -> f64 {
    canvas.height() as f64 * percent
}