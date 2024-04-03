use serde_json;
use std::collections::HashMap;
use std::env;
// use std::io::Read;
// use std::str::FromStr;

fn main() {
    let action = env::args().nth(1).expect("Please specify an action");

    let mut item = String::new();

    if action == "show" {
        item.push_str("");
    } else {
        item = env::args().nth(2).expect("Please specify an item");
    }

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);

        match todo.save() {
            Ok(_) => println!("Todo has been saved."),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo has been saved."),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "show" {
        todo.show();
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    // fn new() -> Result<Todo, std::io::Error> {
    //     let mut file = std::fs::OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .read(true)
    //         .open("db.txt")?;

    //     let mut content = String::new();
    //     file.read_to_string(&mut content)?;
    //     let map: HashMap<String, bool> = content
    //         .lines()
    //         .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
    //         .map(|v| (v[0], v[1]))
    //         .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
    //         .collect();

    //     Ok(Todo { map })
    // }

    fn new() -> Result<Todo, std::io::Error> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        // serialise json into HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        // insert a new item into the map
        // default value is true
        self.map.insert(key, true);
    }

    fn show(&self) {
        for (key, value) in self.map.iter() {
            println!("Task: {}\tProgress: {}", key, value);
        }
    }

    // save takes ownership of self, so this enforces
    // us to use save as the last method of a session.
    // fn save(self) -> Result<(), std::io::Error> {
    //     let mut content = String::new();
    //     for (k, v) in self.map {
    //         let record = format!("{}\t{}\n", k, v);
    //         content.push_str(&record);
    //     }

    //     std::fs::write("db.txt", content)
    // }
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // add this flag
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
