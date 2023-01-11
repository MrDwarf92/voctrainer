pub mod voclist;
pub mod dbhelper;
use crate::voclist::Vocab;
use rand::Rng;
use dbhelper::read_vocabs;
use dbhelper::into_next_drawer;
use dbhelper::random_from_lowest_drawer;
use std::io;

fn main() {

let mut text:String;
let mut number:usize;

let mut modus:String = String::new();

read_vocabs();
println!("Modus 1: OldNorse -> German");
println!("Modus 2: German -> OldNorse");
println!("Modus auswählen: ");
io::stdin().read_line(&mut modus).expect("");

let modus:usize = modus.trim().parse().expect("");

let lang:String = String::from(if modus==1 {"Deutsch"} else {"Altisländisch"});

loop {

let next_v: Vocab = random_from_lowest_drawer();

println!("Was heißt {} auf {}? ",next_v.get_word(modus),lang);

text = String::new();
io::stdin().read_line(&mut text).expect("");

text = String::from(text.trim());
    if text.eq("exit") {
    break;
}

if next_v.check_translation(&text,modus)
{
    println!("Richtig!");
    into_next_drawer(next_v.get_id());
}
else {
    println!("Falsch! Die richtige Antwort ist {}",next_v.get_translation(modus));
}

}

}
