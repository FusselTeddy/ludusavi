use iced::{alignment::Horizontal as HorizontalAlignment, Font, Length, Text};

pub const ICONS: Font = Font::External {
    name: "Material Icons",
    bytes: include_bytes!("../../assets/MaterialIcons-Regular.ttf"),
};

pub enum Icon {
    Add,
    AddCircle,
    #[allow(dead_code)]
    Edit,
    FolderOpen,
    KeyboardArrowRight,
    KeyboardArrowDown,
    #[allow(dead_code)]
    Language,
    OpenInNew,
    Remove,
    RemoveCircle,
    Search,
    SubdirectoryArrowRight,
    Delete,
    #[allow(dead_code)]
    PlayCircleOutline,
    Settings,
    MoreVert,
}

impl Icon {
    pub fn as_char(&self) -> char {
        match self {
            Self::Add => '\u{E145}',
            Self::AddCircle => '\u{E147}',
            Self::Edit => '\u{E150}',
            Self::FolderOpen => '\u{E2C8}',
            Self::KeyboardArrowRight => '\u{E315}',
            Self::KeyboardArrowDown => '\u{E313}',
            Self::Language => '\u{E894}',
            Self::OpenInNew => '\u{E89E}',
            Self::Remove => '\u{E15B}',
            Self::RemoveCircle => '\u{E15C}',
            Self::Search => '\u{E8B6}',
            Self::SubdirectoryArrowRight => '\u{E5DA}',
            Self::Delete => '\u{E872}',
            Self::PlayCircleOutline => '\u{E039}',
            Self::Settings => '\u{E8B8}',
            Self::MoreVert => '\u{E5D4}',
        }
    }

    pub fn as_text(&self) -> Text {
        Text::new(&self.as_char().to_string())
            .font(ICONS)
            .width(Length::Units(60))
            .horizontal_alignment(HorizontalAlignment::Center)
    }
}
