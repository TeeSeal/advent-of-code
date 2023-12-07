
use super::card::Card;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Combination {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Combination {
    pub fn from(cards: &[Card; 5]) -> Self {
        let mut counts = HashMap::new();

        for card in cards {
            *counts.entry(card.to_owned()).or_insert(0) += 1;
        }

        let joker_count: u32 = counts.remove(&Card::Joker).unwrap_or(0);
        if joker_count >= 4 {
            return Self::FiveOfAKind;
        }

        let mut counts_vec: Vec<_> = counts.into_iter().collect();
        counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

        for (card, count) in &counts_vec {
            match count {
                5 => return Self::FiveOfAKind,
                4 => {
                    return match joker_count {
                        1 => Self::FiveOfAKind,
                        _ => Self::FourOfAKind,
                    }
                }
                3 => {
                    match joker_count {
                        2 => return Self::FiveOfAKind,
                        1 => return Self::FourOfAKind,
                        _ => (),
                    }

                    for (other_card, other_count) in &counts_vec {
                        if other_card != card && (other_count == &2 || joker_count == 1) {
                            return Self::FullHouse;
                        }
                    }

                    return Self::ThreeOfAKind;
                }
                2 => {
                    match joker_count {
                        3 => return Self::FiveOfAKind,
                        2 => return Self::FourOfAKind,
                        _ => (),
                    }

                    for (other_card, other_count) in &counts_vec {
                        if other_card != card && other_count == &2 {
                            return if joker_count == 1 {
                                Self::FullHouse
                            } else {
                                Self::TwoPair
                            };
                        }
                    }

                    return match joker_count {
                        1 => Self::ThreeOfAKind,
                        _ => Self::OnePair,
                    };
                }
                _ => {
                    return match joker_count {
                        3 => Self::FourOfAKind,
                        2 => Self::ThreeOfAKind,
                        1 => Self::OnePair,
                        _ => Self::HighCard,
                    }
                }
            }
        }

        Self::HighCard
    }
}
