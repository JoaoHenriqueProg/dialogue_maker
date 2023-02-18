use raylib::prelude::*;

enum NodeTypes {
    Dialogue,
    Options,
    Conditional,
    SetFlag,
    SomethingHasGoneReallyWrong,
}

impl Default for NodeTypes {
    fn default() -> NodeTypes {
        NodeTypes::SomethingHasGoneReallyWrong
    }
}

#[derive(Default)]
struct Node {
    character: Option<String>,
    dialogue: Option<String>,
    options: Option<Vec<String>>,
    flag_to_check: Option<String>,
    flag_to_set: Option<String>,
    value_to_set: Option<String>,
    node_type: NodeTypes,
}

impl Node {
    fn default_dialogue() -> Node {
        Node {
            character: Some("".to_string()),
            dialogue: Some("".to_string()),
            options: None,
            flag_to_check: None,
            flag_to_set: None,
            value_to_set: None,
            node_type: NodeTypes::Dialogue,
        }
    }

    fn new_dialogue<T: ToString>(character: T, dialogue: T) -> Node {
        let mut to_return = Node::default_dialogue();
        to_return.character = Some(character.to_string());
        to_return.dialogue = Some(dialogue.to_string());

        to_return
    }
}

// Note: Cards and widgets will be references to nodes, nodes will not have access to anything related to cards and widgets, but cards and widgets will have knowledge of nodes

enum WidgetType {
    TextInput,
}

struct Widget {
    value: String,
    widget_type: WidgetType,
}

impl Widget {
    fn set_value(&mut self, new_val: String) {
        self.value = new_val;
    }
    fn get_val(&self) -> String {
        self.value.clone()
    }

    fn draw(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>, in_world_origin_pos: Vector2) {
        match self.widget_type {
            WidgetType::TextInput => {
                d.draw_rectangle(
                    in_world_origin_pos.x as i32,
                    in_world_origin_pos.y as i32,
                    150,
                    25,
                    Color::GRAY,
                );
                d.draw_rectangle(
                    in_world_origin_pos.x as i32 + 1,
                    in_world_origin_pos.y as i32 + 1,
                    148,
                    23,
                    Color::WHITE,
                );
                d.draw_text(
                    &self.value,
                    in_world_origin_pos.x as i32 + 3,
                    in_world_origin_pos.y as i32 + 3,
                    19,
                    Color::BLACK,
                )
            }
        }
    }

    fn was_clicked(&self, in_world_origin_pos: Vector2, in_world_mouse_pos: Vector2) -> bool {
        let size = match self.widget_type {
            WidgetType::TextInput => Vector2 { x: 150., y: 25. },
        };

        let origin_x = in_world_origin_pos.x as i32;
        let origin_y = in_world_origin_pos.y as i32;
        let mouse_x = in_world_mouse_pos.x as i32;
        let mouse_y = in_world_mouse_pos.y as i32;

        if mouse_x > origin_x && mouse_x < origin_x + size.x as i32 {
            if mouse_y > origin_y && mouse_y < origin_y + size.y as i32 {
                return true;
            }
        }
        return false;
    }
}

struct Card {
    pos: Vector2,
    size: Vector2,
    widgets: Vec<Widget>,
    card_type: NodeTypes,
}

impl Card {
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
        self.draw_card_bg(d);
        match self.card_type {
            NodeTypes::Dialogue => {
                let mut y_offset = 0.;
                y_offset += 10.;

                self.draw_lable(
                    d,
                    "Character:",
                    Vector2 {
                        x: 10.,
                        y: y_offset,
                    },
                );
                y_offset += 25.;
                y_offset += 10.;

                let chr_label = &self.widgets[0];
                chr_label.draw(
                    d,
                    self.pos
                        + Vector2 {
                            x: 10.,
                            y: y_offset,
                        },
                );
                y_offset += 25.;
                y_offset += 10.;

                self.draw_lable(
                    d,
                    "Dialogue:",
                    Vector2 {
                        x: 10.,
                        y: y_offset,
                    },
                );
                y_offset += 25.;
                y_offset += 10.;

                let dlg_label = &self.widgets[1];
                dlg_label.draw(d, self.pos + Vector2 { x: 10., y: y_offset });
                
                println!("{}", y_offset);
            }
            _ => unimplemented!(),
        }
    }

    fn draw_lable(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>, text: &str, offset: Vector2) {
        d.draw_text(
            text,
            (self.pos + offset).x as i32,
            (self.pos + offset).y as i32,
            24,
            Color::BLACK,
        );
    }

    fn draw_card_bg(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
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

struct CanvasScene {
    cam: Camera2D,
    cards: Vec<Card>,
    node_pool: Vec<Node>,
}

impl CanvasScene {
    pub fn update(&mut self, rl: &RaylibHandle, last_mouse_pos: &mut Vector2) {
        for i in self.cards.iter_mut() {
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
        for i in &self.cards {
            i.draw(d);
        }
    }

    pub fn parse_node_pool(&mut self) {
        println!("Parsing node_pool");

        for i in &self.node_pool {
            match i.node_type {
                NodeTypes::Dialogue => {
                    let chr = i
                        .character
                        .clone()
                        .unwrap_or("ERROR: NO CHARACTER FOUND".to_string());
                    let dlg = i
                        .dialogue
                        .clone()
                        .unwrap_or("ERROR: NO CHARACTER FOUND".to_string());

                    self.cards.push(Card {
                        pos: Vector2::default(),
                        size: Vector2 { x: 170., y: 150. },
                        widgets: vec![
                            Widget {
                                value: chr,
                                widget_type: WidgetType::TextInput,
                            },
                            Widget {
                                value: dlg,
                                widget_type: WidgetType::TextInput,
                            },
                        ],
                        card_type: NodeTypes::Dialogue,
                    });
                }

                _ => unimplemented!(),
            }
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
        cards: Vec::default(),
        node_pool: vec![Node::new_dialogue("John doe", "Test test testing")],
    };
    canvas_scene.parse_node_pool();

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

        // new_d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        new_d.draw_fps(tlp.x as i32, tlp.y as i32);
    }
}
