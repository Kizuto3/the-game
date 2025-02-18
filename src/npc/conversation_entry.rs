use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum ConversationPosition {
    Left,
    Right
}

#[derive(Clone, Copy, Default, Debug)]
pub enum Emotion {
    #[default]
    Regular,
    Happy,
    Sad
}

#[derive(Clone)]
pub struct ConversationEntry {
    pub position: ConversationPosition,
    pub npc_name: String,
    pub text: String,
    pub emotion: Emotion
}

impl Display for Emotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}