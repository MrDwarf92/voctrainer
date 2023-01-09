pub mod voclist;
use crate::voclist::VocabList;
use crate::voclist::Vocab;
use rand::Rng;
use std::io;

fn main() {

let mut text:String;
let mut number:usize;

let mut modus:String = String::new();

println!("Modus auswählen (1/2): ");
io::stdin().read_line(&mut modus).expect("");

let modus:i32 = modus.trim().parse().expect("");


let list:VocabList = VocabList::create_vocab_list(modus);
let list:&Vec<Vocab> = list.get_list();

let lang:String = String::from(if modus==1 {"Deutsch"} else {"Altisländisch"});

loop {

number = rand::thread_rng().gen_range(0..=100000);
number = number%list.len();

println!("Was heißt {} auf {}? ",list[number].get_word(),lang);

text = String::new();
io::stdin().read_line(&mut text).expect("");

text = String::from(text.trim());
    if text.eq("exit") {
    break;
}

if list[number].check_translation(&text)
{
    println!("Richtig!");
}
else {
    println!("Falsch! Die richtige Antwort ist {}",list[number].get_translation());
}

}

}
