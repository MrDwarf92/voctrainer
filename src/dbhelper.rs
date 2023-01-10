use super::voclist::Vocab;

const file:&str = &"vocabs.db";

pub fn create_database() {
    let conn = sqlite::open(file).unwrap();

    let query: String = String::from("
        CREATE TABLE Vocabulary (
        ID INTEGER PRIMARY KEY AUTOINCREMENT,
        OldNorse VARCHAR(30) NOT NULL,
        German VARCHAR(30) NOT NULL
            );");
   conn.execute(query).unwrap();
}

pub fn add_entry(vocab:&Vocab) {
    let conn = sqlite::open(file).unwrap();
    let mut stmt = conn.prepare("SELECT COUNT(ID) AS Count FROM Vocabulary
        WHERE OldNorse=?
        AND German=?").unwrap();
    stmt.bind((1,&vocab.get_word()[..])).unwrap();
    stmt.bind((2,&vocab.get_translation()[..])).unwrap();

    stmt.next().unwrap();
    println!("{}",stmt.read::<i64,_>("Count").unwrap());


}
