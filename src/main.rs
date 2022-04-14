use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    let mut database = Database::new().expect("Creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    match database.flush() {
        Ok(()) => println!("YAY"),
        Err(err) => println!("OH Errr {}", err),
    }
}

struct Database {
    map: HashMap<String, String>,
    flushed: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("no key found");
            let value = chunks.next().expect("no value found");
            map.insert(key.to_owned(), value.to_owned());
        }

        // parse the string
        // populate our map
        Ok(Database {
            map,
            flushed: false,
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flushed = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flushed {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();

    for (key, value) in &database.map {
        let kvpair = format!("{}\t{}\n", key, value);
        contents.push_str(&kvpair);
    }

    std::fs::write("kv.db", contents)
}
