use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Dialogue maker")
        .build();
     
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::WHITE);
        d.draw_rectangle(0, 0, 1280, 720, Color::RED);
        d.draw_rectangle(10, 10, 1260, 700, Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}