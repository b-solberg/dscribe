use color_eyre::Result;
use crossterm::event;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{Block, Padding};
use ratatui::Frame;
use time::{Date, Month, OffsetDateTime};


pub fn enter_tui() -> Result<()> {
    ratatui::run(|terminal| loop {
        terminal.draw(render)?;
        if event::read()?.is_key_press() {
            break Ok(());
        }
    })
}

fn render(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(50); 2]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [left, right] = main.layout(&horizontal);

    let title = Line::from_iter([
        Span::from("Calendar Widget").bold(),
        Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_current_month(frame, left);
    render_styled_month(frame, right);
}

/// Render the current month calendar.
fn render_current_month(frame: &mut Frame, area: Rect) {
    let date = OffsetDateTime::now_utc().date();

    let monthly = Monthly::new(
        date,
        CalendarEventStore::today(Style::default().red().bold()),
    )
    .block(Block::new().padding(Padding::new(0, 0, 2, 0)))
    .show_month_header(Modifier::BOLD)
    .show_weekdays_header(Modifier::ITALIC);
    frame.render_widget(monthly, area);
}

/// Render an arbitrary month with more styles.
fn render_styled_month(frame: &mut Frame, area: Rect) {
    // Release date of the movie Ratatouille.
    let date = Date::from_calendar_date(2007, Month::June, 29).unwrap();

    let mut event_store = CalendarEventStore::today(Style::default().red().bold());
    event_store.add(date, Style::default().blue().italic());

    let monthly = Monthly::new(date, event_store)
        .show_surrounding(Modifier::DIM)
        .show_month_header(Modifier::BOLD)
        .show_weekdays_header(Style::default().bold().green())
        .default_style(Style::default().bold().bg(Color::Rgb(50, 50, 50)));
    frame.render_widget(monthly, area);
}
