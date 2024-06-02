use crate::config::{load_config, Config};
use crate::projects::{initialize_projects, Project};
use crate::term;
use std::{io::Error, time::Duration};

use ratatui::crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
};

use ratatui::{ backend::Backend,
    terminal::Terminal,
    widgets::Widget,
};

#[derive(Debug, Default, PartialEq)]
enum Mode {
    #[default]
    MainScreen,
    EditProject,
    EditTodo,
    Quit,
}

pub struct App {
    pub projects: Vec<Project>,
    pub config: Config,
    pub mode: Mode,
}

impl Default for App {
    fn default() -> Self {
        let config = load_config().unwrap();
        let projects = initialize_projects(&config.data_location).unwrap();
        let mode = Mode::MainScreen;

        App {
            projects,
            config,
            mode,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<(), Error> {
        while self.is_running() {
            self.draw(terminal)?;
            self.handle_keypress()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<(), Error> {
        terminal.draw(|frame| {
            frame.render_widget(&self, frame.size());
        })?;
        Ok(())
    }

    fn handle_keypress(&mut self) -> Result<(), Error> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        match term::next_event(timeout)? {
            Some(Event::Key(key)) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            None => {}
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.mode = Mode::Quit,
            _ => {}
        };
    }
}
