use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Rect, Constraint, Direction};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let rect = terminal.size()?;
    println!("{:?}", rect);
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 10), Constraint::Ratio(10, 20)].as_ref())
            .split(Rect {
                x: 0,
                y: 0,
                width: 10,
                height: 20,
            });
        let width = rect.width.to_string();
        let block = Block::default()
             .title(width.as_str())
             .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default()
             .title("Block 2")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })
}