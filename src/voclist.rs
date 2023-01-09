use std::fs;

pub struct Vocab {
word:String,
translation:String
}

impl Vocab {

pub fn check_translation(&self,text:&String) -> bool {
    return self.translation.eq(text);
}

pub fn get_word(&self) -> &String
{
    return &self.word;
}

pub fn get_translation(&self) ->&String
{
    return &self.translation;
}

}


pub struct VocabList {
    list:Vec<Vocab>
}

impl VocabList {

   pub fn create_vocab_list(modus:i32) -> VocabList {
        let file_path:String = String::from("/home/stefan/Rust/voctrainer/src/vocabs.txt");
        let contents = fs::read_to_string(file_path).expect("");
        let split = contents.trim().split("\n");
        let lines:Vec<&str> = split.collect();

        let mut vlist:Vec<Vocab> = Vec::new();
        let mut counter:i32 = 0;

        let i_1 = if modus == 1 {0} else {1};
        let i_2 = if modus == 1 {1} else {0};

        for s in lines {
            if counter==0 {
            counter+=1; 
            continue;
            }
            let words:Vec<&str> = s.split(";").collect();
            vlist.push(Vocab{word:String::from(words[i_1]),
                translation:String::from(words[i_2])
            });
        }

        return VocabList{list:vlist};
    }

   pub fn get_list(&self) -> &Vec<Vocab> {
       return &(self.list)
   }
}
