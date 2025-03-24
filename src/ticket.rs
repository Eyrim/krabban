use std::{fmt::Display};

use ratatui::{
    prelude::{self, Buffer, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, List, ListDirection, ListItem, Widget},
};

#[derive(Debug)]
pub struct Ticket {
    title: String,
    description: String,
    is_complete: bool,
    subtasks: Vec<Ticket>,
}

impl Ticket {
    pub fn new(title: String, description: String) -> Ticket {
        Ticket {
            title,
            description,
            is_complete: false,
            subtasks: vec![],
        }
    }
}

impl Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n{}", self.title, self.description)
    }
}

impl From<&Ticket> for ListItem<'_> {
    fn from(value: &Ticket) -> Self {
        ListItem::new(format!("{}", value))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Swimlane<'a> {
    pub name: String,
    pub tickets: Vec<&'a Ticket>,
}

impl <'a>Swimlane<'a> {
    pub fn new(name: String) -> Swimlane<'a> {
        Swimlane {
            name,
            tickets: vec![],
        }
    }
}

impl Widget for Swimlane<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::new().title(self.name);

        List::new(self.tickets)
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom)
            .render(area, buf);
    }
}
