#[derive(Clone, Copy)]
pub enum ConversationPosition {
    Left,
    Right
}

#[derive(Clone)]
pub struct ConversationEntry {
    pub position: ConversationPosition,
    pub npc_name: String,
    pub text: String
}