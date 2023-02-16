use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Dialogue maker")
        .build();

    rl.set_target_fps(60);

    let mut cam = Camera2D::default();
    cam.zoom = 1.;
    cam.offset = Vector2 {
        x: 1280. / 2.,
        y: 720. / 2.,
    };

    // Raylib in rust for some reason doesn't provide a get_mouse_delta funcion, so the program will do it ny itself
    let mut last_mouse_pos = rl.get_mouse_position();

    while !rl.window_should_close() {
        // ===== UPDATE =====

        // Adapted from 2d camera_mouse_zoom found at: https://www.raylib.com/examples.html
        if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            let mut delta = rl.get_mouse_position() - last_mouse_pos;
            delta.scale(-1. / cam.zoom);

            cam.target = cam.target + delta;
        }
        last_mouse_pos = rl.get_mouse_position();

        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0. {
            let zoom_increment = 0.125;

            cam.zoom += wheel * zoom_increment;
        }

        let tlp = rl.get_screen_to_world2D(Vector2 { x: 0., y: 0. }, cam);
        let trp = rl.get_screen_to_world2D(Vector2 { x: 1280., y: 0. }, cam);
        let blp = rl.get_screen_to_world2D(Vector2 { x: 0., y: 720. }, cam);
        let brp = rl.get_screen_to_world2D(Vector2 { x: 1280., y: 720. }, cam);
        // ===== DRAW =====

        let mut d = rl.begin_drawing(&thread);

        let mut new_d = d.begin_mode2D(cam);

        new_d.clear_background(Color::WHITE);

        new_d.draw_circle(tlp.x as i32, tlp.y as i32, 5., Color::GREEN);
        new_d.draw_circle(trp.x as i32, trp.y as i32, 5., Color::PINK);
        new_d.draw_circle(blp.x as i32, blp.y as i32, 5., Color::BLUE);
        new_d.draw_circle(brp.x as i32, brp.y as i32, 5., Color::RED);

        //new_d.draw_rectangle(0, 0, 1280, 720, Color::RED);
        //new_d.draw_rectangle(10, 10, 1260, 700, Color::WHITE);
        new_d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
