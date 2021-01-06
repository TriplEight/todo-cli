use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        // open the db file
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        // read its content into a new string   
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        // convert from the String type of the file to a HashMap
        let map: HashMap<String, bool> = content
            // loop over each line of the file
            .lines()
            // split our lines on the tab character and
            // map our Split string into Vec<&str>
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            // transform it into a tuple
            .map(|v| (v[0], v[1]))
            // convert the two elements of the tuple into a String and a bool
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        // Return Ok
        Ok(Todo{ map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k,v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action\n");
    let item = std::env::args().nth(2).expect("Please specify an item\n");
    
    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("DB init has failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occured: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not in the list, item.", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occureed: {}", why),
            },
        }
    }
}
