use crate::config::{load_config, Config};
use crate::projects::{initialize_projects, Project};
use crate::term;
use std::{io::Error, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    terminal::Terminal,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default, PartialEq)]
pub enum Mode {
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
    pub selected_project: u8,
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
            selected_project: 0,
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
            frame.render_widget(self, frame.size());
        })?;
        Ok(())
    }

    fn handle_keypress(&mut self) -> Result<(), Error> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        match term::next_event(timeout)? {
            Some(Event::Key(key)) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => {}
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]);
        let [title, body] = vertical.areas(area);

        let title_block = Block::bordered();
        let title_text = Paragraph::new("Things To Do").block(title_block);
        title_text.render(title, buf);

        self.render_project(body, buf);
    }
}

impl App {
    fn render_project(&self, area: Rect, buf: &mut Buffer) {
        let project = self.projects[self.selected_project as usize].clone();
        project.render(area, buf);
    }
}
