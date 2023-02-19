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

#[derive(Clone)]
enum WidgetType {
    TextInput,
}

#[derive(Clone)]
struct Widget {
    value: String,
    widget_type: WidgetType,
    offset: Vector2,
}

impl Widget {
    fn set_value(&mut self, new_val: String) {
        self.value = new_val;
    }
    fn get_val(&self) -> String {
        self.value.clone()
    }

    fn draw(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>, card_in_world_origin_pos: Vector2) {
        match self.widget_type {
            WidgetType::TextInput => {
                let x_pos = (card_in_world_origin_pos.x + self.offset.x)as i32;
                let y_pos = (card_in_world_origin_pos.y + self.offset.y) as i32;
                d.draw_rectangle(x_pos, y_pos, 150, 25, Color::GRAY);
                d.draw_rectangle(x_pos + 1, y_pos + 1, 148, 23, Color::WHITE);
                let mut text_to_show = self.value.clone();
                if text_to_show.len() > 14 {
                    text_to_show = text_to_show.chars().take(14).collect();
                    text_to_show.push_str("...");
                }
                d.draw_text(&text_to_show, x_pos + 3, y_pos + 3, 19, Color::BLACK)
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
    widgets: Vec<Box<Widget>>,
    card_type: NodeTypes,
}

enum CardNotification {
    EditTextInput(Box<Widget>),
}

impl Card {
    fn update(
        &mut self,
        rl: &RaylibHandle,
        mouse_pos: &mut Vector2,
        cur_zoom: f32,
        cam: &Camera2D,
    ) -> Option<CardNotification> {
        let mouse_world_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), cam);

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            for i in &self.widgets {
                if i.was_clicked(self.pos + i.offset, mouse_world_pos) {
                    return Some(CardNotification::EditTextInput(Box::clone(&i)));
                }
            }
        }

        // Adapted from 2d camera_mouse_zoom found at: https://www.raylib.com/examples.html
        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse_x = mouse_world_pos.x as i32;
            let mouse_y = mouse_world_pos.y as i32;

            let pos_x = self.pos.x as i32;
            let pos_y = self.pos.y as i32;

            if mouse_x > pos_x && mouse_x < (pos_x + self.size.x as i32) {
                if mouse_y + 12 > pos_y && mouse_y - 12 < pos_y {
                    let mut delta = *mouse_pos - rl.get_mouse_position();
                    delta.scale(-1. / cur_zoom);

                    self.pos = self.pos + delta;
                }
            }
        }

        None
    }

    fn draw(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        self.draw_card_bg(d);
        match self.card_type {
            NodeTypes::Dialogue => {
                self.draw_lable(d, "Character:", Vector2 { x: 10., y: 10. });

                let chr_label = &self.widgets[0];
                chr_label.draw(d, self.pos);

                self.draw_lable(d, "Dialogue:", Vector2 { x: 10., y: 80. });

                let dlg_label = &self.widgets[1];
                dlg_label.draw(d, self.pos);
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

        let x_pos = self.pos.x as i32;
        let y_pos = self.pos.y as i32;
        let x_size = self.size.x as i32;
        let y_size = self.size.y as i32;

        d.draw_circle(x_pos, y_pos, 12., Color::BROWN);

        d.draw_rectangle(x_pos, y_pos - 12, x_size, 24, Color::BROWN);

        d.draw_circle(x_pos + x_size, y_pos, 12., Color::BROWN);

        d.draw_circle(
            x_pos + corner_radius,
            y_pos + corner_radius,
            corner_radius as f32,
            Color::SKYBLUE,
        );
        d.draw_circle(
            x_pos + x_size - corner_radius,
            y_pos + corner_radius,
            corner_radius as f32,
            Color::SKYBLUE,
        );
        d.draw_circle(
            x_pos + corner_radius,
            y_pos + y_size - corner_radius,
            corner_radius as f32,
            Color::SKYBLUE,
        );
        d.draw_circle(
            x_pos + x_size - corner_radius,
            y_pos + y_size - corner_radius,
            corner_radius as f32,
            Color::SKYBLUE,
        );

        d.draw_rectangle(
            x_pos + corner_radius,
            y_pos,
            x_size - corner_radius * 2,
            y_size,
            Color::SKYBLUE,
        );
        d.draw_rectangle(
            x_pos,
            y_pos + corner_radius,
            x_size,
            y_size - corner_radius * 2,
            Color::SKYBLUE,
        );
    }
}

enum CanvasSceneStates {
    Roaming,
    EditingTextInput(Box<Widget>),
}

struct CanvasScene {
    cam: Camera2D,
    cards: Vec<Card>,
    node_pool: Vec<Node>,
    state: CanvasSceneStates,
}

impl CanvasScene {
    fn update(&mut self, rl: &RaylibHandle, last_mouse_pos: &mut Vector2) {
        match self.state {
            CanvasSceneStates::Roaming => {
                self.update_roaming(rl, last_mouse_pos);
            }
            _ => unimplemented!(),
        }
    }

    pub fn update_roaming(&mut self, rl: &RaylibHandle, last_mouse_pos: &mut Vector2) {
        for i in self.cards.iter_mut() {
            let notify = i.update(rl, last_mouse_pos, self.cam.zoom, &self.cam);

            match notify {
                Some(notification_type) => match notification_type {
                    CardNotification::EditTextInput(wte) => {
                        self.state = CanvasSceneStates::EditingTextInput(Box::clone(&wte));
                        return;
                    }
                    _ => unimplemented!(),
                },
                None => {}
            }
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
                top_left.y as i32 - 10,
                i * 50,
                bottom_right.y as i32 + 10,
                Color::BLACK,
            );
        }
        for i in (top_left.y - 50.) as i32 / 50..(bottom_right.y + 50.) as i32 / 50 {
            d.draw_line(
                top_left.x as i32 - 10,
                i * 50,
                bottom_right.x as i32 + 10,
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
                            Box::new(Widget {
                                value: chr,
                                widget_type: WidgetType::TextInput,
                                offset: Vector2 { x: 10., y: 45. },
                            }),
                            Box::new(Widget {
                                value: dlg,
                                widget_type: WidgetType::TextInput,
                                offset: Vector2 { x: 10., y: 115. },
                            }),
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
        state: CanvasSceneStates::Roaming,
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
