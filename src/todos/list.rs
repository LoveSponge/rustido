use tui::{
    style::{
        Color,
        Modifier,
        Style
    },
    text::{Span, Spans},
    widgets::{
        Block,
        BorderType,
        Borders,
        List,
        ListItem,
        ListState,
        Paragraph,
    },
};
use crate::Todo;
use crate::db::*;

pub struct TodoList {
    pub todos: Vec<Todo>,
    pub state: ListState
}

impl TodoList {
    pub fn init() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            todos: read_db().expect("read todos from file"),
            state: list_state
        }
    }

    fn x(&mut self) {
        self.todos = read_db().expect("read todos from file")
    }

    pub fn render<'a>(&mut self) -> (List<'a>, Paragraph<'a>) {
        let todo_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Todo")
            .border_type(BorderType::Plain);

        self.x();

        let items: Vec<_> = self.todos
            .iter()
            .map(|todo| {
                ListItem::new(Spans::from(vec![Span::styled(
                    todo.title.clone(),
                    Style::default(),
                )]))
            })
            .collect();

        let selected_todo = match self.get_selected_item() {
            None => Todo { title: String::new(), description: format!("No todo selected") },
            Some(todo) => todo.clone()
        };

        let list = List::new(items).block(todo_block).highlight_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

        let description = Paragraph::new(selected_todo.description).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray))
                .title("Description")
                .border_type(BorderType::Plain));

        (list, description)
    }

    fn select_todo_at_index(&mut self, index: usize) {
        self.state.select(Some(index));
    }
    pub fn select_prev_todo(&mut self) {
        let todos_length = self.todos.len();

        if todos_length == 0 { return }

        if let Some(current_index) = self.state.selected() {
            if current_index == 0
            {
                self.select_todo_at_index(todos_length -1);
            }
            else {
                self.select_todo_at_index(current_index -1);
            }
        }
    }
    pub fn select_next_todo(&mut self) {
        let todos_length = self.todos.len();
        if todos_length == 0 { return }

        if let Some(current_index) = self.state.selected() {
            let todos_length = self.todos.len();
            if current_index == todos_length -1
            {
                self.select_todo_at_index(0);
            }
            else {
                self.select_todo_at_index(current_index +1);
            }
        }
    }

    pub fn add(&self) -> () {
        let mut parsed = read_db().expect("db read successfully");
        let tasks_length = &parsed.len() + 1;
        let new_todo = Todo {
            title: format!("Task {}", tasks_length),
            description: format!("description {}", tasks_length)
        };
        parsed.push(new_todo);
        write_db(parsed).expect("Successfully added todo");
    }

    pub fn remove(&mut self, todo_index: usize) -> () {
        let todos_length = self.todos.len();
        if todos_length == 0 { return }

        if let Some(selected) = self.state.selected() {
            let mut parsed = read_db().unwrap();
            parsed.remove(todo_index);

            if selected == todos_length - 1 {
                self.select_prev_todo();
            }

            write_db(parsed).expect("Successfully deleted todo");
        }
    }

    pub fn get_selected_item(&self) -> Option<&Todo> {
        self.todos.get(self.state
            .selected()
            .expect("there is always a selected todo"))
    }
}
