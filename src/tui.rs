use std::io;
use time::format_description;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{Block};
use ratatui::widgets::Widget;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::widgets::Borders;

use time::{Date, OffsetDateTime};

pub fn select_date() -> io::Result<Option<Vec<Date>>> {
       ratatui::run(|terminal| App::default().run(terminal))
} 

pub fn convert_dates_to_string(dates: &Vec<Date>, prefix: &str) -> String {
    let format = format_description::parse_borrowed::<3>("[month]-[day]-[year]").unwrap();
    dates.iter().map(|date| format!("{}{}",prefix,date.format(&format).unwrap())).collect::<Vec<String>>().join(",")
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    date_cursor: Date,
    selected_date: Option<Vec<Date>>
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            date_cursor: OffsetDateTime::now_local().expect("Local time not found").date(),
            selected_date:None,
        }
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<Option<Vec<Date>>> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(self.selected_date.clone())
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
            KeyCode::Char('q') => {self.selected_date = None; self.exit()},
            KeyCode::Esc => self.exit(),
            KeyCode::Tab => self.next_day(),
            KeyCode::Char('l') => self.next_day(),
            KeyCode::Right => self.next_day(),
            KeyCode::Backspace=> self.remove_date(),
            KeyCode::Left => self.prev_day(),
            KeyCode::Char('h') => self.prev_day(),
            KeyCode::Down=> for _ in 0..7 {self.next_day()},
            KeyCode::Char('j') => for _ in 0..7 {self.next_day()},
            KeyCode::Up=> for _ in 0..7 {self.prev_day()},
            KeyCode::Char('k') => for _ in 0..7 {self.prev_day()},
            KeyCode::Char(' ') => self.add_date(),
            KeyCode::Enter => {self.exit()},
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

    fn add_date(&mut self) {
        match &mut self.selected_date {
            Some(dates) => dates.push(self.date_cursor),
            None => self.selected_date = Some(vec![self.date_cursor]),
        }    
    }

    fn remove_date(&mut self) {
        if let Some(dates) = &mut self.selected_date {
            dates.pop();
        }
    }

    fn display_dates(&self) -> String {
        match &self.selected_date {
            Some(dates) => convert_dates_to_string(dates, ""),
            None => "".to_string()
        }
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
        let first_vertical = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).spacing(1);
        let second_vertical = Layout::vertical([Constraint::Percentage(50); 2]).spacing(1);
        let [top, main] = area.layout(&first_vertical);
        let [top_top, second_top] = top.layout(&second_vertical);
        let title = Line::from_iter([
            Span::from("Calendar Widget").bold(),
            Span::from(" ('q' = quit, 'Enter' = confirm selection, 'Spacebar' = select dates, 'Backspace' = remove dates, move with arrow keys or hjkl)"),
        ]);
        let state = Line::from_iter([
            Span::from("Selected Dates : ").bold(),
            Span::from(self.display_dates()),
        ]);

        title.centered().render(top_top, buf);
        state.centered().render(second_top, buf);

        self.render_current_month(main, buf);
    }
}


