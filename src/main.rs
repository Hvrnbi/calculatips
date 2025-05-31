// GUI crate
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Column, Container},
    Element, Length, window, Size, Font,
};
use iced_aw::{style::tab_bar, TabLabel, Tabs};
// Import of the calculator tab
mod calculator;
use calculator::{CalculatorMessage,CalculatorTab};

mod settings;
use settings::{SettingsMessage,SettingsTab};


const TAB_PADDING: u16 = 8;
const ICON_BYTES: &[u8] = include_bytes!("./fonts/icons.ttf");
const ICON: Font = Font::with_name("icons");

enum Icon {
    Calc,
    Settings,
}

impl From<Icon> for char {
    fn from(icon:Icon) -> Self {
        match icon {
            Icon::Calc => '\u{F1EC}',
            Icon::Settings => '\u{E802}',
        }
    }
}

fn main() -> iced::Result {
    iced::application("Calculatips", Calculatips::update, Calculatips::view)
        .theme(Calculatips::theme)
        .window(window::Settings {
            size : Size { width: (405.), height: (720.) },
            ..window::Settings::default()
        })
        .font(ICON_BYTES)
        .run()
}

#[derive(Debug, Clone)]
enum Message{
    TabSelected(TabId),
    TabClosed(TabId),
    Calculator(CalculatorMessage),
    Settings(SettingsMessage),
}

#[derive(Default)]
struct Calculatips{
    active_tab: TabId,
    calculator_tab: CalculatorTab,
    settings_tab: SettingsTab,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
enum TabId{
    #[default]
    Calculator,
    Settings,
}

impl Calculatips {
    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::TabClosed(id) => println!("Tab {:?} closed", id),
            Message::Calculator(message) => self.calculator_tab.update(message),
            Message::Settings(message) => self.settings_tab.update(message),
        }
    }

    fn view(&self) -> Element<'_, Message> {

        Tabs::new(Message::TabSelected)
            .tab_icon_position(iced_aw::tabs::Position::Top)
            .on_close(Message::TabClosed)
            .push(
                TabId::Calculator,
                self.calculator_tab.tab_label(),
                self.calculator_tab.view()
            )
            .push(
                TabId::Settings,
                self.settings_tab.tab_label(),
                self.settings_tab.view(),
            )
            .set_active_tab(&self.active_tab)
            .tab_bar_style(Box::new(tab_bar::primary))
            .icon_font(ICON)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        self.settings_tab.theme.clone()
    }

}


trait Tab {
    type Message;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(self.content())
            .align_x(iced::Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}