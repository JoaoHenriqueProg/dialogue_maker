#![allow(unreachable_patterns)]

use std::collections::HashMap;

use json_parser::{JsonObject, JsonType, Parser};
use raylib::{input::key_from_i32, prelude::*};

mod json_parser;

#[derive(Debug)]
enum CanvasMouseState {
    Roaming,
    CreatingConnection(String, usize), //id, output_index
    MovingCard(String),
}

#[derive(Clone, Debug, PartialEq)]
enum NodeTypes {
    Dialogue,
    Branches,
    SetFlag,
    Conditional, // Noticed I didn't think enough about this one, decide to make it so that flags are only flags
    EmitEvent,
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
    Branch(usize),
    FlagToCheck,
    FlagToSet,
    ValueToSet,
    EventToEmit,
    EventDataKey(usize),
    EventDataVal(usize),
}

#[derive(Default, Clone)]
struct Node {
    id: String,
    character: Option<String>,
    dialogue: Option<String>,
    branches: Option<Vec<String>>,
    flag_to_check: Option<String>,
    flag_to_set: Option<String>,
    value_to_set: Option<bool>,
    front_links: Vec<String>, // Vector of other Nodes' ids
    event_to_emit: Option<String>,
    event_data: Option<Vec<(String, String)>>, // TODO?: Maybe integrate the JsonType from my json parser
    node_type: NodeTypes,
}

impl Node {
    fn default_dialogue() -> Node {
        let mut to_return = Node::default();
        to_return.character = Some("".to_string());
        to_return.dialogue = Some("".to_string());
        to_return.front_links = vec![];
        to_return.node_type = NodeTypes::Dialogue;
        to_return
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

    fn default_branches() -> Node {
        let mut to_return = Node::default();
        to_return.branches = Some(vec![]);
        to_return.front_links = vec![];
        to_return.node_type = NodeTypes::Branches;
        to_return
    }
    fn new_branches<T: ToString>(id: T, branches: Vec<String>, front_links: Vec<String>) -> Node {
        let mut to_return = Node::default_branches();
        to_return.id = id.to_string();
        to_return.branches = Some(branches);
        to_return.front_links = front_links;
        to_return
    }

    fn default_conditional() -> Node {
        let mut to_return = Node::default();
        to_return.flag_to_check = Some("".to_string());
        to_return.front_links = vec!["".to_string(), "".to_string(), "".to_string()];
        to_return.node_type = NodeTypes::Conditional;
        to_return
    }
    fn new_conditional<T: ToString>(id: T, flag_to_check: T, front_links: Vec<String>) -> Node {
        let mut to_return = Node::default_conditional();
        to_return.id = id.to_string();
        to_return.flag_to_check = Some(flag_to_check.to_string());

        if front_links.len() != 3 {
            println!("ERROR: New_conditional front_links parameter should have a lenght of 3, setting to default");
            to_return.front_links = vec!["".to_string(), "".to_string(), "".to_string()];
        } else {
            to_return.front_links = front_links;
        }

        to_return
    }

    fn default_set_flag() -> Node {
        let mut to_return = Node::default();
        to_return.flag_to_set = Some("".to_string());
        to_return.value_to_set = Some(false);
        to_return.front_links = vec![];
        to_return.node_type = NodeTypes::SetFlag;
        to_return
    }
    fn new_set_flag<T: ToString>(
        id: T,
        flag_to_set: T,
        value_to_set: bool,
        front_links: Vec<String>,
    ) -> Node {
        let mut to_return = Node::default_set_flag();
        to_return.id = id.to_string();
        to_return.flag_to_set = Some(flag_to_set.to_string());
        to_return.value_to_set = Some(value_to_set);
        to_return.front_links = front_links;
        to_return
    }

    fn default_emit_event() -> Node {
        let mut to_return = Node::default();
        to_return.event_to_emit = Some("".to_string());
        to_return.node_type = NodeTypes::EmitEvent;
        to_return
    }
    fn new_emit_event<T: ToString>(
        id: T,
        event_to_emit: T,
        event_data: Vec<(String, String)>,
        front_links: Vec<String>,
    ) -> Node {
        let mut to_return = Node::default_emit_event();
        to_return.id = id.to_string();
        to_return.event_to_emit = Some(event_to_emit.to_string());
        to_return.event_data = Some(event_data);
        to_return.front_links = front_links;
        to_return
    }
}

// Note: Cards and widgets will be references to nodes, nodes will not have access to anything related to cards and widgets, but cards and widgets will have knowledge of nodes

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum WidgetType {
    TextInput,
    CheckBox,
    OutputConnection,
}
// TODO: Implement outputs
#[derive(Clone)]
struct Widget {
    node_ref: String,
    widget_type: WidgetType,
    editing_node_member: Option<NodeMember>,

    offset: Vector2,
}

impl Widget {
    fn draw(
        &self,
        d: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        card_world_pos: Vector2,
        text: Option<String>,
        check_box_state: Option<bool>,
    ) {
        let x_pos = (card_world_pos.x + self.offset.x) as i32;
        let y_pos = (card_world_pos.y + self.offset.y) as i32;
        match self.widget_type {
            WidgetType::TextInput => {
                d.draw_rectangle(x_pos, y_pos, 150, 25, Color::GRAY);
                d.draw_rectangle(x_pos + 1, y_pos + 1, 148, 23, Color::WHITE);
                let mut text_to_show = text.unwrap();
                if text_to_show.len() > 14 {
                    text_to_show = text_to_show.chars().take(14).collect();
                    text_to_show.push_str("...");
                }
                d.draw_text(&text_to_show, x_pos + 3, y_pos + 3, 19, Color::BLACK)
            }
            WidgetType::OutputConnection => d.draw_circle(x_pos, y_pos, 10., Color::GREEN),
            WidgetType::CheckBox => {
                d.draw_rectangle(x_pos, y_pos, 25, 25, Color::GRAY);

                let mut color = Color::WHITE;
                if check_box_state.unwrap() {
                    color = Color::GREEN;
                }

                d.draw_rectangle(x_pos + 1, y_pos + 1, 23, 23, color);

                let text = match check_box_state.unwrap() {
                    true => "True",
                    false => "False",
                };

                d.draw_text(text, x_pos + 35, y_pos, 25, Color::BLACK);
            }
            _ => unimplemented!("{:?}", self.widget_type),
        }
    }

    fn was_clicked(&self, in_world_origin_pos: Vector2, in_world_mouse_pos: Vector2) -> bool {
        let offset = match self.widget_type {
            WidgetType::OutputConnection => Vector2 { x: 10., y: 10. },
            _ => Vector2 { x: 0., y: 0. },
        };

        let size = match self.widget_type {
            WidgetType::TextInput => Vector2 { x: 150., y: 25. },
            WidgetType::OutputConnection => Vector2 { x: 20., y: 20. },
            WidgetType::CheckBox => Vector2 { x: 25., y: 25. },
            _ => unimplemented!("{:?}", self.widget_type),
        };

        let pos_x = in_world_origin_pos.x as i32;
        let pos_y = in_world_origin_pos.y as i32;
        let mouse_x = (in_world_mouse_pos.x + offset.x) as i32;
        let mouse_y = (in_world_mouse_pos.y + offset.y) as i32;

        if mouse_x > pos_x && mouse_x < pos_x + size.x as i32 {
            if mouse_y > pos_y && mouse_y < pos_y + size.y as i32 {
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
    AddBranchToBranchesNode(String),
    AddArgToEmitEventNode(String),
    ToggleCheckBox { id: String, node_member: NodeMember },
    CreatingCardConnection(String, usize), // id, output index
    MovingCard(String),
}

impl Card {
    fn new_dialogue(node_id: String, pos: Vector2) -> Card {
        Card {
            node_ref: node_id.clone(),
            pos: pos,
            size: Vector2 { x: 170., y: 150. },
            widgets: vec![
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::TextInput,
                    offset: Vector2 { x: 10., y: 45. },
                    editing_node_member: Some(NodeMember::Character),
                },
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::TextInput,
                    offset: Vector2 { x: 10., y: 115. },
                    editing_node_member: Some(NodeMember::Dialogue),
                },
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::OutputConnection,
                    offset: Vector2 { x: 170., y: 140. },
                    editing_node_member: None,
                },
            ],
            card_type: NodeTypes::Dialogue,
        }
    }

    fn new_branches(node_id: String, branches: Vec<String>, pos: Vector2) -> Card {
        let mut branches_widgets = vec![];

        let mut offset_y = 10.;
        let mut cur_i = 0;

        for _ in branches {
            branches_widgets.push(Widget {
                node_ref: node_id.clone(),
                widget_type: WidgetType::TextInput,
                offset: Vector2 {
                    x: 10.,
                    y: offset_y,
                },
                editing_node_member: Some(NodeMember::Branch(cur_i)),
            });
            branches_widgets.push(Widget {
                node_ref: node_id.clone(),
                widget_type: WidgetType::OutputConnection,
                offset: Vector2 {
                    x: 170.,
                    y: offset_y + 10.,
                },
                editing_node_member: None,
            });
            offset_y += 35.;
            cur_i += 1;
        }

        Card {
            node_ref: node_id.clone(),
            pos: pos,
            size: Vector2 {
                x: 170.,
                y: offset_y,
            },
            widgets: branches_widgets,
            card_type: NodeTypes::Branches,
        }
    }

    fn new_conditional(node_id: String, pos: Vector2) -> Card {
        Card {
            node_ref: node_id.clone(),
            pos: pos,
            size: Vector2 { x: 170., y: 185. },
            // flag to check
            // if true ->
            // if false ->
            // wasn't set ->
            widgets: vec![
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::TextInput,
                    editing_node_member: Some(NodeMember::FlagToCheck),
                    offset: Vector2 { x: 10., y: 10. },
                },
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::OutputConnection,
                    editing_node_member: None,
                    offset: Vector2 { x: 170., y: 90. },
                },
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::OutputConnection,
                    editing_node_member: None,
                    offset: Vector2 { x: 170., y: 125. },
                },
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::OutputConnection,
                    editing_node_member: None,
                    offset: Vector2 { x: 170., y: 160. },
                },
            ],
            card_type: NodeTypes::Conditional,
        }
    }

    fn new_set_flag(node_id: String, pos: Vector2) -> Card {
        Card {
            node_ref: node_id.clone(),
            pos: pos,
            size: Vector2 { x: 170., y: 80. },
            widgets: vec![
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::TextInput,
                    editing_node_member: Some(NodeMember::FlagToSet),
                    offset: Vector2 { x: 10., y: 10. },
                },
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::CheckBox,
                    editing_node_member: Some(NodeMember::ValueToSet),
                    offset: Vector2 { x: 10., y: 45. },
                },
                Widget {
                    node_ref: node_id.clone(),
                    widget_type: WidgetType::OutputConnection,
                    editing_node_member: None,
                    offset: Vector2 { x: 170., y: 55. },
                },
            ],
            card_type: NodeTypes::SetFlag,
        }
    }

    fn new_emit_event(node_id: String, data: Vec<(String, String)>, pos: Vector2) -> Card {
        let mut wids = vec![];

        wids.push(Widget {
            node_ref: node_id.clone(),
            widget_type: WidgetType::TextInput,
            editing_node_member: Some(NodeMember::EventToEmit),
            offset: Vector2 { x: 10., y: 10. },
        });

        let mut y_offset = 35.;
        for (arg_i, (key, val)) in data.iter().enumerate() {
            y_offset += 10.;
            wids.push(Widget {
                node_ref: node_id.clone(),
                widget_type: WidgetType::TextInput,
                editing_node_member: Some(NodeMember::EventDataKey(arg_i.clone())),
                offset: Vector2 {
                    x: 10.,
                    y: y_offset,
                },
            });
            y_offset += 25.;
            wids.push(Widget {
                node_ref: node_id.clone(),
                widget_type: WidgetType::TextInput,
                editing_node_member: Some(NodeMember::EventDataVal(arg_i.clone())),
                offset: Vector2 {
                    x: 10.,
                    y: y_offset,
                },
            });
            y_offset += 25.;
        }

        wids.push(Widget {
            node_ref: node_id.clone(),
            widget_type: WidgetType::OutputConnection,
            editing_node_member: None,
            offset: Vector2 {
                x: 170.,
                y: y_offset,
            },
        });

        Card {
            node_ref: node_id.clone(),
            pos: pos,
            size: Vector2 {
                x: 170.,
                y: y_offset + 10.,
            },
            widgets: wids,
            card_type: NodeTypes::EmitEvent,
        }
    }

    fn copy_output_widgets(&self) -> Vec<Widget> {
        self.widgets
            .iter()
            .filter(|x| x.widget_type == WidgetType::OutputConnection)
            .map(|x| x.clone())
            .collect()
    }

    fn from_output_widget_i_to_node_front_link_i(&self, wid_i: &usize) -> usize {
        let mut cur_i = 0;
        let mut cur_found_output = -1;
        for w in &self.widgets {
            if w.widget_type == WidgetType::OutputConnection {
                cur_found_output += 1;
            }

            if cur_i as usize == wid_i.clone() {
                return cur_found_output as usize;
            }

            cur_i += 1;
        }

        unreachable!()
    }

    fn update(&mut self, rl: &RaylibHandle, mouse_world_pos: Vector2) -> Option<CardNotification> {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            // Adapted from 2d camera_mouse_zoom found at: https://www.raylib.com/examples.html
            let mouse_x = mouse_world_pos.x as i32;
            let mouse_y = mouse_world_pos.y as i32;

            let pos_x = self.pos.x as i32;
            let pos_y = self.pos.y as i32;

            if mouse_x > pos_x && mouse_x < (pos_x + self.size.x as i32) {
                if mouse_y + 12 > pos_y && mouse_y - 12 < pos_y {
                    return Some(CardNotification::MovingCard(self.node_ref.clone()));
                }
            }

            for (wid_i, wid) in self.widgets.iter().enumerate() {
                if wid.was_clicked(self.pos + wid.offset, mouse_world_pos) {
                    match wid.widget_type {
                        WidgetType::CheckBox => {
                            return Some(CardNotification::ToggleCheckBox {
                                id: wid.node_ref.clone(),
                                node_member: wid.editing_node_member.clone().unwrap(),
                            });
                        }
                        WidgetType::TextInput => {
                            return Some(CardNotification::EditTextInput {
                                id: wid.node_ref.clone(),
                                node_member: wid.editing_node_member.clone().unwrap(),
                            });
                        }
                        WidgetType::OutputConnection => {
                            return Some(CardNotification::CreatingCardConnection(
                                wid.node_ref.clone(),
                                wid_i,
                            ));
                        }
                        _ => {
                            println!("TODO: Handle was_clicked for '{:?}'", wid.widget_type);
                        }
                    }
                }
            }

            match self.card_type {
                NodeTypes::Branches => {
                    let add_button_center = Vector2 {
                        x: self.pos.x + self.size.x / 2.,
                        y: self.pos.y + self.size.y,
                    };

                    if mouse_world_pos.distance_to(add_button_center) < 10. {
                        return Some(CardNotification::AddBranchToBranchesNode(
                            self.node_ref.clone(),
                        ));
                    }
                }
                NodeTypes::EmitEvent => {
                    let add_button_center = Vector2 {
                        x: self.pos.x + self.size.x / 2.,
                        y: self.pos.y + self.size.y,
                    };

                    if mouse_world_pos.distance_to(add_button_center) < 10. {
                        return Some(CardNotification::AddArgToEmitEventNode(
                            self.node_ref.clone(),
                        ));
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn draw(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>, node_data: Node) {
        self.draw_card_bg(d);
        match self.card_type {
            NodeTypes::Dialogue => {
                self.draw_lable(d, "Character:", Vector2 { x: 10., y: 10. });
                self.widgets[0].draw(d, self.pos, node_data.character, None);
                self.draw_lable(d, "Dialogue:", Vector2 { x: 10., y: 80. });
                self.widgets[1].draw(d, self.pos, node_data.dialogue, None);
                self.widgets[2].draw(d, self.pos, None, None)
            }
            NodeTypes::Branches => {
                let mut cur_opt_i = 0;
                for i in &self.widgets {
                    match i.widget_type {
                        WidgetType::TextInput => {
                            let cur_opt_vec = node_data.branches.clone().unwrap();
                            let cur_opt_text = cur_opt_vec[cur_opt_i].clone();
                            i.draw(d, self.pos, Some(cur_opt_text), None);
                            cur_opt_i += 1;
                        }
                        WidgetType::OutputConnection => {
                            i.draw(d, self.pos, None, None);
                        }
                        _ => unimplemented!("{:?}", i.widget_type),
                    }
                }
            }
            NodeTypes::Conditional => {
                for i in &self.widgets {
                    i.draw(
                        d,
                        self.pos,
                        Some(node_data.flag_to_check.clone().unwrap()),
                        None,
                    );
                }

                self.draw_lable(d, "Branches:", Vector2 { x: 10., y: 45. });
                self.draw_lable(d, "If true:", Vector2 { x: 10., y: 80. });
                self.draw_lable(d, "If false:", Vector2 { x: 10., y: 115. });
                self.draw_lable(d, "If not set:", Vector2 { x: 10., y: 150. });
            }
            NodeTypes::SetFlag => {
                for i in &self.widgets {
                    i.draw(
                        d,
                        self.pos,
                        Some(node_data.flag_to_set.clone().unwrap()),
                        Some(node_data.value_to_set.clone().unwrap()),
                    );
                }
            }
            NodeTypes::EmitEvent => {
                self.widgets[0].draw(d, self.pos, Some(node_data.event_to_emit.unwrap()), None);

                for (j, wid_i) in self
                    .widgets
                    .iter()
                    .filter(|x| match x.editing_node_member.clone() {
                        None => false,
                        Some(found) => match found {
                            NodeMember::EventDataKey(_) => true,
                            NodeMember::EventDataVal(_) => true,
                            _ => false,
                        },
                    })
                    .collect::<Vec<&Widget>>()
                    .iter()
                    .enumerate()
                {
                    let is_key = j % 2 == 0;

                    let arg = node_data.event_data.clone().unwrap()[j / 2].clone();

                    if is_key {
                        wid_i.draw(d, self.pos, Some(arg.0), None);
                    } else {
                        wid_i.draw(d, self.pos, Some(arg.1), None);
                    }
                }

                self.widgets[self.widgets.len() - 1].draw(d, self.pos, None, None)
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

        d.draw_circle(x_pos, y_pos, corner_radius as f32, Color::PINK);

        if self.card_type == NodeTypes::Branches || self.card_type == NodeTypes::EmitEvent {
            d.draw_circle(
                x_pos + x_size / 2,
                y_pos + y_size,
                corner_radius as f32,
                Color::RED,
            );
            d.draw_line_ex(
                Vector2 {
                    x: (x_pos + x_size / 2 - 8) as f32,
                    y: (y_pos + y_size) as f32,
                },
                Vector2 {
                    x: (x_pos + x_size / 2 + 8) as f32,
                    y: (y_pos + y_size) as f32,
                },
                2.,
                Color::WHITE,
            );
            d.draw_line_ex(
                Vector2 {
                    x: (x_pos + x_size / 2) as f32,
                    y: (y_pos + y_size - 8) as f32,
                },
                Vector2 {
                    x: (x_pos + x_size / 2) as f32,
                    y: (y_pos + y_size + 8) as f32,
                },
                2.,
                Color::WHITE,
            );
        }
    }
}

enum CanvasContextMenuState {
    Hidden,
    NewCard,
}

enum CanvasContextMenuNotification {
    CreateNewCard(NodeTypes),
}

struct CanvasContextMenu {
    state: CanvasContextMenuState,
    pos: Vector2,
    images: HashMap<String, Texture2D>,
}

impl CanvasContextMenu {
    fn update(
        &mut self,
        d: &RaylibHandle,
        m_w_pos: Vector2,
    ) -> Option<CanvasContextMenuNotification> {
        if !d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            return None;
        }

        match self.state {
            CanvasContextMenuState::Hidden => {}
            CanvasContextMenuState::NewCard => {
                let hovering = ((m_w_pos - self.pos).x / 30.).floor() as i64;
                if hovering > 4
                    || hovering < 0
                    || m_w_pos.y < self.pos.y
                    || m_w_pos.y > self.pos.y + 30.
                {
                    self.state = CanvasContextMenuState::Hidden;
                    return None;
                }

                self.state = CanvasContextMenuState::Hidden;
                match hovering {
                    0 => {
                        return Some(CanvasContextMenuNotification::CreateNewCard(
                            NodeTypes::Dialogue,
                        ));
                    }
                    1 => {
                        return Some(CanvasContextMenuNotification::CreateNewCard(
                            NodeTypes::Branches,
                        ));
                    }
                    2 => {
                        return Some(CanvasContextMenuNotification::CreateNewCard(
                            NodeTypes::SetFlag,
                        ));
                    }
                    3 => {
                        return Some(CanvasContextMenuNotification::CreateNewCard(
                            NodeTypes::Conditional,
                        ));
                    }
                    4 => {
                        return Some(CanvasContextMenuNotification::CreateNewCard(
                            NodeTypes::EmitEvent,
                        ))
                    }
                    _ => {
                        panic!()
                    }
                }
            }
        }
        return None;
    }

    fn draw(&self, d: &mut RaylibMode2D<'_, RaylibDrawHandle>, mouse_world_pos: Vector2) {
        match self.state {
            CanvasContextMenuState::Hidden => {}
            CanvasContextMenuState::NewCard => {
                d.draw_rectangle(self.pos.x as i32, self.pos.y as i32, 150, 30, Color::PINK);
                d.draw_texture(
                    self.images.get("new_card").unwrap(),
                    self.pos.x as i32,
                    self.pos.y as i32,
                    Color::WHITE,
                );

                let hovering = ((mouse_world_pos - self.pos).x / 30.).floor() as i64;

                if hovering < 5
                    && hovering >= 0
                    && mouse_world_pos.y > self.pos.y
                    && mouse_world_pos.y < self.pos.y + 30.
                {
                    d.draw_rectangle(
                        self.pos.x as i32 + (hovering * 30) as i32,
                        self.pos.y as i32,
                        30,
                        30,
                        Color {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: 50,
                        },
                    );
                }
            }
        }
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

    // mouse state
    // TODO: Maybe move the mouse state to a separate struct
    mouse_sate: CanvasMouseState,
    last_l_mouse_pressed: f32,
    last_r_mouse_pressed: f32,

    context_menu: CanvasContextMenu,
}

impl CanvasScene {
    fn save_to_file(&self) -> bool {
        let mut dialogue = nfd::dialog_save();
        let dialogue = dialogue.filter("json");
        let res = dialogue.open();
        let mut path = "".to_string();
        match res {
            Ok(nfd::Response::Okay(file_path)) => {
                println!("SAVE_FILE_INFO: File selected: {}", file_path);
                path = file_path;
                if !path.ends_with(".json") {
                    path.push_str(".json");
                }
            }
            Ok(nfd::Response::Cancel) => {
                println!("SAVE_FILE_INFO: User cancelled the dialog");
                return false;
            }
            Ok(nfd::Response::OkayMultiple(_)) => {
                println!("SAVE_FILE_ERR: Tried to open multiple files when it shouldn't?");
                return false;
            }
            Err(error) => {
                println!("SAVE_FILE_ERR: {}", error);
                return false;
            }
        }

        let mut obj = JsonObject::new();

        for n in &self.node_pool {
            obj.push_obj(&n.id);
            let sub_obj = obj.get_obj_ref(&n.id).unwrap();
            match n.node_type {
                NodeTypes::Dialogue => {
                    // obj.set_string("id", n.id.as_str());
                    sub_obj.set_string("type", "dialogue");
                    sub_obj.set_string("character", &n.character.clone().unwrap());
                    sub_obj.set_string("dialogue", &n.dialogue.clone().unwrap());
                    sub_obj.set_string("next", &n.front_links[0]);
                }
                NodeTypes::Branches => {
                    sub_obj.set_string("type", "branches");
                    sub_obj.push_obj("branches");
                    let exits = sub_obj.get_obj_ref("branches").unwrap();
                    for (i, o) in n.branches.clone().unwrap().iter().enumerate() {
                        exits.set_string(o, &n.front_links[i]);
                    }
                }
                NodeTypes::Conditional => {
                    sub_obj.set_string("type", "conditional");
                    sub_obj.set_string("flag_to_check", &n.flag_to_check.clone().unwrap());
                    sub_obj.push_obj("if");
                    let exits = sub_obj.get_obj_ref("if").unwrap();
                    exits.set_string("true", &n.front_links[0]);
                    exits.set_string("false", &n.front_links[1]);
                    exits.set_string("not_set", &n.front_links[2]);
                }
                NodeTypes::SetFlag => {
                    sub_obj.set_string("type", "set_flag");
                    sub_obj.set_string("flag_to_set", &n.flag_to_set.clone().unwrap());
                    sub_obj.set_bool("value", n.value_to_set.clone().unwrap());
                    sub_obj.set_string("next", &n.front_links[0]);
                }
                NodeTypes::EmitEvent => {
                    sub_obj.set_string("type", "emit_event");
                    sub_obj.set_string("event", &n.event_to_emit.clone().unwrap());
                    sub_obj.push_obj("args");
                    let exits = sub_obj.get_obj_ref("args").unwrap();
                    for o in n.event_data.clone().unwrap() {
                        exits.set_string(&o.0, &o.1);
                    }
                    sub_obj.set_string("next", &n.front_links[0]);
                }
                _ => unimplemented!("{:?}", n.node_type),
            }
            // obj.print();
        }

        let file_content = obj.stringify();
        match std::fs::write(path, file_content) {
            Ok(()) => println!("SAVE_FILE_INFO: File written successfully"),
            Err(e) => println!("SAVE_FILE_ERR: {:?}", e),
        }

        true
    }

    fn load_from_file(&mut self) -> bool {
        let res = nfd::open_file_dialog(Some("json"), None);
        let mut path = "".to_string();
        match res {
            Ok(nfd::Response::Okay(file_path)) => {
                println!("LOAD_FILE_INFO: File selected: {}", file_path);
                path = file_path;
            }
            Ok(nfd::Response::Cancel) => {
                println!("LOAD_FILE_INFO: User cancelled the dialog");
                return false;
            }
            Ok(nfd::Response::OkayMultiple(_)) => {
                println!("LOAD_FILE_INFO: Tried to open multiple files when it shouldn't?");
                return false;
            }
            Err(error) => {
                println!("LOAD_FILE_ERR: {}", error);
                return false;
            }
        }

        let file_res = std::fs::read(path);
        let file_content;
        match file_res {
            Ok(res) => {
                file_content = String::from_utf8(res).unwrap();
            }
            Err(err) => {
                println!("LOAD_FILE_ERR: {}", err);
                return false;
            }
        }

        let mut parser = Parser::new();
        parser.load(file_content);
        let parsed_obj = parser.parse();

        self.node_pool.clear();
        self.cards.clear();

        for (n_id, n_obj) in parsed_obj.children {
            match n_obj {
                JsonType::Object(obj) => match obj.get_string("type") {
                    Ok(n_type) => match n_type.as_str() {
                        "dialogue" => self.node_pool.push(Node::new_dialogue(
                            n_id,
                            obj.get_string("character").unwrap(),
                            obj.get_string("dialogue").unwrap(),
                            vec![obj.get_string("next").unwrap()],
                        )),
                        "branches" => {
                            let mut branches_vec: Vec<String> = vec![];
                            let mut front_vec: Vec<String> = vec![];

                            for exit in obj.get_obj("branches").unwrap().children {
                                branches_vec.push(exit.0);

                                match exit.1 {
                                    JsonType::String(next_node) => front_vec.push(next_node),
                                    _ => {
                                        println!("LOAD_FILE_ERR: branches' exits must be Strings.");
                                        return false;
                                    }
                                }
                            }

                            self.node_pool
                                .push(Node::new_branches(n_id, branches_vec, front_vec))
                        }
                        "conditional" => {
                            let exits = obj.get_obj("if").unwrap();
                            self.node_pool.push(Node::new_conditional(
                                n_id,
                                obj.get_string("flag_to_check").unwrap(),
                                vec![
                                    exits.get_string("true").unwrap(),
                                    exits.get_string("false").unwrap(),
                                    exits.get_string("not_set").unwrap(),
                                ],
                            ))
                        }
                        "set_flag" => self.node_pool.push(Node::new_set_flag(
                            n_id,
                            obj.get_string("flag_to_set").unwrap(),
                            obj.get_bool("value").unwrap(),
                            vec![obj.get_string("next").unwrap()],
                        )),
                        "emit_event" => {
                            let mut arg_vec = vec![];

                            for arg in obj.get_obj("args").unwrap().children {
                                let val = match arg.1 {
                                    JsonType::String(found_val) => found_val,
                                    _ => {
                                        println!("LOAD_FILE_ERR: Options' exits must be Strings.");
                                        return false;
                                    }
                                };

                                arg_vec.push((arg.0, val));
                            }

                            self.node_pool.push(Node::new_emit_event(
                                n_id,
                                obj.get_string("event").unwrap(),
                                arg_vec,
                                vec![obj.get_string("next").unwrap()],
                            ))
                        }
                        _ => unimplemented!("{}", n_type),
                    },
                    Err(err) => {
                        println!("LOAD_FILE_ERR: {:?}", err);
                        return false;
                    }
                },
                _ => return false,
            }
        }

        self.parse_node_pool();

        true
    }

    fn get_free_node_id(&self) -> String {
        'outer_loop: for i in 1..99999 {
            let cur_i = format!("{:0>5}", i.to_string()); // i with left 0 tabs

            for j in &self.node_pool {
                if j.id == cur_i {
                    continue 'outer_loop;
                }
            }

            return cur_i;
        }

        panic!("There isn't enough ids.");
    }

    fn get_node_ref<'a>(&'a mut self, id: &String) -> &'a mut Node {
        for n in &mut self.node_pool {
            if n.id == id.as_str() {
                return n;
            }
        }

        unreachable!()
    }

    fn get_card_i(&self, id: String) -> usize {
        for (i, c) in self.cards.iter().enumerate() {
            if c.node_ref == id {
                return i;
            }
        }

        unreachable!()
    }

    fn get_mouse_world_pos(&self, rl: &RaylibHandle) -> Vector2 {
        rl.get_screen_to_world2D(rl.get_mouse_position(), self.cam)
    }

    fn update(&mut self, rl: &RaylibHandle, last_mouse_pos: &mut Vector2) {
        self.last_l_mouse_pressed += rl.get_frame_time();
        self.last_r_mouse_pressed += rl.get_frame_time();

        match &self.state {
            CanvasSceneStates::Roaming => {
                self.update_roaming(rl, last_mouse_pos);
            }
            CanvasSceneStates::EditingTextInput(_, _) => {}
            _ => unimplemented!(),
        }
    }

    pub fn update_roaming(&mut self, rl: &RaylibHandle, last_mouse_pos: &mut Vector2) {
        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            let test = self.save_to_file();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_L) {
            let test = self.load_from_file();
        }

        let context_menu_notification = self.context_menu.update(rl, self.get_mouse_world_pos(rl));
        match context_menu_notification {
            None => {}
            Some(notification) => match notification {
                CanvasContextMenuNotification::CreateNewCard(node_type) => {
                    let new_id = self.get_free_node_id();
                    match node_type {
                        NodeTypes::Dialogue => {
                            let mut new_node = Node::default_dialogue();
                            new_node.id = new_id.clone();
                            new_node.front_links = vec!["".to_string()];
                            self.node_pool.push(new_node);

                            let new_card = Card::new_dialogue(new_id, self.get_mouse_world_pos(rl));
                            self.cards.push(new_card);
                        }
                        NodeTypes::Branches => {
                            let mut new_node = Node::default_branches();
                            new_node.id = new_id.clone();
                            self.node_pool.push(new_node);

                            let new_card =
                                Card::new_branches(new_id, vec![], self.get_mouse_world_pos(rl));
                            self.cards.push(new_card);
                        }
                        NodeTypes::SetFlag => {
                            let mut new_node = Node::default_set_flag();
                            new_node.id = new_id.clone();
                            new_node.front_links = vec!["".to_string()];
                            new_node.flag_to_set = Some("".to_string());
                            self.node_pool.push(new_node);

                            let new_card = Card::new_set_flag(new_id, self.get_mouse_world_pos(rl));
                            self.cards.push(new_card);
                        }
                        NodeTypes::Conditional => {
                            let mut new_node = Node::default_conditional();
                            new_node.id = new_id.clone();
                            new_node.front_links =
                                vec!["".to_string(), "".to_string(), "".to_string()];
                            self.node_pool.push(new_node);

                            let new_card =
                                Card::new_conditional(new_id, self.get_mouse_world_pos(rl));
                            self.cards.push(new_card);
                        }
                        NodeTypes::EmitEvent => {
                            let new_node = Node::new_emit_event(
                                new_id.clone(),
                                "".to_string(),
                                vec![],
                                vec!["".to_string()],
                            );
                            self.node_pool.push(new_node);

                            let new_card =
                                Card::new_emit_event(new_id, vec![], self.get_mouse_world_pos(rl));
                            self.cards.push(new_card);
                        }

                        _ => unimplemented!("{:?}", node_type),
                    }
                }
            },
        }

        let mut post_handle_notification = None;

        // mouse update
        match &self.mouse_sate {
            CanvasMouseState::Roaming => {
                if rl.is_mouse_button_released(MouseButton::MOUSE_RIGHT_BUTTON) {
                    if self.last_r_mouse_pressed < 0.2 {
                        self.context_menu.pos = self.get_mouse_world_pos(rl);
                        self.context_menu.state = CanvasContextMenuState::NewCard;
                    }
                }
            }
            CanvasMouseState::CreatingConnection(ref_id, i) => {
                if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON)
                    && self.last_l_mouse_pressed > 0.5
                // this check allows both ways of creating connection
                {
                    // TODO: Create function that gets the position of the card input

                    let mut found = "".to_string();
                    for c in &self.cards {
                        // found the card it will be linked to, it's c
                        // writing this code made my head hurt
                        if self.get_mouse_world_pos(rl).distance_to(c.pos) < 10. {
                            found = c.node_ref.clone();
                            break;
                        }
                    }
                    if found != "" {
                        // this is getting too confusing
                        let node_output_i = self
                            .copy_card_data(&ref_id)
                            .from_output_widget_i_to_node_front_link_i(&i);

                        self.get_node_ref(&ref_id.clone()).front_links[node_output_i] = found;
                    }

                    self.mouse_sate = CanvasMouseState::Roaming;
                }
                return;
            }
            CanvasMouseState::MovingCard(id) => {
                // FIXME: strange behavior when moving card and camera at the same time
                let mut delta = rl.get_mouse_position() - *last_mouse_pos;
                delta.scale(-1. / self.cam.zoom);

                for c in &mut self.cards {
                    if c.node_ref == id.as_str() {
                        c.pos -= delta;
                    }
                }

                if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
                    self.mouse_sate = CanvasMouseState::Roaming;
                }
            }
            _ => unimplemented!("{:?}", self.mouse_sate),
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
            self.last_r_mouse_pressed = 0.;
        }
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            self.last_l_mouse_pressed = 0.;
        }

        let m_pos = self.get_mouse_world_pos(rl);
        for c in self.cards.iter_mut() {
            let notify = c.update(rl, m_pos.clone());

            match notify {
                Some(notification_type) => match notification_type {
                    CardNotification::EditTextInput { id, node_member } => {
                        self.state = CanvasSceneStates::EditingTextInput(id, node_member);
                        return;
                    }
                    CardNotification::AddBranchToBranchesNode(id) => {
                        post_handle_notification =
                            Some(CardNotification::AddBranchToBranchesNode(id));
                    }
                    CardNotification::AddArgToEmitEventNode(id) => {
                        post_handle_notification =
                            Some(CardNotification::AddArgToEmitEventNode(id));
                    }
                    CardNotification::ToggleCheckBox { id, node_member } => {
                        post_handle_notification =
                            Some(CardNotification::ToggleCheckBox { id, node_member });
                    }
                    CardNotification::CreatingCardConnection(id, i) => {
                        self.mouse_sate = CanvasMouseState::CreatingConnection(id.clone(), i);

                        let output_i = c.from_output_widget_i_to_node_front_link_i(&i);

                        self.get_node_ref(&id).front_links[output_i] = "".to_string();
                        return;
                    }
                    CardNotification::MovingCard(id) => {
                        self.mouse_sate = CanvasMouseState::MovingCard(id);
                        return;
                    }
                    _ => {
                        unimplemented!("{:?}", notification_type)
                    }
                },
                None => {}
            }
        }

        match post_handle_notification {
            None => {}
            Some(notification) => match notification {
                CardNotification::AddBranchToBranchesNode(id) => {
                    let pos = self.copy_card_data(&id).pos;

                    let mut cur_node = self.get_node_ref(&id);
                    let mut next_node_opt_vec = cur_node.branches.clone().unwrap();
                    next_node_opt_vec.push("Empty".to_string());
                    cur_node.branches = Some(next_node_opt_vec);
                    let mut next_node_exit_vec = cur_node.front_links.clone();
                    next_node_exit_vec.push("".to_string());
                    cur_node.front_links = next_node_exit_vec;

                    let new_card = Card::new_branches(
                        cur_node.id.clone(),
                        cur_node.clone().branches.unwrap(),
                        pos,
                    );

                    let i = self.get_card_i(id);
                    self.cards[i] = new_card;
                }
                CardNotification::ToggleCheckBox { id, node_member } => {
                    let mut i = 0;
                    for j in &self.node_pool {
                        if j.id == id {
                            break;
                        }
                        i += 1;
                    }

                    let mut cur_node = &mut self.node_pool[i];

                    match node_member {
                        NodeMember::ValueToSet => {
                            cur_node.value_to_set = Some(!cur_node.value_to_set.clone().unwrap());
                        }
                        _ => unimplemented!("{:?}", node_member),
                    }
                }
                CardNotification::AddArgToEmitEventNode(id) => {
                    let pos = self.copy_card_data(&id).pos;

                    let mut cur_node = self.get_node_ref(&id);
                    let mut next_node_arg_vec = cur_node.event_data.clone().unwrap();
                    next_node_arg_vec.push(("".to_string(), "".to_string()));
                    cur_node.event_data = Some(next_node_arg_vec);

                    let new_card = Card::new_emit_event(
                        cur_node.id.clone(),
                        cur_node.clone().event_data.unwrap(),
                        pos,
                    );

                    let i = self.get_card_i(id);
                    self.cards[i] = new_card;
                }
                _ => unimplemented!("{:?}", notification),
            },
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
            i.draw(d, self.copy_node_data(&i.node_ref));
        }

        self.draw_card_connections(d);
        self.context_menu.draw(d, self.get_mouse_world_pos(d));
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

        // let keymap = [
        //     ('A', KeyboardKey::KEY_A),
        //     ('B', KeyboardKey::KEY_B),
        //     ('C', KeyboardKey::KEY_C),
        //     ('D', KeyboardKey::KEY_D),
        //     ('E', KeyboardKey::KEY_E),
        //     ('F', KeyboardKey::KEY_F),
        //     ('G', KeyboardKey::KEY_G),
        //     ('H', KeyboardKey::KEY_H),
        //     ('I', KeyboardKey::KEY_I),
        //     ('J', KeyboardKey::KEY_J),
        //     ('K', KeyboardKey::KEY_K),
        //     ('L', KeyboardKey::KEY_L),
        //     ('M', KeyboardKey::KEY_M),
        //     ('N', KeyboardKey::KEY_N),
        //     ('O', KeyboardKey::KEY_O),
        //     ('P', KeyboardKey::KEY_P),
        //     ('Q', KeyboardKey::KEY_Q),
        //     ('R', KeyboardKey::KEY_R),
        //     ('S', KeyboardKey::KEY_S),
        //     ('T', KeyboardKey::KEY_T),
        //     ('U', KeyboardKey::KEY_U),
        //     ('V', KeyboardKey::KEY_V),
        //     ('W', KeyboardKey::KEY_W),
        //     ('X', KeyboardKey::KEY_X),
        //     ('Y', KeyboardKey::KEY_Y),
        //     ('Z', KeyboardKey::KEY_Z),
        // ];

        let mut cur_text;

        match &self.state {
            CanvasSceneStates::EditingTextInput(wte, member) => match member {
                NodeMember::Character => cur_text = self.copy_node_data(&wte).character.unwrap(),
                NodeMember::Dialogue => cur_text = self.copy_node_data(&wte).dialogue.unwrap(),
                NodeMember::Branch(i) => {
                    let branches_vec = &self.copy_node_data(&wte).branches.unwrap();
                    cur_text = branches_vec[*i].clone();
                }
                NodeMember::FlagToCheck => {
                    cur_text = self.copy_node_data(&wte).flag_to_check.unwrap()
                }
                NodeMember::FlagToSet => cur_text = self.copy_node_data(&wte).flag_to_set.unwrap(),
                NodeMember::EventToEmit => {
                    cur_text = self.copy_node_data(&wte).event_to_emit.unwrap()
                }
                NodeMember::EventDataKey(i) => {
                    let args_vec = &self.copy_node_data(&wte).event_data.unwrap();
                    cur_text = args_vec[*i].clone().0;
                }
                NodeMember::EventDataVal(i) => {
                    let args_vec = &self.copy_node_data(&wte).event_data.unwrap();
                    cur_text = args_vec[*i].clone().1;
                }
                _ => unimplemented!("{:?}", member),
            },
            _ => panic!("Something has gone incredibly wrong."),
        }

        // I had to copy the code from d.get_key_pressed() since it simply doesn't work
        let pressed_key;
        let key = unsafe { ffi::GetKeyPressed() };
        if key > 0 {
            pressed_key = key_from_i32(key);
        } else {
            pressed_key = None
        }

        match pressed_key {
            Some(key) => {
                let to_ascii = key as u8 + 32;
                if to_ascii >= 97 && to_ascii <= 123 {
                    cur_text.push(to_ascii as char);
                }}
            None => {}
        }

        match &self.state {
            CanvasSceneStates::EditingTextInput(id, member) => {
                for i in &mut self.node_pool {
                    if i.id == id.as_str() {
                        match member {
                            NodeMember::Dialogue => i.dialogue = Some(cur_text.clone()),
                            NodeMember::Character => i.character = Some(cur_text.clone()),
                            NodeMember::Branch(opt_i) => {
                                let mut cur_vec = i.branches.clone().unwrap();
                                cur_vec[*opt_i] = cur_text.clone();
                                i.branches = Some(cur_vec);
                            }
                            NodeMember::FlagToCheck => i.flag_to_check = Some(cur_text.clone()),
                            NodeMember::FlagToSet => i.flag_to_set = Some(cur_text.clone()),
                            NodeMember::EventToEmit => i.event_to_emit = Some(cur_text.clone()),
                            NodeMember::EventDataKey(arg_i) => {
                                let mut cur_vec = i.event_data.clone().unwrap();
                                cur_vec[*arg_i].0 = cur_text.clone();
                                i.event_data = Some(cur_vec);
                            }
                            NodeMember::EventDataVal(arg_i) => {
                                let mut cur_vec = i.event_data.clone().unwrap();
                                cur_vec[*arg_i].1 = cur_text.clone();
                                i.event_data = Some(cur_vec);
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
                    self.cards.push(Card::new_dialogue(i.id.clone(), card_pos));
                    x_offset += 200.;
                }
                NodeTypes::Branches => {
                    let card_pos = Vector2 { x: x_offset, y: 0. };
                    self.cards.push(Card::new_branches(
                        i.id.clone(),
                        i.clone().branches.unwrap(),
                        card_pos,
                    ));
                    x_offset += 200.;
                }
                NodeTypes::Conditional => {
                    let card_pos = Vector2 { x: x_offset, y: 0. };
                    self.cards
                        .push(Card::new_conditional(i.id.clone(), card_pos));
                    x_offset += 200.;
                }
                NodeTypes::SetFlag => {
                    let card_pos = Vector2 { x: x_offset, y: 0. };
                    self.cards.push(Card::new_set_flag(i.id.clone(), card_pos));
                    x_offset += 200.;
                }
                NodeTypes::EmitEvent => {
                    let card_pos = Vector2 { x: x_offset, y: 0. };
                    self.cards.push(Card::new_emit_event(
                        i.id.clone(),
                        i.event_data.clone().unwrap(),
                        card_pos,
                    ));
                    x_offset += 200.;
                }
                _ => unimplemented!("{:?}", i.node_type),
            }
        }
    }

    fn copy_card_data(&self, id: &String) -> Card {
        for i in &self.cards {
            if i.node_ref == id.as_str() {
                return i.clone();
            }
        }

        unreachable!()
    }
    fn copy_node_data(&self, id: &String) -> Node {
        for i in &self.node_pool {
            if i.id == id.as_str() {
                return i.clone();
            }
        }

        unreachable!()
    }

    fn draw_card_connections(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        for i in &self.node_pool {
            let i_card = self.copy_card_data(&i.id);
            let outputs = i_card.copy_output_widgets();

            if i.front_links.len() != outputs.len() {
                println!("ERROR: Something is wrong at 'draw_card_connections'");
                println!("{} e {}", i.front_links.len(), outputs.len());
                continue;
            }

            for j in 0..i.front_links.len() {
                if i.front_links[j] == "" {
                    continue;
                }

                let start_pos = i_card.pos + outputs[j].offset;
                let end_pos = self.copy_card_data(&i.front_links[j]).pos;
                d.draw_line_ex(start_pos, end_pos, 5., Color::PURPLE);
            }
        }

        match &self.mouse_sate {
            CanvasMouseState::CreatingConnection(id, i) => {
                let start_pos = self.copy_card_data(id).pos
                    + self
                        .copy_card_data(id)
                        .widgets
                        .get(i.clone())
                        .unwrap()
                        .offset;
                let end_pos = self.get_mouse_world_pos(d);
                d.draw_line_ex(start_pos, end_pos, 5., Color::PURPLE);
            }
            _ => {}
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Dialogue maker")
        .build();

    rl.set_target_fps(60);

    // TODO: new() function for context menu
    let mut cm_images = HashMap::new();

    let new_card_code = include_bytes!("../assets/context_menu_new_card.png").to_vec();
    let new_card_image =
        Image::load_image_from_mem(".png", &new_card_code, new_card_code.len() as i32).unwrap();
    cm_images.insert(
        "new_card".to_string(),
        rl.load_texture_from_image(&thread, &new_card_image)
            .unwrap(),
    );
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
            // Node::new_dialogue(
            //     "00001",
            //     "John doe",
            //     "Test test testing",
            //     vec!["00002".to_string()],
            // ),
            // Node::new_dialogue(
            //     "00002",
            //     "Second one coming",
            //     "I really hope this doesn't break everything.",
            //     vec!["00003".to_string()],
            // ),
            // Node::new_options(
            //     "00003",
            //     vec![
            //         "Hi".to_string(),
            //         "Bye".to_string(),
            //         "Let's go".to_string(),
            //         "To the conditionals!".to_string(),
            //     ],
            //     vec![
            //         "00001".to_string(),
            //         "00002".to_string(),
            //         "00003".to_string(),
            //         "00004".to_string(),
            //     ],
            // ),
            // Node::new_conditional(
            //     "00004",
            //     "FLAG1",
            //     vec![
            //         "00001".to_string(),
            //         "00005".to_string(),
            //         "00003".to_string(),
            //     ],
            // ),
            // Node::new_set_flag("00005", "FLAG1", true, vec!["00006".to_string()]),
            // Node::new_emit_event(
            //     "00006",
            //     "FLIP_H_SPRITE",
            //     vec![("CHAR_TO_FLIP".to_string(), "CHAR_NAME".to_string())],
            //     vec!["00007".to_string()],
            // ),
            // Node::new_emit_event(
            //     "00007",
            //     "ERR_EXIT",
            //     vec![
            //         ("CODE".to_string(), "001".to_string()),
            //         ("MESSAGE".to_string(), "ERR MESSAGE HERE".to_string()),
            //     ],
            //     vec!["00001".to_string()],
            // ),
        ],
        state: CanvasSceneStates::Roaming,
        mouse_sate: CanvasMouseState::Roaming,
        last_l_mouse_pressed: 0.,
        last_r_mouse_pressed: 0.,
        context_menu: CanvasContextMenu {
            state: CanvasContextMenuState::Hidden,
            pos: Vector2 { x: 0., y: 0. },
            images: cm_images,
        },
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

        // ===== IMGUI LIKE PART =====
        canvas_scene.update_and_draw_text_input_edit(&mut new_d, tlp); // Runs only if canvas state is EditingTextInput

        // new_d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        new_d.draw_fps(tlp.x as i32, tlp.y as i32);
    }
}
