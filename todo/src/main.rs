mod structs;

use crate::structs::Todo;
use crate::structs::TodoList;
use std::io;

fn main() {
    let l = [
        Todo::from_title(String::from("Clean")),
        Todo::from_title(String::from("Laundry")),
        Todo::from_title(String::from("Cook")),
    ];

    let mut tdl = TodoList::from_iter(l);

    loop {
        let tdlc = tdl.clone();
        for (i, e) in tdlc.into_iter().enumerate() {
            println!("{i} {}", e.to_string());
        }

        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line.");

        let (action, n) = match inp.find(' ') {
            Some(e) => (String::from(&inp[..e]), String::from(&inp[e + 1..])),
            None => (String::from(inp.trim()), String::from("")),
        };

        let n: Option<usize> = match n.trim().parse() {
            Ok(e) => Some(e),
            Err(_) => None,
        };

        match (action.as_str(), n) {
            ("c", Some(e)) => tdl.complete(e),
            ("d", Some(e)) => {
                println!("Enter yyyy-mm-dd: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl.set_due_iso8601(e, inp)
            }
            ("r", Some(e)) => {
                println!("Enter duration: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl.set_repeat(e, inp)
            }
            ("t", Some(e)) => println!("{} {}", "title", e),
            ("rt", Some(e)) => {
                println!("d: from due c: from completed");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                match inp.as_str() {
                    "c" => tdl.set_repeat_type(e, true),
                    "d" => tdl.set_repeat_type(e, false),
                    _ => continue,
                }
            }
            ("q", _) => break,
            (_, _) => continue,
        }
    }
}
