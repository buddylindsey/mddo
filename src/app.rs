use crate::{
    config::{load_config, Config},
    projects::{initialize_projects, Project},
    term,
};

use std::{io::Error, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    terminal::Terminal,
    widgets::{Block, Paragraph, Tabs, Widget},
};

#[derive(Debug, Default, PartialEq)]
pub enum Mode {
    #[default]
    MainScreen,
    AddProject,
    EditProject,
    EditTodo,
    Quit,
    Popup,
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
            KeyCode::Char('p') => {
                if self.mode == Mode::MainScreen {
                    self.mode = Mode::Popup;
                } else {
                    self.mode = Mode::MainScreen;
                }
            }
            KeyCode::Char('c') => self.mode = Mode::MainScreen,
            KeyCode::Esc => self.mode = Mode::MainScreen,
            KeyCode::Tab => {
                self.selected_project = (self.selected_project + 1) % self.projects.len() as u8
            }
            _ => {}
        };
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]);
        let [tabs, body] = vertical.areas(area);

        self.render_tabs(tabs, buf);
        self.render_project(body, buf);

        if self.mode == Mode::Popup {
            self.render_popup(area, buf);
        }
    }
}

fn calculate_popup_area(area: Rect, height: u16, width: u16) -> Rect {
    let popup_x = (area.width.saturating_sub(width)) / 2;
    let popup_y = (area.height.saturating_sub(height)) / 2;

    return Rect::new(popup_x, popup_y, width, height);
}

impl App {
    fn render_popup(&self, area: Rect, buf: &mut Buffer) {
        let popup_area = calculate_popup_area(area, 10, 30);

        // Create a block for the popup with a border
        let block = Block::default().borders(ratatui::widgets::Borders::ALL);

        // Create a Paragraph for the popup content
        let text = Paragraph::new("I am a popup").block(block);

        // Render the popup
        text.render(popup_area, buf);
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        if self.projects.is_empty() {
            return;
        }
        let tab_titles: Vec<&str> = self.projects.iter().map(|tab| tab.title.as_str()).collect();

        let tabs_block = Block::bordered().title("Projects");
        let tabs = Tabs::new(tab_titles)
            .select(self.selected_project as usize)
            .block(tabs_block);

        tabs.render(area, buf);
    }

    fn render_project(&self, area: Rect, buf: &mut Buffer) {
        if self.projects.is_empty() {
            return;
        }
        let project = self.projects[self.selected_project as usize].clone();
        project.render(area, buf);
    }
}
