use iced::widget::{button, column, text, text_input};
use iced::{alignment, Alignment, Color, Element, Length, Sandbox, Settings};

pub fn ui() -> iced::Result {
    Falion::run(Settings::default())
}

struct Falion {
    query: String,
}

#[derive(Debug, Clone)]
enum Message {
    Search,
    InputChanged(String),
}

impl Sandbox for Falion {
    type Message = Message;

    fn new() -> Self {
        Self {
            query: "".to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("Falion - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.query = value;
            }
            Message::Search => {}
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text("Falion")
                .width(Length::Fill)
                .size(100)
                .style(Color::from([0.5, 0.5, 0.5]))
                .horizontal_alignment(alignment::Horizontal::Center),
            text_input("Search...", &self.query)
                .id(text_input::Id::unique())
                .on_input(Message::InputChanged)
                .on_submit(Message::Search)
                .padding(15)
                .size(30),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
