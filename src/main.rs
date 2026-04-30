use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Chart, Dataset, GraphType, Borders, Axis, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

use std::time::Duration;
use ratatui::symbols::Marker;
mod sens;
use sens::Sens;

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    ratatui::run(run)?;
    Ok(())
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let sens = Sens::new()?;
    loop {
        terminal.draw(|frame| render(frame, &sens))?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, sens: &Sens) {
    // refactor some point
    let pressure = sens.get_pressure();
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30)])
        .split(area);

    let top = chunks[0];
    let middle = chunks[1];
    let bottom = chunks[2];

    let top_block = Block::default()
        .title("Sense")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let top_inner = top_block.inner(top);
    frame.render_widget(top_block, top);

    let middle_block = Block::default()
        .title("graph weight view ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let middle_inner = middle_block.inner(middle);
    frame.render_widget(middle_block, middle);



   // top
    let weight = sens.calc_weight();
    let text = format!(
        "Device: {}  | Pressure: {} Status: Runing \nFolder: /dev/input/event2
        Weight: {} kg \n
        State: {}
        \n\nPress 'q' to quit.",

        sens.name, pressure, weight, get_status(sens)
    );
    let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, top_inner);

    let bottom_block = Block::default()
        .title("Battery")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let bottom_inner = bottom_block.inner(bottom);
    frame.render_widget(bottom_block, bottom);

    let para_two = format!("Status: running \n State: {}", get_status(sens));


    let footer = Paragraph::new(para_two).wrap(Wrap { trim: true });
    frame.render_widget(footer, bottom_inner);
    render_chart(frame, middle_inner);
}

pub fn render_chart(frame: &mut Frame, area: Rect) {
    let dataset = Dataset::default()
        .name("Stonks")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Color::Blue)
        .data(&[
            (0.0, 1.0),
            (1.0, 3.0),
            (2.0, 0.5),
            (3.0, 2.0),
            (4.0, 0.8),
            (5.0, 4.0),
            (6.0, 1.0),
            (7.0, 6.0),
            (8.0, 3.0),
            (10.0, 10.0),
        ]);

    let x_axis = Axis::default()
        .title("Hustle".blue())
        .bounds([0.0, 10.0])
        .labels(["0%", "50%", "100%"]);

    let y_axis = Axis::default()
        .title("Profit".blue())
        .bounds([0.0, 10.0])
        .labels(["0", "5", "10"]);

    let chart = Chart::new(vec![dataset]).x_axis(x_axis).y_axis(y_axis);
    frame.render_widget(chart, area);
}
fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(125))? {
        let ev = event::read()?;
        if let Event::Key(key) = ev {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn get_status(sens: &Sens) -> &str {
    let str: i32 = sens.get_pressure();
    match str {
        0..=10 => "put finger on haptic pad",
        11..=30 => "place object: try to minimize force",
        31..=9999 => "Okkei",
        _ => "no force exerted",

    }
}
