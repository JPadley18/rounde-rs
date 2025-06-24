use core::array::from_fn;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts
}

impl Suit {
    fn from_usize(i: usize) -> Suit {
        match i % 4 {
            0 => Suit::Spades,
            1 => Suit::Clubs,
            2 => Suit::Diamonds,
            3 => Suit::Hearts,

            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    value: usize,
    suit: Suit,
}

impl Card {
    pub fn new(value: usize, suit: Suit) -> Card {
        return Card {
            value: value,
            suit: suit,
        };
    }

    pub fn new_deck() -> [Card; 52] {
        from_fn::<Card, 52, _>(|i| {
            Card {
                value: (i % 13) + 1,
                suit: Suit::from_usize(i / 13),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_card() {
        let card = Card::new(13, Suit::Spades);
        assert_eq!(card.value, 13);
        assert_eq!(card.suit, Suit::Spades);
    }

    #[test]
    fn test_create_deck() {
        let deck = Card::new_deck();
        // Check that the content of the deck is valid
        for n in 2..14 {
            // There should be four of each value
            let value_cards: Vec<&Card> = deck.iter().filter(|x| x.value == n).collect();
            assert_eq!(value_cards.len(), 4, "check that there are four cards of value {n}");
            assert_eq!(value_cards.iter().filter(|x| x.suit == Suit::Spades).count(), 1, "check that there is only one {n} of Spades");
            assert_eq!(value_cards.iter().filter(|x| x.suit == Suit::Clubs).count(), 1, "check that there is only one {n} of Clubs");
            assert_eq!(value_cards.iter().filter(|x| x.suit == Suit::Diamonds).count(), 1, "check that there is only one {n} of Diamonds");
            assert_eq!(value_cards.iter().filter(|x| x.suit == Suit::Hearts).count(), 1, "check that there is only one {n} of Hearts");
        }
    }
}
