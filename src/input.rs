use crate::*;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputEvent {
    ViewUp,
    ViewDown,
    ViewLeft,
    ViewRight,
    ViewLayerUp,
    ViewLayerDown,
    SelectUp,
    SelectDown,
    Cancel,
    Accept,
    CenterCursor,
    Inventory,
    Crafting,
    AttemptStartCraft(u32),
    Hanged(HangedInput),
    PlayerAction(PlayerAction),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveLayerUp,
    MoveLayerDown,
    Mine(Direction),
    StartCraft(ItemTransitions),
}

#[derive(Clone, Default, Debug)]
pub struct PlayerActionQueue {
    pub queue: VecDeque<PlayerAction>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keymap {
    pub map: HashMap<Input, InputEvent>,
}

impl Keymap {
    pub fn key_for_event(&self, ev: &InputEvent) -> Option<char> {
        for (k, v) in self.map.iter() {
            if v == ev {
                if let Input::Character(c) = k {
                    return Some(*c);
                }
            }
        }
        None
    }
}

impl Default for Keymap {
    fn default() -> Self {
        Keymap {
            map: [
                (
                    Input::Character('w'),
                    InputEvent::PlayerAction(PlayerAction::MoveUp),
                ),
                (
                    Input::Character('a'),
                    InputEvent::PlayerAction(PlayerAction::MoveLeft),
                ),
                (
                    Input::Character('s'),
                    InputEvent::PlayerAction(PlayerAction::MoveDown),
                ),
                (
                    Input::Character('d'),
                    InputEvent::PlayerAction(PlayerAction::MoveRight),
                ),
                (
                    Input::Character('q'),
                    InputEvent::PlayerAction(PlayerAction::MoveLayerDown),
                ),
                (
                    Input::Character('e'),
                    InputEvent::PlayerAction(PlayerAction::MoveLayerUp),
                ),
                (Input::Character('W'), InputEvent::ViewUp),
                (Input::Character('A'), InputEvent::ViewLeft),
                (Input::Character('S'), InputEvent::ViewDown),
                (Input::Character('D'), InputEvent::ViewRight),
                (Input::Character('Q'), InputEvent::ViewLayerDown),
                (Input::Character('E'), InputEvent::ViewLayerUp),
                // TODO find better bindings or use arrow keys here
                (Input::Character('1'), InputEvent::SelectUp),
                (Input::Character('2'), InputEvent::SelectDown),
                (Input::Character('z'), InputEvent::CenterCursor),
                (Input::Character('i'), InputEvent::Inventory),
                (Input::Character('c'), InputEvent::Crafting),
                (Input::Character('m'), InputEvent::Hanged(HangedInput::Mine)),
                (Input::Character('\n'), InputEvent::Accept),
                (Input::Character('\u{1b}'), InputEvent::Cancel), // Escape
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
}
