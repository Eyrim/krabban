use std::{io, ptr::slice_from_raw_parts_mut};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::ticket::{Swimlane, Ticket};

#[derive(Debug, Default)]
pub struct App<'a> {
    swimlanes: Vec<&'a Swimlane<'a>>,
    current_page: Pages,
    should_exit: bool,
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Krabban".bold());
        let keybinds = Line::from(vec![
            " (h)elp ".into(),
            " (n)ew ".into(),
            " (e)nter ".into(),
            " (b)ack ".into(),
            " (s)ubtask ".into(),
            " (q)uit ".into(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(keybinds.centered())
            .border_set(border::ROUNDED);

        self.swimlanes
            .iter()
            .for_each(|swimlane| swimlane.clone().clone().render(area, buf));

        Paragraph::new("")
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl <'a>App<'a> {
    pub fn new() -> App<'a> {
        let mut swimlanes = Vec::new();
        let todo = Swimlane {
            name: "To Do".to_owned(),
            tickets: vec![],
        };

        swimlanes.push(&todo);

        App {
            swimlanes,
            current_page: Pages::Main,
            should_exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn build_swimlane_layout(lane_count: i32) -> Layout {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Self::build_swimlane_constraints(lane_count))
    }

    fn build_swimlane_constraints(n: i32) -> Vec<Constraint> {
        let mut constraints: Vec<Constraint> = vec![];
        let percentage_per_lane = 100 / n;
        
        for _ in 0..n {
            constraints.push(Constraint::Percentage(percentage_per_lane .try_into().unwrap()));
        }

        constraints
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.should_exit = true
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }

        Ok(())
    }

    fn page(&mut self, new_page: Pages) {
        self.current_page = new_page
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('n') => self.page(Pages::NewTicket),
            _ => {}
        }
    }
}

#[derive(Debug, Default)]
enum Pages {
    #[default]
    Main,
    NewTicket,
}
