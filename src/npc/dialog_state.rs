use bevy::state::state::States;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DialogState {
    #[default]
    None,
    LeftCharacterTalking,
    RightCharacterTalking,
}