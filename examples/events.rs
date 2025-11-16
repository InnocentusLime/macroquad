use macroquad::prelude::*;

#[macroquad::main("Events")]
async fn main() {
    let y_off = 32.0;
    loop {
        clear_background(WHITE);

        let (mouse_x, mouse_y) = mouse_position();
        draw_text(&format!("Mouse position: {mouse_x} {mouse_y}"), 0.0, y_off * 1.0, 16.0, BLACK);

        let (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
        draw_text(&format!("Mouse wheel x: {mouse_wheel_x}"), 0.0, y_off * 2.0, 16.0, BLACK);
        draw_text(&format!("Mouse wheel y: {mouse_wheel_y}"), 0.0, y_off * 3.0, 16.0, BLACK);

        let key = get_last_key_pressed();
        draw_text("Pressed keyboard keys", 0.0, y_off * 5.0, 16.0, BLACK);
        draw_text(&format!("{key:?}"), 0.0, y_off * 6.0, 16.0, BLACK);

        draw_text("Pressed mouse keys", 0.0, y_off * 7.0, 16.0, BLACK);
        let l_down = is_mouse_button_down(MouseButton::Left);
        let r_down = is_mouse_button_down(MouseButton::Right);
        let m_down = is_mouse_button_down(MouseButton::Middle);
        draw_text(&format!("left:{l_down}"), 0.0, y_off * 8.0, 16.0, BLACK);
        draw_text(&format!("right:{r_down}"), 0.0, y_off * 9.0, 16.0, BLACK);
        draw_text(&format!("middle:{m_down}"), 0.0, y_off * 10.0, 16.0, BLACK);
    
        next_frame().await;
    }
}
