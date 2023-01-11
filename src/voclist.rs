pub struct Vocab {
id: i64,
lang_1:String,
lang_2:String
}

impl Vocab {

pub fn check_translation(&self,text:&String,mode:usize) -> bool {
    return self.get_translation(mode).eq(text);
}

pub fn new(id:i64, lang_1:String, lang_2:String) -> Vocab {
    return Vocab {
        id,
        lang_1,
        lang_2
    };
}

pub fn get_word(&self, mode:usize) -> &String
{
    if mode==1 {
        return &self.lang_1;
    }
    else {
        return &self.lang_2;
    }
}

pub fn get_translation(&self, mode:usize) ->&String
{
    if mode==1 {
        return &self.lang_2;
    }
    else {
        return &self.lang_1;
    }
}

pub fn get_lang_1(&self) -> &String {
    return &self.lang_1;
}

pub fn get_lang_2(&self) -> &String {
    return &self.lang_2;
}

pub fn get_id(&self)->i64 {
    return self.id;
}
}

