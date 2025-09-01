use std::{io::{self, Result, Write}, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend, crossterm, layout::Alignment, widgets::{Block, Borders, Paragraph, Wrap}, Terminal
};

pub fn run(
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<()> {
     // 1) Terminali TUI-tilaan
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // 2) Sovellus-tila
    let mut app = App::new();

    // 3) Pääsilmukka
    let result = loop {
        // Piirto
        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .title("Ratatui demo — kirjoita lisätäksesi, Backspace poistaa, Esc poistuu")
                .borders(Borders::ALL);

            let paragraph = Paragraph::new(app.full_text())
                .block(block)
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false });

            f.render_widget(paragraph, size);
        })?;

        // Syöte (pollataan pienen viiveen kanssa, jotta UI pysyy responsiivisena)
        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Esc => break Ok(()),
                    KeyCode::Char(c) => app.push_char(c),
                    KeyCode::Backspace => app.pop_char(),
                    _ => {} // muut näppäimet ignoorataan
                },
                Event::Resize(_, _) => {
                    // Resize = piirretään seuraavalla kierroksella (ei tarvitse erikoiskäsittelyä)
                }
                _ => {}
            }
        }
    };

    // 4) Siivous ja paluu normaaliin tilaan
    cleanup_terminal(&mut terminal)?;

    // Palautetaan silmukan tulos (Ok tai mahdollinen virhe)
    result
}

struct App {
    suffix: String,
}

impl App {
    fn new() -> Self {
        Self { suffix: String::new() }
    }

    fn push_char(&mut self, c: char) {
        self.suffix.push(c);
    }

    fn pop_char(&mut self) {
        self.suffix.pop();
    }

    fn full_text(&self) -> String {
        format!("Hello world{}", self.suffix)
    }
}

/// Varma siivous, kutsutaan aina ennen paluuta
fn cleanup_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}