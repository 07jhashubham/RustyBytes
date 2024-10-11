pub fn enu() -> Option<u8> {
   let mut opt:Option<u8> = None;
   opt = Some(14);
   opt
}


pub fn ennu() -> Option<CharacterType>{
    let mut charty:Option<CharacterType> = None;
    charty  = Some(CharacterType::Arjun);
    charty
}

pub enum CharacterType {
    Bhim,
    Nakul,
    Sahdev,
    Arjun,
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Arjun => "Arjun",
            CharacterType::Bhim => "Bhim",
            CharacterType::Nakul => "Nakul",
            CharacterType::Sahdev => "Sahdev",
          
        }.to_string()
    }
}