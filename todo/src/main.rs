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

    let mut save = String::from("");
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
            ("n", _) => {
                println!("enter new title: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl.add(Todo::from_title(inp));
            }
            ("c", Some(e)) => match tdl[e].complete() {
                Some(e) => tdl.add(e),
                None => (),
            },
            ("cd", Some(e)) => {
                println!("Enter yyyy-mm-dd: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl[e].set_completed_iso8601(inp)
            }
            ("sd", Some(e)) => {
                println!("Enter yyyy-mm-dd: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl[e].set_start_iso8601(inp)
            }
            ("d", Some(e)) => {
                println!("Enter yyyy-mm-dd: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl[e].set_due_iso8601(inp)
            }
            ("r", Some(e)) => {
                println!("Enter duration: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl[e].set_repeat(inp)
            }
            ("t", Some(e)) => {
                println!("enter new title: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl[e].set_title(inp);
            }
            ("dur", Some(e)) => {
                println!("Enter duration: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl[e].set_duration(inp)
            }
            ("tag", Some(e)) => {
                println!("Enter tag: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl[e].add_tag(inp);
            }
            ("utag", Some(e)) => {
                println!("Enter tag: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                inp = String::from(inp.trim());
                tdl[e].remove_tag(inp);
            }
            ("dep", Some(e)) => {
                println!("path: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                let split = inp.trim().split(" ").collect::<Vec<&str>>();
                let path: Vec<usize> = match split[0].len() {
                    0 => Vec::new(),
                    _ => split
                        .into_iter()
                        .map(|x| match x.trim().parse() {
                            Ok(e) => e,
                            Err(_) => panic!("Invalid path"),
                        })
                        .collect(),
                };
                println!("Depends on: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                let n: usize = match inp.trim().parse() {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let t = tdl.remove(n);
                tdl[e].add_dependency(path, t);
            }
            ("cdep", Some(e)) => {
                println!("path: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                let split = inp.trim().split(" ").collect::<Vec<&str>>();
                let path: Vec<usize> = match split[0].len() {
                    0 => Vec::new(),
                    _ => split
                        .into_iter()
                        .map(|x| match x.trim().parse() {
                            Ok(e) => e,
                            Err(_) => panic!("Invalid path"),
                        })
                        .collect(),
                };
                tdl[e].complete_dependency(path);
            }
            ("sub", Some(e)) => {
                println!("path: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                let split = inp.trim().split(" ").collect::<Vec<&str>>();
                let path: Vec<usize> = match split[0].len() {
                    0 => Vec::new(),
                    _ => split
                        .into_iter()
                        .map(|x| match x.trim().parse() {
                            Ok(e) => e,
                            Err(_) => panic!("Invalid path"),
                        })
                        .collect(),
                };
                println!("sub task: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                let n: usize = match inp.trim().parse() {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let t = tdl.remove(n);
                tdl[e].add_sub_task(path, t);
            }
            ("csub", Some(e)) => {
                println!("path: ");
                inp = String::new();
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                let split = inp.trim().split(" ").collect::<Vec<&str>>();
                let path: Vec<usize> = match split[0].len() {
                    0 => Vec::new(),
                    _ => split
                        .into_iter()
                        .map(|x| match x.trim().parse() {
                            Ok(e) => e,
                            Err(_) => panic!("Invalid path"),
                        })
                        .collect(),
                };
                tdl[e].complete_sub_task(path);
            }
            ("save", _) => {
                save = tdl.to_json();
            }
            ("load", _) => {
                tdl = TodoList::from_json(save.as_str());
            }
            ("q", _) => break,
            (_, _) => continue,
        }
    }
}
