use macroquad::prelude::*;

#[macroquad::main("Nested Triangle Zoom")]
async fn main() {
    let mut color = WHITE;
    let mut angle = 0.0;
    let mut size = 2.0;

    loop {
        clear_background(BLACK);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        angle += 0.3 * get_frame_time(); // Rotate in radians
        let degrees = angle.to_degrees();

        draw_nested_triangle(center_x, center_y, size, degrees, WHITE, 0);



        size *= 1.02;

        next_frame().await;
    }
}

fn draw_nested_triangle(x: f32, y: f32, mut size: f32, mut angle: f32, mut color: Color, mut iteration: i32) {

    while size > 2.0 && iteration < 128 {
        draw_poly(x, y, 3, size, angle, color);

        // Prepare for next inner triangle
        size /= 2.0;
        angle += 60.0;
        color = if color == WHITE { BLACK } else { WHITE };
        iteration += 1;
    }
}