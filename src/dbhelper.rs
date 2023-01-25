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
        WHERE L1_Word=?
        AND L2_Word=?").unwrap();
    let word_1: &str = &vocab.get_lang_1()[..];
    let word_2: &str = &vocab.get_lang_2()[..];
    let friendly_1: &str = &vocab.get_friendly_1()[..];
    let friendly_2: &str = &vocab.get_friendly_2()[..];
    stmt.bind((1,word_1)).unwrap();
    stmt.bind((2,word_2)).unwrap();
    stmt.next().unwrap();
    let count: i64 = stmt.read::<i64,_>("Count").unwrap();
    if count==0 {
        let mut stmt_insert = conn.prepare("INSERT INTO Vocabulary (L1_Word, L2_Word, L1_Friendly, L2_Friendly) VALUES(?,?,?,?);").unwrap();
        stmt_insert.bind((1,word_1)).unwrap();
        stmt_insert.bind((2,word_2)).unwrap();
        stmt_insert.bind((3,friendly_1)).unwrap();
        stmt_insert.bind((4,friendly_2)).unwrap();
        stmt_insert.next().unwrap();
    }
}

fn create_db_if_not_exists() {
    let conn = sqlite::open(FILE).unwrap();
    let mut query: String = String::from("SELECT * FROM Vocabulary;");
    match conn.execute(query) {
        Ok(x) => return,
        Err(m) => println!("No database found. Creating database Vocabulary")
    };
    let query: String = String::from("
        CREATE TABLE Vocabulary (
        ID INTEGER PRIMARY KEY AUTOINCREMENT, LangID INT,
        L1_Word VARCHAR(30) NOT NULL,
        L2_Word VARCHAR(30) NOT NULL,
        L1_Friendly VARCHAR(50) NOT NULL,
        L2_Friendly VARCHAR(50) NOT NULL,
        Drawer INTEGER DEFAULT 0
            );");
   conn.execute(query).unwrap();
   let query: String = String::from("CREATE TABLE Langs (LangID INTEGER PRIMARY KEY, L1_Name VARCHAR(20), L2_Name VARCHAR(20));");
   conn.execute(query).unwrap();
   let query: String = String::from("INSERT INTO Langs (LangID,L1_Name,L2_Name) VALUES(1,'Altisländisch','Deutsch');");
   conn.execute(query).unwrap();

}

pub fn read_vocabs() {
        create_db_if_not_exists();
        let file_path:String = String::from("/home/stefan/Rust/voctrainer/src/vocabs.txt");
        let contents = fs::read_to_string(file_path).expect("");
        let split = contents.trim().split("\n");
        let lines:Vec<&str> = split.collect();
        let mut counter:i32 = 0;

        for s in lines {
            if counter==0 {
            counter+=1;
            continue;
            }
            let words:Vec<&str> = s.split(";").collect();
            let vocab: Vocab = Vocab::new(0,String::from(words[0]),String::from(words[1]),String::from(words[2]),String::from(words[3]));
            add_entry(&vocab);
        }
}


pub fn random_from_lowest_drawer() -> Vocab {
    let conn = sqlite::open(FILE).unwrap();
    let mut query = conn.prepare("
    SELECT a.ID, a.L1_Word, a.L2_Word, a.L1_Friendly, a.L2_Friendly FROM
        (SELECT ID, L1_Word, L2_Word, L1_Friendly, L2_Friendly FROM Vocabulary 
            WHERE Drawer = (SELECT MIN(Drawer) FROM Vocabulary)) a ORDER BY RANDOM() LIMIT 1;
        ").unwrap();
    query.next().unwrap();
    let id: i64 = query.read::<i64,_>(0).unwrap();
    let l1_w: String = query.read::<String,_>(1).unwrap();
    let l2_w: String = query.read::<String,_>(2).unwrap(); 
    let l1_f: String = query.read::<String,_>(3).unwrap(); 
    let l2_f: String = query.read::<String,_>(4).unwrap(); 
    return Vocab::new(id,l1_w,l2_w,l1_f,l2_f);
}

pub fn into_next_drawer(id:i64) {
    let conn = sqlite::open(FILE).unwrap();
    let mut query = conn.prepare("
        UPDATE Vocabulary SET Drawer=Drawer+1
        WHERE ID=?;
        ").unwrap();
    query.bind((1,id)).unwrap();
    query.next().unwrap();
}
