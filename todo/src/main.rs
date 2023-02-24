mod structs;

use crate::structs::Todo;
use crate::structs::TodoList;
use std::io;

fn main() {
    let mut tdl = TodoList::new();

    loop {
        println!("{}", tdl.to_string());

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
                let mut p = &mut tdl[e];
                for i in path {
                    p = &mut p.get_dependencies()[i];
                }
                p.get_dependencies().add(t);
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
                let mut t = &mut tdl[e];
                for i in path {
                    t = &mut t.get_dependencies()[i]
                }
                t.complete();
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
                let mut p = &mut tdl[e];
                for i in path {
                    p = &mut p.get_sub_tasks()[i];
                }
                p.get_sub_tasks().add(t);
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
                let mut t = &mut tdl[e];
                for i in path {
                    t = &mut t.get_sub_tasks()[i];
                }
                t.complete();
            }
            ("save", _) => match tdl.to_json_file("test.json") {
                Err(e) => print!("{}", e),
                Ok(_) => continue,
            },
            ("load", _) => {
                tdl = TodoList::from_json_file("test.json");
            }
            ("savemd", _) => match tdl.to_markdown_file("test.md") {
                Err(e) => print!("{}", e),
                Ok(_) => continue,
            },
            ("loadmd", _) => {
                tdl = TodoList::from_markdown_file("test.md");
            }
            ("q", _) => break,
            (_, _) => continue,
        }
    }
}
