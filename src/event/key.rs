use std::rc::Rc;

use crate::{Event, EventBox, EventHandler, Template};

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub enum Key {
    Unknown,
    Backspace,
    Up,
    Down,
    Left,
    Right,
    Space,
    Enter,
    A(bool),
    B(bool),
    C(bool),
    D(bool),
    E(bool),
    F(bool),
    G(bool),
    H(bool),
    I(bool),
    J(bool),
    K(bool),
    L(bool),
    M(bool),
    N(bool),
    O(bool),
    P(bool),
    Q(bool),
    S(bool),
    R(bool),
    T(bool),
    U(bool),
    V(bool),
    W(bool),
    X(bool),
    Y(bool),
    Z(bool),
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Dot,
    QuestionMark,
    ExclamationMark,
}

impl From<Key> for &'static str {
    fn from(key: Key) -> &'static str {
        match key {
            Key::A(false) => "a",
            Key::B(false) => "b",
            Key::C(false) => "c",
            Key::D(false) => "d",
            Key::E(false) => "e",
            Key::F(false) => "f",
            Key::G(false) => "g",
            Key::H(false) => "h",
            Key::I(false) => "i",
            Key::J(false) => "j",
            Key::K(false) => "k",
            Key::L(false) => "l",
            Key::M(false) => "m",
            Key::N(false) => "n",
            Key::O(false) => "o",
            Key::P(false) => "p",
            Key::Q(false) => "q",
            Key::R(false) => "r",
            Key::S(false) => "s",
            Key::T(false) => "t",
            Key::U(false) => "u",
            Key::V(false) => "v",
            Key::W(false) => "w",
            Key::X(false) => "x",
            Key::Y(false) => "y",
            Key::Z(false) => "z",
            Key::A(true) => "A",
            Key::B(true) => "B",
            Key::C(true) => "C",
            Key::D(true) => "D",
            Key::E(true) => "E",
            Key::F(true) => "F",
            Key::G(true) => "G",
            Key::H(true) => "H",
            Key::I(true) => "I",
            Key::J(true) => "J",
            Key::K(true) => "K",
            Key::L(true) => "L",
            Key::M(true) => "M",
            Key::N(true) => "N",
            Key::O(true) => "O",
            Key::P(true) => "P",
            Key::Q(true) => "Q",
            Key::R(true) => "R",
            Key::S(true) => "S",
            Key::T(true) => "T",
            Key::U(true) => "U",
            Key::V(true) => "V",
            Key::W(true) => "W",
            Key::X(true) => "X",
            Key::Y(true) => "Y",
            Key::Z(true) => "Z",
            Key::Zero => "0",
            Key::One => "1",
            Key::Two => "2",
            Key::Three => "3",
            Key::Four => "4",
            Key::Five => "5",
            Key::Six => "6",
            Key::Seven => "7",
            Key::Eight => "8",
            Key::Nine => "9",
            Key::Space => " ",
            Key::Dot => ".",
            Key::QuestionMark => "?",
            Key::ExclamationMark => "!",
            _ => "",
        }
    }
}

impl From<Key> for Option<u8> {
    fn from(key: Key) -> Option<u8> {
        match key {
            Key::A(false) => Some(b'a'),
            Key::B(false) => Some(b'b'),
            Key::C(false) => Some(b'c'),
            Key::D(false) => Some(b'd'),
            Key::E(false) => Some(b'e'),
            Key::F(false) => Some(b'f'),
            Key::G(false) => Some(b'g'),
            Key::H(false) => Some(b'h'),
            Key::I(false) => Some(b'i'),
            Key::J(false) => Some(b'j'),
            Key::K(false) => Some(b'k'),
            Key::L(false) => Some(b'l'),
            Key::M(false) => Some(b'm'),
            Key::N(false) => Some(b'n'),
            Key::O(false) => Some(b'o'),
            Key::P(false) => Some(b'p'),
            Key::Q(false) => Some(b'q'),
            Key::R(false) => Some(b'r'),
            Key::S(false) => Some(b's'),
            Key::T(false) => Some(b't'),
            Key::U(false) => Some(b'u'),
            Key::V(false) => Some(b'v'),
            Key::W(false) => Some(b'w'),
            Key::X(false) => Some(b'x'),
            Key::Y(false) => Some(b'y'),
            Key::Z(false) => Some(b'z'),
            Key::A(true) => Some(b'A'),
            Key::B(true) => Some(b'B'),
            Key::C(true) => Some(b'C'),
            Key::D(true) => Some(b'D'),
            Key::E(true) => Some(b'E'),
            Key::F(true) => Some(b'F'),
            Key::G(true) => Some(b'G'),
            Key::H(true) => Some(b'H'),
            Key::I(true) => Some(b'I'),
            Key::J(true) => Some(b'J'),
            Key::K(true) => Some(b'K'),
            Key::L(true) => Some(b'L'),
            Key::M(true) => Some(b'M'),
            Key::N(true) => Some(b'N'),
            Key::O(true) => Some(b'O'),
            Key::P(true) => Some(b'P'),
            Key::Q(true) => Some(b'Q'),
            Key::R(true) => Some(b'R'),
            Key::S(true) => Some(b'S'),
            Key::T(true) => Some(b'T'),
            Key::U(true) => Some(b'U'),
            Key::V(true) => Some(b'V'),
            Key::W(true) => Some(b'W'),
            Key::X(true) => Some(b'X'),
            Key::Y(true) => Some(b'Y'),
            Key::Z(true) => Some(b'Z'),
            Key::Zero => Some(b'0'),
            Key::One => Some(b'1'),
            Key::Two => Some(b'2'),
            Key::Three => Some(b'3'),
            Key::Four => Some(b'4'),
            Key::Five => Some(b'5'),
            Key::Six => Some(b'6'),
            Key::Seven => Some(b'7'),
            Key::Eight => Some(b'8'),
            Key::Nine => Some(b'9'),
            Key::Space => Some(b' '),
            Key::Dot => Some(b'.'),
            Key::QuestionMark => Some(b'?'),
            Key::ExclamationMark => Some(b'!'),
            _ => None,
        }
    }
}

impl ToString for Key {
    fn to_string(&self) -> String {
        <&'static str>::from(*self).to_owned()
    }
}

impl From<char> for Key {
    fn from(sight: char) -> Self {
        match sight {
            'a' => Key::A(false),
            'b' => Key::B(false),
            'c' => Key::C(false),
            'd' => Key::D(false),
            'e' => Key::E(false),
            'f' => Key::F(false),
            'g' => Key::G(false),
            'h' => Key::H(false),
            'i' => Key::I(false),
            'j' => Key::J(false),
            'k' => Key::K(false),
            'l' => Key::L(false),
            'm' => Key::M(false),
            'n' => Key::N(false),
            'o' => Key::O(false),
            'p' => Key::P(false),
            'q' => Key::Q(false),
            'r' => Key::R(false),
            's' => Key::S(false),
            't' => Key::T(false),
            'u' => Key::U(false),
            'v' => Key::V(false),
            'w' => Key::W(false),
            'x' => Key::X(false),
            'y' => Key::Y(false),
            'z' => Key::Z(false),
            'A' => Key::A(true),
            'B' => Key::B(true),
            'C' => Key::C(true),
            'D' => Key::D(true),
            'E' => Key::E(true),
            'F' => Key::F(true),
            'G' => Key::G(true),
            'H' => Key::H(true),
            'I' => Key::I(true),
            'J' => Key::J(true),
            'K' => Key::K(true),
            'L' => Key::L(true),
            'M' => Key::M(true),
            'N' => Key::N(true),
            'O' => Key::O(true),
            'P' => Key::P(true),
            'Q' => Key::Q(true),
            'R' => Key::R(true),
            'S' => Key::S(true),
            'T' => Key::T(true),
            'U' => Key::U(true),
            'V' => Key::V(true),
            'W' => Key::W(true),
            'X' => Key::X(true),
            'Y' => Key::Y(true),
            'Z' => Key::Z(true),
            '0' => Key::Zero,
            '1' => Key::One,
            '2' => Key::Two,
            '3' => Key::Three,
            '4' => Key::Four,
            '5' => Key::Five,
            '6' => Key::Six,
            '7' => Key::Seven,
            '8' => Key::Eight,
            '9' => Key::Nine,
            ' ' => Key::Space,
            '.' => Key::Dot,
            '?' => Key::QuestionMark,
            '!' => Key::ExclamationMark,
            _ => Key::Unknown,
        }
    }
}

pub struct KeyDownEvent {
    pub key: Key,
}

impl Event for KeyDownEvent {}

pub struct KeyUpEvent {
    pub key: Key,
}

impl Event for KeyUpEvent {}

pub type KeyHandler = Fn(Key) -> bool + 'static;

pub struct KeyDownEventHandler {
    handler: Rc<KeyHandler>,
}

impl Into<Rc<dyn EventHandler>> for KeyDownEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for KeyDownEventHandler {
    fn handle_event(&self, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<KeyDownEvent>() {
            return (self.handler)(event.key);
        }

        return false;
    }
}

pub trait KeyDownHandler: Sized + From<Template> + Into<Template> {
    /// Transforms the handler into a template.
    fn template<F: FnOnce(Template) -> Template>(self, transform: F) -> Self {
        Self::from(transform(self.into()))
    }

    /// Inserts a handler.
    fn on_key_down<H: Fn(Key) -> bool + 'static>(self, handler: H) -> Self {
        self.template(|template| {
            template.event_handler(KeyDownEventHandler {
                handler: Rc::new(handler),
            })
        })
    }
}
