use cosmic::app::{Command, Core};
use cosmic::iced::widget::{button, column, text};
use cosmic::{executor, Application, ApplicationExt, Element};
use cosmic::iced::Length;
use cosmic::iced_core::alignment;

pub struct Window {
    core: Core,
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Messages {
    IncPressed,
    DecPressed,
}

impl Application for Window {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Messages;

    const APP_ID: &'static str = "com.jwestall.LibcosmicDemo";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut win = Window {
            core,
            value: 0,
        };

        win.set_header_title("Libcosmic Demo".into());
        let cmd = win.set_window_title("Libcosmic Demo".into());

        (win, cmd)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Messages::IncPressed => self.value += 1,
            Messages::DecPressed => self.value -= 1,
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            button(text("Add").width(Length::Fill).horizontal_alignment(alignment::Horizontal::Center))
                .on_press(Messages::IncPressed)
                .width(Length::Fill),
            text(self.value)
                .width(Length::Fill)
                .height(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
            button(text("Subtract").width(Length::Fill).horizontal_alignment(alignment::Horizontal::Center))
                .on_press(Messages::DecPressed)
                .width(Length::Fill)
        ]
        .into()
    }
}