use chrono::{DateTime, Local};
use std::collections::HashMap;
use rand::Rng;


#[derive(Debug, PartialEq)]
enum TodoState {
    Active,
    Complete,
}
#[derive(Debug, PartialEq)]
struct Todo {
    title: String,
    state: TodoState,
    date_created: DateTime<Local>,
    date_finished: Option<DateTime<Local>>,
}

impl Todo {
    fn new(title: &str) -> Todo {
        Todo {
            title: String::from(title),
            state: TodoState::Active,
            date_created: Local::now(),
            date_finished: None,
        }
    }

    fn update_state(&mut self, state: TodoState) -> () {
        self.state = state;
    }
}
#[derive(PartialEq, Debug)]
enum ListFilter {
    Active,
    Closed,
    All,
}

struct TodoList {
    todos: HashMap<usize, Todo>,
    filter: ListFilter,
}
impl TodoList {
    fn new() -> TodoList {
       TodoList{
        todos: HashMap::new(),
        filter: ListFilter::All,
       }
    }

    fn add(&mut self, title: &str) -> () {
        let mut rng = rand::thread_rng();
        let id: usize = rng.gen();
        let todo: Todo = Todo::new(title);
        self.todos.insert(
            id,
            todo
        );
    }

    fn mark_complete(&mut self, index: usize) -> () {
        let todo = self.todos.get_mut(&index).unwrap();
        todo.date_finished = Some(Local::now());
        todo.state = TodoState::Complete;
    }

    fn mark_active(&mut self, index: usize) -> () {
        let todo = self.todos.get_mut(&index).unwrap();
        todo.date_finished = None;
        todo.state = TodoState::Active;
    }

fn apply_filter(&mut self, filter_type: ListFilter) -> HashMap<&usize, &Todo> {
        match filter_type {
            ListFilter::Active => {
                let todos: HashMap<&usize, &Todo> = self.todos.iter().filter(|(id, item)| {item.state == TodoState::Active}).collect();
                return todos
            }

            ListFilter::Closed => {
                let todos: HashMap<&usize, &Todo> = self.todos.iter().filter(|(id, item)| {item.state == TodoState::Active}).collect();
                return todos;
            }

            ListFilter::All => {
                return self.todos.iter().collect();
            }
        }
    }

    fn get_todos(&self) -> &HashMap<usize, Todo> {
        let todos = &self.todos;
        return todos;
    }
}