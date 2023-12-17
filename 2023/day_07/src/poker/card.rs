#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Debug)]
pub struct ParseCardError;

impl Card {
    pub fn from_char(c: char, with_jokers: bool) -> Result<Self, ParseCardError> {
        Ok(match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => {
                if with_jokers {
                    Self::Joker
                } else {
                    Self::Jack
                }
            }
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => return Err(ParseCardError),
        })
    }
}
