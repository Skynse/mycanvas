use crate::ui::course;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{BorderType, Borders, Block, Tabs, Widget, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::vec::IntoIter;


use super::course::{Course, get_courses};
#[derive(Copy, Clone, Debug)]
enum Menus {
    Course,
    Assignments,
    Description
}

impl From<Menus> for usize {
    fn from(menu: Menus) -> usize {
        match menu {
            Menus::Course => 0,
            Menus::Assignments => 1,
            Menus::Description => 2,
        }
    }
}
pub struct Window<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub menu_index: usize,
    active_menu: Menus,
    courses: Vec<Course>,
}

impl <'a> Window<'a> {
    pub fn new() -> Window<'a> {
        Window {

            titles: vec![
                "Courses",
                "Assignments",
            ],
            index: 0,
            menu_index: 0,
            active_menu: Menus::Course,
            courses: course::get_courses().unwrap(),
        }
    }


    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
    
    pub fn menu_down(&mut self) {
        self.menu_index = (self.menu_index + 1) % 3;
    }

    pub fn menu_up(&mut self) {
        self.menu_index = (self.menu_index + 1) % 3;
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn render_assignments(&self, assignment_list_statte: &ListState) -> List {
        unimplemented!();
    }
    pub fn render_courses(&self, course_list_state: &ListState) -> List {
        let courses = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Courses")
        .border_type(BorderType::Plain);
        let _courses = &self.courses;
        let course_list_items =  _courses.into_iter().map(|course: &Course| {
            ListItem::new(Spans::from(vec![
                Span::styled(course.name(), Style::default().fg(Color::White)),
            ]))
        }).collect::<Vec<ListItem>>();

        let list = List::new(course_list_items).block(courses).highlight_style(
            Style::default().
            fg(Color::Yellow));

        list
    }

    fn set_active_menu(&mut self, menu: Menus) {
        self.active_menu = menu;
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>, app: &Window) {
        let active_menu = Menus::Course;
        let mut course_list_state = ListState::default();
        course_list_state.select(Some(0));
        let size = f.size();
        let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

        match active_menu {
            Menus::Course => {
                let course_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(50),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[1]);
                let course_list = self.render_courses(&course_list_state);
                f.render_stateful_widget(course_list, course_chunks[0], &mut course_list_state);
            }
            Menus::Assignments => {
                let assignments_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(50),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[1]);
                    // create stub list of assignments
                    let assignments = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Assignments")
                    .border_type(BorderType::Plain);
                    let assignments_list_items =  vec![
                        ListItem::new(Spans::from(vec![
                            Span::styled("Assignment 1", Style::default().fg(Color::White)),
                        ]))
                    ];
                    let list = List::new(assignments_list_items).block(assignments).highlight_style(
                        Style::default().
                        fg(Color::Yellow));
                    f.render_widget(list, assignments_chunks[0]);

            }
            Menus::Description => {
                let description_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(50),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[1]);
            }
        }

    }

   pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
    
    loop {
            terminal.draw(|f| Window::<'a>::ui(&self, f, self))?;
    
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Right => &self.next(),
                    KeyCode::Left => &self.previous(),
                    KeyCode::Char('a') => &self.set_active_menu(Menus::Assignments),
                    KeyCode::Char('c') => &self.set_active_menu(Menus::Course),
                    KeyCode::Char('d') => &self.set_active_menu(Menus::Description),
                    _ => &{}
                };
            }
        }
    }
}