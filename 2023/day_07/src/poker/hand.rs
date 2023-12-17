use super::card::Card;
use super::combination::Combination;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Hand {
    pub bid: u32,
    pub cards: [Card; 5],
    pub combination: Combination,
}

#[derive(Debug)]
pub struct ParseHandError;

impl Hand {
    pub fn parse(s: &str, with_jokers: bool) -> Result<Self, ParseHandError> {
        let [cards_str, bid_str, ..] = s.split_whitespace().collect::<Vec<_>>()[..] else {
            return Err(ParseHandError);
        };

        let bid = bid_str.parse().map_err(|_| ParseHandError)?;
        let cards = cards_str
            .chars()
            .map(|c| Card::from_char(c, with_jokers).map_err(|_| ParseHandError))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| ParseHandError)?;

        let combination = Combination::from(&cards);

        Ok(Hand {
            bid,
            cards,
            combination,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let types_ordering = self.combination.cmp(&other.combination);

        if types_ordering != Ordering::Equal {
            return types_ordering;
        }

        self.cards.cmp(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
