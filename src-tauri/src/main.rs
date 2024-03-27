use std::fs::{read_to_string, write};
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use tauri::{Manager, State};

struct NotesState {
    notes: Mutex<Vec<String>>,
}

impl Default for NotesState {
    fn default() -> Self {
        Self {
            notes: Mutex::new(Vec::new()),
        }
    }
}

#[tauri::command]
fn load_notes(app_data_path: PathBuf, state: State<'_, NotesState>) -> Vec<String> {
    let notes_file = app_data_path.join("notes.txt");
    if let Ok(content) = read_to_string(&notes_file) {
        let notes: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut state_notes = state.notes.lock().unwrap();
        *state_notes = notes.clone();
        notes
    } else {
        vec![]
    }
}

#[tauri::command]
fn add_note(app_data_path: PathBuf, note: String, state: State<'_, NotesState>) {
    let mut notes = state.notes.lock().unwrap();
    notes.push(note);
    let notes_file = app_data_path.join("notes.txt");
    let content = notes.join("\n");
    write(&notes_file, content).unwrap();
}

#[tauri::command]
fn delete_note(app_data_path: PathBuf, index: usize, state: State<'_, NotesState>) {
    let mut notes = state.notes.lock().unwrap();
    if index < notes.len() {
        notes.remove(index);
        let notes_file = app_data_path.join("notes.txt");
        let content = notes.join("\n");
        write(&notes_file, content).unwrap();
    }
}

fn alpha() {
  let _immutable_binding = 1;
  let mut mutable_binding = 1;

  println!("Before mutation: {}", mutable_binding);

  // Ok
  mutable_binding += 1;

  println!("After mutation: {}", mutable_binding);

  // Error! Cannot assign a new value to an immutable variable
  _immutable_binding += 1;
}

fn is_odd(n: u32) -> bool {
  n % 2 == 1
}

fn beta() {
  println!("Find the sum of all the numbers with odd squares under 1000");
  let upper = 1000;

  // Imperative approach
  // Declare accumulator variable
  let mut acc = 0;
  // Iterate: 0, 1, 2, ... to infinity
  for n in 0.. {
      // Square the number
      let n_squared = n * n;

      if n_squared >= upper {
          // Break loop if exceeded the upper limit
          break;
      } else if is_odd(n_squared) {
          // Accumulate value, if it's odd
          acc += n_squared;
      }
  }
  println!("imperative style: {}", acc);

  // Functional approach
  let sum_of_squared_odd_numbers: u32 =
      (0..).map(|n| n * n)                             // All natural numbers squared
           .take_while(|&n_squared| n_squared < upper) // Below upper limit
           .filter(|&n_squared| is_odd(n_squared))     // That are odd
           .sum();                                     // Sum them
  println!("functional style: {}", sum_of_squared_odd_numbers);
}

struct Point {
  x: f64,
  y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
impl Point {
  // This is an "associated function" because this function is associated with
  // a particular type, that is, Point.
  //
  // Associated functions don't need to be called with an instance.
  // These functions are generally used like constructors.
  fn origin() -> Point {
      Point { x: 0.0, y: 0.0 }
  }

  // Another associated function, taking two arguments:
  fn new(x: f64, y: f64) -> Point {
      Point { x: x, y: y }
  }
}

struct Rectangle {
  p1: Point,
  p2: Point,
}

impl Rectangle {
  // This is a method
  // `&self` is sugar for `self: &Self`, where `Self` is the type of the
  // caller object. In this case `Self` = `Rectangle`
  fn area(&self) -> f64 {
      // `self` gives access to the struct fields via the dot operator
      let Point { x: x1, y: y1 } = self.p1;
      let Point { x: x2, y: y2 } = self.p2;

      // `abs` is a `f64` method that returns the absolute value of the
      // caller
      ((x1 - x2) * (y1 - y2)).abs()
  }

  fn perimeter(&self) -> f64 {
      let Point { x: x1, y: y1 } = self.p1;
      let Point { x: x2, y: y2 } = self.p2;

      2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
  }

  // This method requires the caller object to be mutable
  // `&mut self` desugars to `self: &mut Self`
  fn translate(&mut self, x: f64, y: f64) {
      self.p1.x += x;
      self.p2.x += x;

      self.p1.y += y;
      self.p2.y += y;
  }
}

fn main() {
    tauri::Builder::default()
        .manage(NotesState::default())
        .invoke_handler(tauri::generate_handler![
            load_notes,
            add_note,
            delete_note
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}