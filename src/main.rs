use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::Alignment,
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Terminal},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

// App state
struct App {
    counter: i64,
    should_quit: bool,
}

// App ui render function
fn ui(app: &App, f: &mut Frame<'_>) {
    f.render_widget(
        Paragraph::new(format!(
            "
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
            Press `j` and `k` to increment and decrement the counter respectively.\n\
            Counter: {}
          ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        f.size(),
    )
}

// App update function
fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('j') => app.counter += 1,
                    Char('k') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    // ratatui terminal
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // application state
    let mut app = App {
        counter: 0,
        should_quit: false,
    };

    loop {
        // application update
        update(&mut app)?;

        // application render
        t.draw(|f| {
            ui(&app, f);
        })?;

        // application exit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
fn main() -> Result<()> {
    color_eyre::install()?;

    startup()?;

    let result = run();

    // teardown terminal before unwrapping Result of app run
    shutdown()?;

    result?;

    Ok(())
}
