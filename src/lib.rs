wai_bindgen_rust::import!("fib.wai");
wai_bindgen_rust::import!("io.wai");
wai_bindgen_rust::export!("hello-world-rust-wasmer-lib.wai");

use crate::hello_world_rust_wasmer_lib::*;
use std::sync::Mutex;
use wai_bindgen_rust::Handle;

pub struct HelloWorldRustWasmerLib;
impl hello_world_rust_wasmer_lib::HelloWorldRustWasmerLib for HelloWorldRustWasmerLib {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    fn greet(name: String) -> String {
        format!("Hello {name}!")
    }

    fn greet_many(people: Vec<String>) -> String {
        match people.as_slice() {
            [] => "Hello!".to_string(),
            [person] => HelloWorldRustWasmerLib::greet(person.into()),
            [person_1, person_2] => format!("Hello {person_1} and {person_2}!"),
            [people @ .., last] => {
                let people = people.join(", ");
                format!("Hello {people}, and {last}!")
            }
        }
    }

    fn distance_between(p1: Point, p2: Point) -> f32 {
        let Point { x: x1, y: y1 } = p1;
        let Point { x: x2, y: y2 } = p2;

        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    fn perimeter_of_circle(c: Circle) -> f32 {
        let Circle { center: _, radius } = c;
        (2.0 * 22.0 * radius as f32) / 7.0
    }
    fn area_of_circle(c: Circle) -> f32 {
        let Circle { center: _, radius } = c;
        (22.0 * (radius * radius) as f32) / 7.0
    }
    fn multi_line_length(l: MultiLine) -> f32 {
        if l.points.len() == 0 {
            return 0.0;
        }
        let mut result = 0.0;
        for i in 1..l.points.len() {
            let p1 = l.points[i - 1];
            let p2 = l.points[i];
            result += HelloWorldRustWasmerLib::distance_between(p1, p2);
        }
        result
    }

    fn fib_n_plus_one(n: u32) -> u32 {
        return fib::fib(n + 1);
    }

    fn cowsay(message: String) {
        let cow = r#"
      ^__^
     (oo)\_______
     (__)\       )\/\
         ||----w |
         ||     ||
    "#;

        // Create the speech bubble
        let speech_bubble = format!(
            "
      {}
    < {} >
      {}
    ",
            "-".repeat(message.len() + 2),
            message,
            "-".repeat(message.len() + 2)
        );

        // Combine the cow and speech bubble
        io::print(&format!("{}{}", cow, speech_bubble));
    }
}

pub struct Calculator(Mutex<f32>);
impl hello_world_rust_wasmer_lib::Calculator for Calculator {
    fn new(initial_value: f32) -> Handle<Calculator> {
        Handle::new(Calculator(Mutex::new(initial_value)))
    }

    fn current_value(&self) -> f32 {
        *self.0.lock().unwrap()
    }

    fn add(&self, value: f32) {
        *self.0.lock().unwrap() += value;
    }

    fn multiply(&self, value: f32) {
        *self.0.lock().unwrap() *= value;
    }
}
