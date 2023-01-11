use super::voclist::Vocab;
use std::fs;

const FILE:&str = &"vocabs.db";

pub fn create_database() {
    let conn = sqlite::open(FILE).unwrap();

    let query: String = String::from("
        CREATE TABLE Vocabulary (
        ID INTEGER PRIMARY KEY AUTOINCREMENT,
        OldNorse VARCHAR(30) NOT NULL,
        German VARCHAR(30) NOT NULL,
        Drawer INTEGER DEFAULT 0
            );");
   conn.execute(query).unwrap();
}

fn add_entry(vocab:&Vocab) {
    let conn = sqlite::open(FILE).unwrap();
    let mut stmt = conn.prepare("SELECT COUNT(ID) AS Count FROM Vocabulary
        WHERE OldNorse=?
        AND German=?").unwrap();
    let word_1: &str = &vocab.get_word()[..];
    let word_2: &str = &vocab.get_translation()[..];
    stmt.bind((1,word_1)).unwrap();
    stmt.bind((2,word_2)).unwrap();

    stmt.next().unwrap();
    let count: i64 = stmt.read::<i64,_>("Count").unwrap();
    if count==0 {
        let mut stmt_insert = conn.prepare("INSERT INTO Vocabulary (OldNorse, German) VALUES(?,?);").unwrap();
        stmt_insert.bind((1,word_1)).unwrap();
        stmt_insert.bind((2,word_2)).unwrap();
        stmt_insert.next().unwrap();
    }
}


pub fn read_vocabs() {
        let file_path:String = String::from("/home/stefan/Rust/voctrainer/src/vocabs.txt");
        let contents = fs::read_to_string(file_path).expect("");
        let split = contents.trim().split("\n");
        let lines:Vec<&str> = split.collect();
        let mut vlist:Vec<Vocab> = Vec::new();
        let mut counter:i32 = 0;

        for s in lines {
            if counter==0 {
            counter+=1;
            continue;
            }
            let words:Vec<&str> = s.split(";").collect();
            let vocab: Vocab = Vocab::new(String::from(words[0]),String::from(words[1]));
            add_entry(&vocab);
        }
}
