use raylib::prelude::*;

#[derive(Clone, Debug)]
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

#[derive(Debug, Clone)]
enum NodeMember {
    Character,
    Dialogue,
    Options(usize),
    FlagToCheck,
    FlagToSet,
}

#[derive(Default, Clone)]
struct Node {
    id: String,
    character: Option<String>,
    dialogue: Option<String>,
    options: Option<Vec<String>>,
    flag_to_check: Option<String>,
    flag_to_set: Option<String>,
    value_to_set: Option<String>,
    front_links: Vec<String>, // Vector of other Nodes' ids
    node_type: NodeTypes,
}

impl Node {
    fn default_dialogue() -> Node {
        Node {
            id: String::default(),
            character: Some("".to_string()),
            dialogue: Some("".to_string()),
            options: None,
            flag_to_check: None,
            flag_to_set: None,
            value_to_set: None,
            front_links: Vec::default(),
            node_type: NodeTypes::Dialogue,
        }
    }
    fn new_dialogue<T: ToString>(
        id: T,
        character: T,
        dialogue: T,
        front_links: Vec<String>,
    ) -> Node {
        let mut to_return = Node::default_dialogue();
        to_return.id = id.to_string();
        to_return.character = Some(character.to_string());
        to_return.dialogue = Some(dialogue.to_string());
        to_return.front_links = front_links;

        to_return
    }

    fn default_options() -> Node {
        Node {
            id: String::default(),
            character: None,
            dialogue: None,
            options: Some(vec![]),
            flag_to_check: None,
            flag_to_set: None,
            value_to_set: None,
            front_links: Vec::default(),
            node_type: NodeTypes::Options,
        }
    }
    fn new_options<T: ToString>(id: T, options: Vec<String>, front_links: Vec<String>) -> Node {
        let mut to_return = Node::default_options();
        to_return.id = id.to_string();
        to_return.options = Some(options);
        to_return.front_links = front_links;

        to_return
    }
}

// Note: Cards and widgets will be references to nodes, nodes will not have access to anything related to cards and widgets, but cards and widgets will have knowledge of nodes

#[derive(Clone)]
enum WidgetType {
    TextInput,
}
// TODO: Implement outputs
#[derive(Clone)]
struct Widget {
    node_ref: String,
    widget_type: WidgetType,
    editing_node_member: NodeMember,

    offset: Vector2,
}

impl Widget {
    fn draw(
        &self,
        d: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        card_in_world_origin_pos: Vector2,
        text: Option<String>,
    ) {
        match self.widget_type {
            WidgetType::TextInput => {
                let x_pos = (card_in_world_origin_pos.x + self.offset.x) as i32;
                let y_pos = (card_in_world_origin_pos.y + self.offset.y) as i32;
                d.draw_rectangle(x_pos, y_pos, 150, 25, Color::GRAY);
                d.draw_rectangle(x_pos + 1, y_pos + 1, 148, 23, Color::WHITE);
                let mut text_to_show = text.unwrap();
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

#[derive(Clone)]
struct Card {
    node_ref: String,
    pos: Vector2,
    size: Vector2,
    widgets: Vec<Widget>,
    card_type: NodeTypes,
}

#[derive(Debug)]
enum CardNotification {
    EditTextInput { id: String, node_member: NodeMember },
    AddOptionToOptionsNode(String),
}

impl Card {
    fn new_dialogue_card(node_id: String, pos: Vector2) -> Card {
        Card {
            node_ref: node_id.clone(),
            pos: pos,
            size: Vector2 { x: 170., y: 150. },
            widgets: vec![
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::TextInput,
                    offset: Vector2 { x: 10., y: 45. },
                    editing_node_member: NodeMember::Character,
                },
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::TextInput,
                    offset: Vector2 { x: 10., y: 115. },
                    editing_node_member: NodeMember::Dialogue,
                },
            ],
            card_type: NodeTypes::Dialogue,
        }
    }

    fn new_options_card(node_id: String, options: Vec<String>, pos: Vector2) -> Card {
        let mut options_widgets = vec![];

        let mut y_offset = 10.;
        let mut cur_i = 0;
        for i in options {
            options_widgets.push(Widget {
                node_ref: node_id.clone(),
                widget_type: WidgetType::TextInput,
                offset: Vector2 {
                    x: 10.,
                    y: y_offset,
                },
                editing_node_member: NodeMember::Options(cur_i),
            });
            y_offset += 35.;
            cur_i += 1;
        }

        Card {
            node_ref: node_id.clone(),
            pos: pos,
            size: Vector2 {
                x: 170.,
                y: y_offset,
            },
            widgets: options_widgets,
            card_type: NodeTypes::Options,
        }
    }

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
                    return Some(CardNotification::EditTextInput {
                        id: i.node_ref.clone(),
                        node_member: i.editing_node_member.clone(),
                    });
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

        match self.card_type {
            NodeTypes::Options => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    let add_button_center = Vector2 {
                        x: self.pos.x + self.size.x / 2.,
                        y: self.pos.y + self.size.y,
                    };

                    if mouse_world_pos.distance_to(add_button_center) < 10. {
                        return Some(CardNotification::AddOptionToOptionsNode(self.node_ref.clone()));
                    }
                }
            }
            _ => {}
        }

        None
    }

    fn draw(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>, node_data: Node) {
        self.draw_card_bg(d, node_data.clone());
        match self.card_type {
            NodeTypes::Dialogue => {
                self.draw_lable(d, "Character:", Vector2 { x: 10., y: 10. });

                let chr_label = &self.widgets[0];
                chr_label.draw(d, self.pos, node_data.character);

                self.draw_lable(d, "Dialogue:", Vector2 { x: 10., y: 80. });

                let dlg_label = &self.widgets[1];
                dlg_label.draw(d, self.pos, node_data.dialogue);
            }
            NodeTypes::Options => {
                let mut cur_opt_i = 0;
                for i in &self.widgets {
                    let cur_opt_vec = node_data.options.clone().unwrap();
                    let cur_opt_text = cur_opt_vec[cur_opt_i].clone();
                    i.draw(d, self.pos, Some(cur_opt_text));
                    cur_opt_i += 1;
                }
            }
            _ => unimplemented!("{:?}", self.card_type),
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

    fn draw_card_bg(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>, node_data: Node) {
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

        d.draw_circle(x_pos, y_pos, corner_radius as f32, Color::PINK);

        match self.card_type {
            NodeTypes::Options => {
                let mut y_offset = 25;
                for i in node_data.options.unwrap() {
                    d.draw_circle(
                        x_pos + x_size - 10 + corner_radius,
                        y_pos + y_offset,
                        corner_radius as f32,
                        Color::PINK,
                    );
                    y_offset += 35;
                }

                y_offset -= 15;
                d.draw_circle(
                    x_pos + x_size / 2,
                    y_pos + y_offset,
                    corner_radius as f32,
                    Color::RED,
                );
            }
            _ => {
                d.draw_circle(
                    x_pos + x_size - 10 + corner_radius,
                    y_pos + y_size - corner_radius,
                    corner_radius as f32,
                    Color::PINK,
                );
            }
        }
    }

    fn get_output_pos(&self) -> Vector2 {
        let to_return = self.pos + self.size - Vector2 { x: 0., y: 10. };
        to_return
    }
    fn get_input_pos(&self) -> Vector2 {
        let to_return = self.pos;
        to_return
    }
}

enum CanvasSceneStates {
    Roaming,
    EditingTextInput(String, NodeMember), // Id the currently being modified Node
}

struct CanvasScene {
    cam: Camera2D,
    cards: Vec<Card>,
    node_pool: Vec<Node>,
    state: CanvasSceneStates,
}

impl CanvasScene {
    fn update(&mut self, rl: &RaylibHandle, last_mouse_pos: &mut Vector2) {
        match &self.state {
            CanvasSceneStates::Roaming => {
                self.update_roaming(rl, last_mouse_pos);
            }
            CanvasSceneStates::EditingTextInput(_, _) => {}
            _ => unimplemented!(),
        }
    }

    pub fn update_roaming(&mut self, rl: &RaylibHandle, last_mouse_pos: &mut Vector2) {
        let mut post_handle_notification = None;

        for i in self.cards.iter_mut() {
            let notify = i.update(rl, last_mouse_pos, self.cam.zoom, &self.cam);

            match notify {
                Some(notification_type) => match notification_type {
                    CardNotification::EditTextInput { id, node_member } => {
                        self.state = CanvasSceneStates::EditingTextInput(id, node_member);
                        return;
                    }
                    CardNotification::AddOptionToOptionsNode(id) => {
                        post_handle_notification = Some(CardNotification::AddOptionToOptionsNode(id));
                    }
                    _ => {unimplemented!("{:?}", notification_type)},
                },
                None => {}
            }
        }

        match post_handle_notification {
            None => {}
            Some(notification) => match notification {
                CardNotification::AddOptionToOptionsNode(id) => {
                    let pos = self.copy_card_data_from_id(id.clone()).pos;
                    let mut i = 0;
                    for j in &self.node_pool {
                        if j.id == id {
                            break;
                        }
                        i += 1;
                    }

                    let mut cur_node = &mut self.node_pool[i];
                    let mut next_node_opt_vec = cur_node.options.clone().unwrap();
                    next_node_opt_vec.push("Empty".to_string());
                    cur_node.options = Some(next_node_opt_vec);

                    let new_card = Card::new_options_card(
                        cur_node.id.clone(),
                        cur_node.clone().options.unwrap(),
                        pos,
                    );

                    self.cards[i] = new_card;
                }
                _ => unimplemented!("{:?}", notification)
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
            i.draw(d, self.copy_node_data_from_id(i.node_ref.clone()));
        }
    }

    // Yes, I diceded to go with some imediate ui here
    fn update_and_draw_text_input_edit(
        &mut self,
        d: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        tlp: Vector2,
    ) {
        match self.state {
            CanvasSceneStates::EditingTextInput(_, _) => {}
            _ => return,
        }

        let keymap = [
            ('A', KeyboardKey::KEY_A),
            ('B', KeyboardKey::KEY_B),
            ('C', KeyboardKey::KEY_C),
            ('D', KeyboardKey::KEY_D),
            ('E', KeyboardKey::KEY_E),
            ('F', KeyboardKey::KEY_F),
            ('G', KeyboardKey::KEY_G),
            ('H', KeyboardKey::KEY_H),
            ('I', KeyboardKey::KEY_I),
            ('J', KeyboardKey::KEY_J),
            ('K', KeyboardKey::KEY_K),
            ('L', KeyboardKey::KEY_L),
            ('M', KeyboardKey::KEY_M),
            ('N', KeyboardKey::KEY_N),
            ('O', KeyboardKey::KEY_O),
            ('P', KeyboardKey::KEY_P),
            ('Q', KeyboardKey::KEY_Q),
            ('R', KeyboardKey::KEY_R),
            ('S', KeyboardKey::KEY_S),
            ('T', KeyboardKey::KEY_T),
            ('U', KeyboardKey::KEY_U),
            ('V', KeyboardKey::KEY_V),
            ('W', KeyboardKey::KEY_W),
            ('X', KeyboardKey::KEY_X),
            ('Y', KeyboardKey::KEY_Y),
            ('Z', KeyboardKey::KEY_Z),
        ];

        let mut cur_text;

        match &self.state {
            CanvasSceneStates::EditingTextInput(wte, member) => match member {
                NodeMember::Character => {
                    cur_text = self.copy_node_data_from_id(wte.clone()).character.unwrap();
                }
                NodeMember::Dialogue => {
                    cur_text = self.copy_node_data_from_id(wte.clone()).dialogue.unwrap();
                }
                NodeMember::Options(i) => {
                    let options_vec = &self.copy_node_data_from_id(wte.clone()).options.unwrap();
                    cur_text = options_vec[*i].clone();
                }
                _ => unimplemented!("{:?}", member),
            },
            _ => panic!("Something has gone incredibly wrong."),
        }
        for &(c, key) in &keymap {
            if d.is_key_pressed(key) {
                cur_text.push(c);
            }
        }
        match &self.state {
            CanvasSceneStates::EditingTextInput(id, member) => {
                for i in &mut self.node_pool {
                    if i.id == id.clone() {
                        match member {
                            NodeMember::Dialogue => {
                                i.dialogue = Some(cur_text.clone());
                            }
                            NodeMember::Character => {
                                i.character = Some(cur_text.clone());
                            }
                            NodeMember::Options(opt_i) => {
                                let mut cur_vec = i.options.clone().unwrap();
                                cur_vec[*opt_i] = cur_text.clone();
                                i.options = Some(cur_vec);
                            }
                            _ => unimplemented!("{:?}", member),
                        }
                    }
                }
            }
            _ => panic!("Something has gone incredibly wrong."),
        }

        if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.state = CanvasSceneStates::Roaming;
            return;
        }

        d.draw_rectangle(
            (tlp.x) as i32 - 10,
            (tlp.y) as i32 - 10,
            1290,
            730,
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 50,
            },
        );
        d.draw_rectangle(
            (tlp.x) as i32 + 10,
            (tlp.y) as i32 + 10,
            1250,
            690,
            Color::WHITE,
        );

        d.draw_text(
            &cur_text,
            (tlp.x) as i32 + 20,
            (tlp.y) as i32 + 20,
            24,
            Color::BLACK,
        );
    }

    pub fn parse_node_pool(&mut self) {
        println!("Parsing node_pool");

        let mut x_offset = 0.;
        for i in &self.node_pool {
            match i.node_type {
                NodeTypes::Dialogue => {
                    let card_pos = Vector2 { x: x_offset, y: 0. };
                    self.cards
                        .push(Card::new_dialogue_card(i.id.clone(), card_pos));
                    x_offset += 200.;
                }
                NodeTypes::Options => {
                    let card_pos = Vector2 { x: x_offset, y: 0. };
                    self.cards.push(Card::new_options_card(
                        i.id.clone(),
                        i.clone().options.unwrap(),
                        card_pos,
                    ));
                    x_offset += 200.;
                }
                _ => unimplemented!("{:?}", i.node_type),
            }
        }
    }

    fn copy_card_data_from_id(&self, id: String) -> Card {
        for i in &self.cards {
            if i.node_ref == id {
                return i.clone();
            }
        }

        unreachable!()
    }
    fn copy_node_data_from_id(&self, id: String) -> Node {
        for i in &self.node_pool {
            if i.id == id {
                return i.clone();
            }
        }

        unreachable!()
    }

    fn draw_card_connections(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        for i in &self.node_pool {
            let i_card = self.copy_card_data_from_id(i.id.clone());

            match i_card.card_type {
                NodeTypes::Options => {
                    let mut y_offset = 25;
                    for j in &i.front_links {
                        let j_card = self.copy_card_data_from_id(j.clone());
                        let start_pos = Vector2 {
                            x: i_card.pos.x + i_card.size.x,
                            y: i_card.pos.y + y_offset as f32,
                        };
                        d.draw_line_ex(start_pos, j_card.get_input_pos(), 5., Color::PURPLE);
                        y_offset += 35;
                    }
                }

                _ => {
                    for j in &i.front_links {
                        let j_card = self.copy_card_data_from_id(j.clone());
                        d.draw_line_ex(
                            i_card.get_output_pos(),
                            j_card.get_input_pos(),
                            5.,
                            Color::PURPLE,
                        );
                    }
                }
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
        node_pool: vec![
            Node::new_dialogue(
                "001",
                "John doe",
                "Test test testing",
                vec!["002".to_string()],
            ),
            Node::new_dialogue(
                "002",
                "Second one coming",
                "I really hope this doesn't break everything.",
                vec!["003".to_string()],
            ),
            Node::new_options(
                "003",
                vec!["Hi".to_string(), "Bye".to_string(), "Let's go".to_string()],
                vec!["001".to_string(), "002".to_string(), "003".to_string()],
            ),
        ],
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
        canvas_scene.draw_card_connections(&mut new_d);

        // ===== IMGUI LIKE PART =====
        canvas_scene.update_and_draw_text_input_edit(&mut new_d, tlp); // Runs only if canvas state is EditingTextInput

        // new_d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        new_d.draw_fps(tlp.x as i32, tlp.y as i32);
    }
}
