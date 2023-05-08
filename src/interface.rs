use iced::{ Color, Length, Sandbox, Alignment, Renderer, Settings, Element, window::Settings as WindowSettings};
use iced::widget::{ column, container, button, row, text, text_input };
use iced::theme::{self, Theme};

use crate::attack::*;
use crate::file::*;
use crate::rainbow_table::Node;

use hex;

pub fn main() -> iced::Result {
    RainbowTableAttack::run(Settings {
        window: WindowSettings {
            size: (500, 350),
            resizable: false,
            decorations: true,
            ..WindowSettings::default()
        },
        ..Settings::default()
    })
}

pub struct RainbowTableAttack {
    steps: State,
    hash: String,
    size: u8,
    result: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementSize,
    DecrementSize,
    InputChanged(String),
    AttackPressed,
    Back
}

impl Sandbox for RainbowTableAttack {
    type Message = Message;

    fn new() -> RainbowTableAttack {
        RainbowTableAttack {
            steps: State::new(),
            hash: String::from(""),
            size: 1,
            result: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("Rainbow Table Attack")
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementSize => {
                if self.size < 7 {
                    self.size += 1;
                }
            }
            Message::DecrementSize => {
                if self.size != 1 {
                    self.size -= 1;
                }
            }
            Message::InputChanged(value) => self.hash = value,
            Message::Back => {
                self.steps = State::Form;
            }
            Message::AttackPressed => {

                self.steps.next();

                let file_path = [
                    [ "RainbowTable_1_1_30_5.json", "RainbowTable_1_2_30_5.json", "RainbowTable_1_3_30_5.json" ], 
                    [ "RainbowTable_2_1_477_9.json", "RainbowTable_2_2_477_9.json", "RainbowTable_2_3_477_9.json" ], 
                    [ "RainbowTable_3_1_3444_68.json", "RainbowTable_3_2_3444_68.json", "RainbowTable_3_3_3444_68.json" ], 
                    [ "RainbowTable_4_1_24049_480.json", "RainbowTable_4_2_24049_480.json", "RainbowTable_4_3_24049_480.json" ], 
                    [ "RainbowTable_5_1_x_x.json", "RainbowTable_5_2_x_x.json", "RainbowTable_5_3_x_x.json" ], 
                    [ "RainbowTable_6_1_x_x.json", "RainbowTable_6_2_x_x.json", "RainbowTable_6_3_x_x.json" ], 
                ];

                for file in file_path[(self.size-1) as usize] {

                    let data: &str = file.trim_end_matches(".json");

                    let numbers: Vec<u32> = data
                    .split('_')
                    .skip(1)
                    .map(|s| s.parse().unwrap_or(0))
                    .collect();

                    let size = numbers[0] as u8;
                    let id = numbers[1] as u8;
                    let nb_password = numbers[2];
                    let nb_node = numbers[3];

                    let hash_bytes = hex::decode(&self.hash).unwrap().try_into().unwrap();

                    let mut rainbow_table: Vec<Node> = deserialize(&format!("RainbowTable_{}_{}_{}_{}.json", size, id, nb_password, nb_node)).unwrap();

                    let result = execution(&mut rainbow_table, hash_bytes, nb_node, nb_password, size);
                    
                    match result {
                        None => { self.steps.loaded(false); continue },
                        Some(value) => { 
                            self.result = value;
                            self.steps.loaded(true);
                            return;
                        },
                    }
                }
            },
        }
    }
    fn view(&self) -> Element<Message> {

        let text_input: iced_native::widget::text_input::TextInput<'_, Message, Renderer> = text_input("SHA 3 hash...", &self.hash)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);
        

        let input_password = column![
            text("Insérez le hash SHA-3 à casser").size(20),
            row![
                text_input, 
            ],
        ]
        .height(85)
        .spacing(10);

        let input_size = column![
            text("Taille du mot de passe").size(20),
            row![
                button("-").on_press(Message::DecrementSize),
                text(self.size).size(20),
                button("+").on_press(Message::IncrementSize),
            ]
            .spacing(10),
        ]
        .height(80)
        .spacing(10);

        let button_attack = column![
            button("Attack").on_press(Message::AttackPressed),
        ]
        .width(Length::Fill)
        .height(50)
        .align_items(Alignment::Center);

        let retry = column![
            button("Retour").on_press(Message::Back),
        ]
        .width(Length::Fill)
        .height(50)
        .align_items(Alignment::Center);

        let success = column![
            text("L'attaque à réussi ! ").size(40),
            row![
                text("Le mot de passe est : ").size(20),
                text(format!("{:?}", self.result)).size(20),
            ]
        ]
        .width(Length::Fill)
        .height(100)
        .spacing(20)
        .align_items(Alignment::Center);

        let fail = column![
            text("L'attaque a échoué ! ").size(40),
        ]
        .width(Length::Fill)
        .height(100)
        .spacing(20)
        .align_items(Alignment::Center);

        let mut controls = column![];

        match &self.steps {
            State::Form => {
                controls = controls
                .push(input_password)
                .push(input_size)
                .push(button_attack);
            }
            State::Loaded => {
                controls = controls
            }
            State::Success => {
                controls = controls
                .push(success)
                .push(retry);
            }
            State::Fail => {
                controls = controls
                .push(fail)
                .push(retry);
            }
            State::Error => {

            }
        }

        let content: Element<_> = column![
            controls,
        ]
        .max_width(500)
        .spacing(20)
        .padding(20)
        .into();

        container(content).height(Length::Fill).center_y().into()
    }
    fn theme(&self) -> Theme {
        Theme::custom(theme::Palette {
            background: Color::from_rgb(0.14, 0.16, 0.2),
            text: Color::WHITE,
            primary: Color::from_rgb(19.0/255.0, 136.0/255.0, 113.0/255.0),
            success: Color::from_rgb(0.0, 1.0, 0.0),
            danger: Color::from_rgb(1.0, 0.0, 0.0),
        })
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Form,
    Loaded,
    Success,
    Fail,
    Error
}

impl State {
    fn new() -> State {
        State::Form
    }

    fn next(&mut self) {
        match self {
            State::Form => { *self = State::Loaded},
            State::Loaded => { *self = State::Form},
            State::Success => { *self = State::Form},
            State::Fail => { *self = State::Form},
            State::Error => { *self = State::Error},
        }
    }

    fn loaded(&mut self, success: bool) {
        if success {
            *self = State::Success;
        }
        else {
            *self = State::Fail;
        }
    }
}