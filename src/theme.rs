use iced::{Theme, widget::button};

pub fn ripple_button(theme: &Theme, status: button::Status) -> button::Style {
    let _ = theme;
    let normal = button::Style {
        background: None,
        text_color: iced::Color::from_rgb8(248, 250, 252),
        border: iced::Border { color: iced::Color::from_rgba8(248, 250, 252, 0.30), width: 1.0, radius: 6.0.into() },
        shadow: iced::Shadow::default(),
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb8(248, 250, 252))),
            text_color: iced::Color::from_rgb8(11, 15, 25),
            border: iced::Border { color: iced::Color::from_rgb8(248, 250, 252), width: 1.0, radius: 6.0.into() },
            shadow: iced::Shadow::default(),
        },
        button::Status::Pressed => button::Style { transform: iced::Vector::new(0.98, 0.98), ..normal },
        _ => normal,
    }
}

pub fn section(theme: &Theme) -> iced::widget::container::Style {
    let _ = theme;
    iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb8(15, 21, 34))),
        text_color: Some(iced::Color::from_rgb8(248, 250, 252)),
        border: iced::Border { color: iced::Color::from_rgb8(38, 48, 65), width: 1.0, radius: 8.0.into() },
        shadow: iced::Shadow::default(),
    }
}
