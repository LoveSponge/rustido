use crossterm::{
    event::{
        self,
        Event as CEvent,
        KeyCode,
        KeyModifiers
    },
    terminal::{
        disable_raw_mode,
        enable_raw_mode
    },
};
use serde::{
    Deserialize,
    Serialize
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{
    Duration,
    Instant
};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{
        Constraint,
        Direction,
        Layout
    },
    Terminal,
};

pub mod todos;
use todos::{list::TodoList, popup::Popup};

pub mod db;

pub const DB_PATH: &str = "./data/todos.json";

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
//     id: usize,
    title: String,
    description: String
//     age: usize,
//     created_at: DateTime<Utc>,
}

// #[derive(Copy, Clone, Debug)]
// enum MenuItem {
//     Home,
//     Pets,
// }

// impl From<MenuItem> for usize {
//     fn from(input: MenuItem) -> usize {
//         match input {
//             MenuItem::Home => 0,
//             MenuItem::Pets => 1,
//         }
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("events readable") {
                    tx.send(Event::Input(key)).expect("events sendable");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut popup = Popup::new();
    let mut todo_list = TodoList::init();

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ]
                    .as_ref()
                )
                .split(size);

            let todos_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(80)
                    ].as_ref()
                )
                .split(chunks[1]);

            let (left, right) = todo_list.render();
            rect.render_stateful_widget(left, todos_chunks[0], &mut todo_list.state);
            rect.render_widget(right, todos_chunks[1]);

            if let Some((popup_block, popup_rect)) = popup.render(size) {
                rect.render_widget(popup_block, popup_rect);
            }
        })?;

        match rx.recv()? {
            Event::Input(event) => match (event.modifiers, event.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c'))
                | (KeyModifiers::NONE, KeyCode::Char('q')) => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    break;
                }
                (KeyModifiers::NONE, KeyCode::Char('a')) => {
                    popup.show_popup = true;
                    todo_list.add();
                }
                (KeyModifiers::NONE, KeyCode::Char('d')) => {
                    todo_list.remove(todo_list.state.selected().unwrap());
                }
                (KeyModifiers::NONE,  KeyCode::Down)
                | (KeyModifiers::NONE, KeyCode::Char('j')) => {
                    todo_list.select_next_todo();
                }
                (KeyModifiers::NONE, KeyCode::Up)
                | (KeyModifiers::NONE, KeyCode::Char('k')) => {
                    todo_list.select_prev_todo();
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

