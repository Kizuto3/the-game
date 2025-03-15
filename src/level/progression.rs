use std::fmt::Display;

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub enum Progression {
    #[default]
    None, 
    MetMilk,
    HasCherish,
    MilkWokeUp,
    HasLetter,
    GivenLetter
}

impl Display for Progression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}