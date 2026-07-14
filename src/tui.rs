#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{Block, Padding};
use ratatui::widgets::Widget;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::text::Text;
use ratatui::symbols::border;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Borders;

use time::{Date, Month, OffsetDateTime};

pub fn enter_tui() -> io::Result<()> {
       ratatui::run(|terminal| App::default().run(terminal))
} 

#[derive(Debug)]
pub struct App {
    exit: bool,
    date_cursor: Date,
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            date_cursor: OffsetDateTime::now_local().expect("U oh no").date(),
        }
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }


    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.exit(),
            KeyCode::Tab => self.next_day(),
            KeyCode::Char('l') => self.next_day(),
            KeyCode::Right => self.next_day(),
            KeyCode::Backspace=> self.prev_day(),
            KeyCode::Left => self.prev_day(),
            KeyCode::Char('h') => self.prev_day(),
            KeyCode::Down=> for _ in 0..7 {self.next_day()},
            KeyCode::Char('j') => for _ in 0..7 {self.next_day()},
            KeyCode::Up=> for _ in 0..7 {self.prev_day()},
            KeyCode::Char('k') => for _ in 0..7 {self.prev_day()},
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    
    fn next_day(&mut self) {
        self.date_cursor = self.date_cursor.next_day().unwrap();
    }

    fn prev_day(&mut self) {
        self.date_cursor = self.date_cursor.previous_day().unwrap();
    }

    fn render_current_month(&self,area: Rect, buf: &mut Buffer) {
        let date = self.date_cursor;
        
        let mut event_store = CalendarEventStore::today(Style::default().red().bold());
        event_store.add(date, Style::default().red().bold().on_light_yellow());
        let monthly = Monthly::new(
            date,
            event_store,
        )
        .block(Block::new().borders(Borders::ALL))
        .show_month_header(Modifier::BOLD)
        .show_weekdays_header(Modifier::ITALIC);
        monthly.render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        //let horizontal = Layout::horizontal([Constraint::Percentage(50); 2]).spacing(1);
        let [top, main] = area.layout(&vertical);
        //let [left, right] = main.layout(&horizontal);
        let title = Line::from_iter([
            Span::from("Calendar Widget").bold(),
            Span::from(" (Press 'q' to quit)"),
        ]);

        title.centered().render(top, buf);

        self.render_current_month(main, buf);
    }
}
