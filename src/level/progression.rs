use std::fmt::Display;

#[derive(Clone, Copy, Default, Debug)]
pub enum Progression {
    #[default]
    None, 
    MetMilk,
}

impl Display for Progression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}