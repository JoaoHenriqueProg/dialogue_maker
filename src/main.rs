use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Dialogue maker")
        .build();

    rl.set_target_fps(60);

    let mut cam = Camera2D::default();
    cam.zoom = 1.;

    // Raylib in rust for some reason doesn't provide a get_mouse_delta funcion, so the program will do it ny itself
    let mut last_mouse_pos = rl.get_mouse_position();

    while !rl.window_should_close() {
        // ===== UPDATE =====
        
        // Adapted from 2d camera_mouse_zoom found at: https://www.raylib.com/examples.html
        if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            let mut delta = rl.get_mouse_position() - last_mouse_pos;
            delta.scale(-1./cam.zoom);

            cam.target = cam.target + delta;
        }
        last_mouse_pos = rl.get_mouse_position();
        
        
        
        // ===== DRAW =====

        let mut d = rl.begin_drawing(&thread);

        let mut new_d = d.begin_mode2D(cam);

        new_d.clear_background(Color::WHITE);
        new_d.draw_rectangle(0, 0, 1280, 720, Color::RED);
        new_d.draw_rectangle(10, 10, 1260, 700, Color::WHITE);
        new_d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
