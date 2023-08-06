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
    Complete,
    All,
}

// Public
struct TodoList {
    todos: Vec<Todo>,
    filter: ListFilter,
}

impl TodoList {
    fn new() -> TodoList {
       TodoList{
        todos: Vec::new(),
        filter: ListFilter::All,
       }
    }

    fn add(&mut self, title: &str) -> () {
        let todo: Todo = Todo::new(title);
        self.todos.push(todo)
    }

    fn mark_complete(&mut self, index: usize) -> () {
        let todo = self.todos.get_mut(index).unwrap();
        todo.date_finished = Some(Local::now());
        todo.state = TodoState::Complete;
    }

    fn mark_active(&mut self, index: usize) -> () {
        let todo = self.todos.get_mut(index).unwrap();
        todo.date_finished = None;
        todo.state = TodoState::Active;
    }

    fn apply_filter(&mut self, filter_type: ListFilter) -> Vec<&Todo> {
        match filter_type {
            ListFilter::Active => {
                let todos: Vec<&Todo> = self.todos.iter().filter(|todo| {todo.state == TodoState::Active}).collect();
                return todos
            }

            ListFilter::Complete => {
                let todos: Vec<&Todo> = self.todos.iter().filter(|todo| {todo.state == TodoState::Complete}).collect();
                return todos
            }

            ListFilter::All => {
                return self.todos.iter().collect();
            }
        }
    }

    fn get_todos(&self) -> &Vec<Todo> {
        let todos = &self.todos;
        return todos;
    }
}

#[cfg(test)]
mod todo_tests {
    use super::*;
    #[test]
    fn it_creates_a_new_todo(){
        let todo = Todo::new("My Todo");
        assert_eq!(todo.title, String::from("My Todo"));
        assert_eq!(todo.date_finished, None);
        assert_eq!(todo.state, TodoState::Active);
    }

    #[test]
    fn it_updates_the_state() {
        let mut todo = Todo::new("My Todo");
        todo.update_state(TodoState::Complete);
        assert_eq!(todo.state, TodoState::Complete);
    }
}

#[cfg(test)]
mod todo_list_tests {
    use super::*;

    #[test]
    fn it_creates_an_empty_list(){
        let list = TodoList::new();
        assert_eq!(list.todos.len(), 0);
        assert_eq!(list.filter, ListFilter::All);
    }

    #[test]
    fn it_adds_a_new_todo(){
        let mut list = TodoList::new();
        list.add("A THING");
        assert_eq!(list.todos.len(), 1);
    }

    #[test]
    fn it_updates_a_todos_to_complete(){
        let mut list = TodoList::new();
        list.add("A THING");
        list.mark_complete(0);

        assert_eq!(list.todos[0].state, TodoState::Complete)
    }

    #[test]
    fn it_updates_a_todos_to_active(){
        let mut list = TodoList::new();
        list.add("A THING");
        list.mark_complete(0);
        list.mark_active(0);

        assert_eq!(list.todos[0].state, TodoState::Active)
    }

    #[test]
    fn it_returns_only_active_todos(){
        let mut list = TodoList::new();
        list.add("A THING");
        list.add("Another THING");
        list.mark_complete(0);
        let new = list.apply_filter(ListFilter::Active);

        assert_eq!(new.len(), 1);
        assert_eq!(new[0].state, TodoState::Active);
        assert_eq!(new[0].title, String::from("Another THING"));
    }

    #[test]
    fn it_returns_only_completed_todos(){
        let mut list = TodoList::new();
        list.add("A THING");
        list.add("Another THING");
        list.mark_complete(0);
        let new = list.apply_filter(ListFilter::Complete);

        assert_eq!(new.len(), 1);
        assert_eq!(new[0].state, TodoState::Complete);
        assert_eq!(new[0].title, String::from("A THING"));
    }
}