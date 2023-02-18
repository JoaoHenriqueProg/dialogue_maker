use raylib::prelude::*;

trait CanvasCard {
    fn set_pos(&mut self, new_pos: Vector2);
    fn get_pos(&self) -> Vector2;
    fn set_size(&mut self, new_size: Vector2);
    fn get_size(&self) -> Vector2;
    fn update(
        &mut self,
        rl: &RaylibHandle,
        last_mouse_pos: &mut Vector2,
        cur_zoom: f32,
        cam: &Camera2D,
    ) {
        // Adapted from 2d camera_mouse_zoom found at: https://www.raylib.com/examples.html
        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse_x = rl.get_screen_to_world2D(rl.get_mouse_position(), cam).x as i32;
            let mouse_y = rl.get_screen_to_world2D(rl.get_mouse_position(), cam).y as i32;

            let pos_x = self.get_pos().x as i32;
            let pos_y = self.get_pos().y as i32;

            if mouse_x > pos_x && mouse_x < (pos_x + self.get_size().x as i32) {
                if mouse_y + 12 > pos_y && mouse_y - 12 < pos_y {
                    let mut delta = *last_mouse_pos - rl.get_mouse_position();
                    delta.scale(-1. / cur_zoom);

                    self.set_pos(self.get_pos() + delta);
                }
            }
        }
    }

    fn is_being_moved(&self) -> bool {
        return false;
    }

    fn draw(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        self.draw_node_bg(d);
    }

    fn draw_node_bg(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        let corner_radius = 10;

        d.draw_circle(
            self.get_pos().x as i32,
            self.get_pos().y as i32,
            12.,
            Color::BROWN,
        );

        d.draw_rectangle(
            self.get_pos().x as i32,
            self.get_pos().y as i32 - 12,
            self.get_size().x as i32,
            24,
            Color::BROWN,
        );

        d.draw_circle(
            self.get_pos().x as i32 + self.get_size().x as i32,
            self.get_pos().y as i32,
            12.,
            Color::BROWN,
        );

        d.draw_circle(
            self.get_pos().x as i32 + corner_radius,
            self.get_pos().y as i32 + corner_radius,
            corner_radius as f32,
            Color::SKYBLUE,
        );
        d.draw_circle(
            self.get_pos().x as i32 + self.get_size().x as i32 - corner_radius,
            self.get_pos().y as i32 + corner_radius,
            corner_radius as f32,
            Color::SKYBLUE,
        );
        d.draw_circle(
            self.get_pos().x as i32 + corner_radius,
            self.get_pos().y as i32 + self.get_size().y as i32 - corner_radius,
            corner_radius as f32,
            Color::SKYBLUE,
        );
        d.draw_circle(
            self.get_pos().x as i32 + self.get_size().x as i32 - corner_radius,
            self.get_pos().y as i32 + self.get_size().y as i32 - corner_radius,
            corner_radius as f32,
            Color::SKYBLUE,
        );

        d.draw_rectangle(
            self.get_pos().x as i32 + corner_radius,
            self.get_pos().y as i32,
            self.get_size().x as i32 - corner_radius * 2,
            self.get_size().y as i32,
            Color::SKYBLUE,
        );
        d.draw_rectangle(
            self.get_pos().x as i32,
            self.get_pos().y as i32 + corner_radius,
            self.get_size().x as i32,
            self.get_size().y as i32 - corner_radius * 2,
            Color::SKYBLUE,
        );
    }
}

struct DialogueCard {
    pos: Vector2,
    size: Vector2,
}
struct OptionsCard {
    pos: Vector2,
    size: Vector2,
}
struct ConditionalCard {
    pos: Vector2,
    size: Vector2,
}
struct SetFlagCard {
    pos: Vector2,
    size: Vector2,
}

impl CanvasCard for DialogueCard {
    fn set_pos(&mut self, new_pos: Vector2) {
        self.pos = new_pos;
    }
    fn get_pos(&self) -> Vector2 {
        self.pos
    }
    fn set_size(&mut self, new_size: Vector2) {
        self.size = new_size;
    }
    fn get_size(&self) -> Vector2 {
        self.size
    }
}
impl CanvasCard for OptionsCard {
    fn set_pos(&mut self, new_pos: Vector2) {
        self.pos = new_pos;
    }
    fn get_pos(&self) -> Vector2 {
        self.pos
    }
    fn set_size(&mut self, new_size: Vector2) {
        self.size = new_size;
    }
    fn get_size(&self) -> Vector2 {
        self.size
    }
}
impl CanvasCard for ConditionalCard {
    fn set_pos(&mut self, new_pos: Vector2) {
        self.pos = new_pos;
    }
    fn get_pos(&self) -> Vector2 {
        self.pos
    }
    fn set_size(&mut self, new_size: Vector2) {
        self.size = new_size;
    }
    fn get_size(&self) -> Vector2 {
        self.size
    }
}
impl CanvasCard for SetFlagCard {
    fn set_pos(&mut self, new_pos: Vector2) {
        self.pos = new_pos;
    }
    fn get_pos(&self) -> Vector2 {
        self.pos
    }
    fn set_size(&mut self, new_size: Vector2) {
        self.size = new_size;
    }
    fn get_size(&self) -> Vector2 {
        self.size
    }
}

struct CanvasScene {
    cam: Camera2D,
    nodes: Vec<Box<dyn CanvasCard>>,
}

impl CanvasScene {
    pub fn update(&mut self, rl: &RaylibHandle, last_mouse_pos: &mut Vector2) {
        for i in self.nodes.iter_mut() {
            i.update(rl, last_mouse_pos, self.cam.zoom, &self.cam);
        }

        // Adapted from 2d camera_mouse_zoom found at: https://www.raylib.com/examples.html
        if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            let mut delta = rl.get_mouse_position() - *last_mouse_pos;
            delta.scale(-1. / self.cam.zoom);

            self.cam.target = self.cam.target + delta;
        }
        *last_mouse_pos = rl.get_mouse_position();

        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0. {
            let zoom_increment = 0.125;

            self.cam.zoom += wheel * zoom_increment;
        }
    }

    pub fn draw_background(
        &self,
        d: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        top_left: Vector2,
        bottom_right: Vector2,
    ) {
        for i in (top_left.x - 50.) as i32 / 50..(bottom_right.x + 50.) as i32 / 50 {
            d.draw_line(
                i * 50,
                top_left.y as i32,
                i * 50,
                bottom_right.y as i32,
                Color::BLACK,
            );
        }
        for i in (top_left.y - 50.) as i32 / 50..(bottom_right.y + 50.) as i32 / 50 {
            d.draw_line(
                top_left.x as i32,
                i * 50,
                bottom_right.x as i32,
                i * 50,
                Color::BLACK,
            );
        }

        d.draw_line(0, i32::MIN, 0, i32::MAX, Color::ORANGE);
        d.draw_line(i32::MIN, 0, i32::MAX, 0, Color::ORANGE);
    }

    pub fn draw(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        for i in &self.nodes {
            i.draw(d);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Dialogue maker")
        .build();

    rl.set_target_fps(60);

    let mut canvas_scene = CanvasScene {
        cam: Camera2D {
            offset: Vector2 {
                x: 1280. / 2.,
                y: 720. / 2.,
            },
            zoom: 1.,
            target: Vector2::default(),
            rotation: 0.,
        },
        nodes: vec![Box::new(DialogueCard {
            pos: Vector2 { x: 800., y: 200. },
            size: Vector2 { x: 600., y: 400. },
        })],
    };

    // Raylib in rust for some reason doesn't provide a get_mouse_delta funcion, so the program will do it ny itself
    let mut last_mouse_pos = rl.get_mouse_position();

    while !rl.window_should_close() {
        // ===== UPDATE =====
        canvas_scene.update(&rl, &mut last_mouse_pos);

        let tlp = rl.get_screen_to_world2D(Vector2 { x: 0., y: 0. }, canvas_scene.cam);
        let trp = rl.get_screen_to_world2D(Vector2 { x: 1280., y: 0. }, canvas_scene.cam);
        let blp = rl.get_screen_to_world2D(Vector2 { x: 0., y: 720. }, canvas_scene.cam);
        let brp = rl.get_screen_to_world2D(Vector2 { x: 1280., y: 720. }, canvas_scene.cam);
        // ===== DRAW =====

        let mut d = rl.begin_drawing(&thread);

        let mut new_d = d.begin_mode2D(canvas_scene.cam);
        new_d.clear_background(Color::WHITE);

        canvas_scene.draw_background(&mut new_d, tlp.clone(), brp.clone());
        canvas_scene.draw(&mut new_d);

        new_d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        new_d.draw_fps(tlp.x as i32, tlp.y as i32);
    }
}
