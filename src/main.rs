use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Key not provided.");
    let value = args.next().expect("Value not provided");
    println!("K: {}, V: {}", key, value);

    let mut db = Database::new().expect("DB init failed");
    db.insert(key, value);
    db.flush().unwrap();
}


struct Database {
    inner: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let contents = std::fs::read_to_string("kvs.db")?;
        let mut inner = HashMap::new();
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            inner.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            inner,
            flush: false,
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    // the last method that a database can execute
    // prevent writes after flush
    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(db: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &db.inner {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kvs.db", contents)
}
