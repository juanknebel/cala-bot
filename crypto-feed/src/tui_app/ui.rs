use tui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Style},
  text::{Span, Spans},
  widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Wrap},
  Frame,
};

use super::{actions::Actions, state::AppState};
use crate::tui_app::app::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
  B: Backend, {
  let size = rect.size();
  check_size(&size);

  // Vertical layout
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
    .split(size);

  // Title
  let title = draw_title();
  rect.render_widget(title, chunks[0]);

  // Body & Commands
  let body_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
      [
        Constraint::Percentage(50),
        Constraint::Percentage(30),
        Constraint::Percentage(20),
      ]
      .as_ref(),
    )
    .split(chunks[1]);

  let left_body = draw_stream(false, app.state());
  rect.render_widget(left_body, body_chunks[0]);

  let right_body = draw_api(app.request_content(), app.line_separator());
  rect.render_widget(right_body, body_chunks[1]);

  let help = draw_commands(app.actions());
  rect.render_widget(help, body_chunks[2]);
}

fn draw_api<'a>(content: String, line_separator: String) -> Paragraph<'a> {
  Paragraph::new(vec![Spans::from(Span::raw(content))])
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Left)
    .block(
      Block::default()
        .title("Api responses")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain),
    )
    .wrap(Wrap {
      trim: true,
    })
}

fn draw_title<'a>() -> Paragraph<'a> {
  Paragraph::new("Cala Bot Trading TUI")
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Center)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain),
    )
}

fn check_size(rect: &Rect) {
  if rect.width < 52 {
    panic!("Require width >= 52, (got {})", rect.width);
  }
  if rect.height < 28 {
    panic!("Require height >= 28, (got {})", rect.height);
  }
}

fn draw_stream<'a>(loading: bool, state: &AppState) -> Paragraph<'a> {
  // let loading_text = if loading { "Loading..." } else { "" };
  // let tick_text = if let Some(ticks) = state.count_tick() {
  // format!("Tick count: {}", ticks)
  // } else {
  // String::default()
  // };
  let title = format!("Live stream for: ");
  Paragraph::new(vec![
    // Spans::from(Span::raw(loading_text)),
    // Spans::from(Span::raw(tick_text)),
    Spans::from(Span::raw(String::default())),
  ])
  .style(Style::default().fg(Color::LightCyan))
  .alignment(Alignment::Left)
  .block(
    Block::default()
      .title(title)
      .borders(Borders::ALL)
      .style(Style::default().fg(Color::White))
      .border_type(BorderType::Plain),
  )
}

fn draw_commands(actions: &Actions) -> Table {
  let key_style = Style::default().fg(Color::LightCyan);
  let help_style = Style::default().fg(Color::Gray);

  let mut rows = vec![];
  for action in actions.actions().iter() {
    let mut first = true;
    for key in action.keys() {
      let help = if first {
        first = false;
        action.to_string()
      } else {
        String::from("")
      };
      let row = Row::new(vec![
        Cell::from(Span::styled(key.to_string(), key_style)),
        Cell::from(Span::styled(help, help_style)),
      ]);
      rows.push(row);
    }
  }

  Table::new(rows)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title("Commands"),
    )
    .widths(&[Constraint::Length(11), Constraint::Min(20)])
    .column_spacing(1)
}