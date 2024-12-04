use std::cmp::Ordering;
use std::fmt;

use counter::Counter;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands = hands
        .iter()
        .map(|source| Hand::try_from(*source))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    hands
        .last()
        .map(|last| {
            hands
                .iter()
                .rev()
                .take_while(|&item| item.partial_cmp(last) == Some(Ordering::Equal))
                .map(|hand| hand.source)
                .collect()
        })
        .unwrap_or_default()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::Suit::*;
        write!(
            f,
            "{}",
            match *self {
                Spades => "S",
                Clubs => "C",
                Diamonds => "D",
                Hearts => "H",
            }
        )
    }
}

impl TryFrom<&str> for Suit {
    type Error = &'static str;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        use crate::Suit::*;
        match source {
            "S" => Ok(Spades),
            "C" => Ok(Clubs),
            "D" => Ok(Diamonds),
            "H" => Ok(Hearts),
            _ => Err("Invalid suit"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn value(&self) -> usize {
        use crate::Rank::*;
        match *self {
            Ace => 14,
            King => 13,
            Queen => 12,
            Jack => 11,
            Number(n) => n as usize,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::Rank::*;
        let num_str; // early declaration to placate NLL of Number case
        write!(
            f,
            "{}",
            match *self {
                Ace => "A",
                King => "K",
                Queen => "Q",
                Jack => "J",
                Number(n) => {
                    num_str = n.to_string();
                    &num_str
                }
            }
        )
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Rank) -> Option<Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl TryFrom<&str> for Rank {
    type Error = &'static str;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        use crate::Rank::*;
        match source {
            "A" => Ok(Ace),
            "K" => Ok(King),
            "Q" => Ok(Queen),
            "J" => Ok(Jack),
            "10" => Ok(Number(10)),
            "9" => Ok(Number(9)),
            "8" => Ok(Number(8)),
            "7" => Ok(Number(7)),
            "6" => Ok(Number(6)),
            "5" => Ok(Number(5)),
            "4" => Ok(Number(4)),
            "3" => Ok(Number(3)),
            "2" => Ok(Number(2)),
            _ => Err("Invalid rank"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn try_from_split(source: &str, split: usize) -> Result<Self, &'static str> {
        Ok(Card {
            rank: Rank::try_from(&source[..split])?,
            suit: Suit::try_from(&source[split..])?,
        })
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl TryFrom<&str> for Card {
    type Error = &'static str;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        match source.len() {
            3 => Card::try_from_split(source, 2),
            2 => Card::try_from_split(source, 1),
            _ => Err("Invalid card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl PokerHand {
    fn is_ace_low_straight(cards: &[Card]) -> bool {
        // special case: ace-low straight
        // still depends on the sorted precondition
        cards[0].rank.value() == 2
            && cards[4].rank == Rank::Ace
            && cards
                .windows(2)
                .take(3) // (0, 1), (1, 2), (2, 3) --> skips 4, ace
                .map(|pair| pair[1].rank.value() - pair[0].rank.value())
                .all(|diff| diff == 1)
    }

    fn analyze(cards: &[Card]) -> Result<Self, &'static str> {
        if cards.len() == 5 {
            let suit_counter = cards.iter().map(|c| c.suit).collect::<Counter<_>>();
            let is_flush = suit_counter
                .most_common()
                .into_iter()
                .map(|(_suit, count)| count)
                .next()
                == Some(5);
            // Note that `is_straight` depends on a precondition: it only works
            // if the input `cards` are sorted by rank value ascending.
            let is_straight = cards
                .windows(2)
                .map(|pair| pair[1].rank.value() - pair[0].rank.value())
                .all(|diff| diff == 1)
                || PokerHand::is_ace_low_straight(cards);

            if is_flush && is_straight {
                return Ok(PokerHand::StraightFlush);
            }

            let rank_counter = cards.iter().map(|c| c.rank).collect::<Counter<_>>();
            let mut rc_iter = rank_counter
                .most_common()
                .into_iter()
                .map(|(_rank, count)| count);
            let rc_most = rc_iter.next();
            let rc_second = rc_iter.next();

            if rc_most == Some(4) {
                return Ok(PokerHand::FourOfAKind);
            }
            if rc_most == Some(3) && rc_second == Some(2) {
                return Ok(PokerHand::FullHouse);
            }
            if is_flush {
                return Ok(PokerHand::Flush);
            }
            if is_straight {
                return Ok(PokerHand::Straight);
            }
            if rc_most == Some(3) {
                return Ok(PokerHand::ThreeOfAKind);
            }
            if rc_most == Some(2) && rc_second == Some(2) {
                return Ok(PokerHand::TwoPair);
            }
            if rc_most == Some(2) {
                return Ok(PokerHand::OnePair);
            }
            Ok(PokerHand::HighCard)
        } else {
            Err("Not a hand of five")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    source: &'a str,
    cards: [Card; 5],
    hand_type: PokerHand,
}

impl Hand<'_> {
    fn cmp_high_card(&self, other: &Hand, card: usize) -> Ordering {
        let mut ordering = self.cards[card]
            .rank
            .value()
            .cmp(&other.cards[card].rank.value());
        if card != 0 {
            ordering = ordering.then_with(|| self.cmp_high_card(other, card - 1));
        }
        ordering
    }

    fn value_by_frequency(&self) -> (Option<Rank>, Option<Rank>, Option<Rank>) {
        let rank_counter = self.cards.iter().map(|c| c.rank).collect::<Counter<_>>();
        let mut rc_iter = rank_counter
            .most_common_tiebreaker(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Less))
            .into_iter()
            .map(|(rank, _count)| rank);
        (rc_iter.next(), rc_iter.next(), rc_iter.next())
    }

    fn cmp_cascade_by_freq(&self, other: &Hand) -> Ordering {
        let (s1, s2, s3) = self.value_by_frequency();
        let (o1, o2, o3) = other.value_by_frequency();
        s1.partial_cmp(&o1)
            .map(|c| {
                c.then(
                    s2.partial_cmp(&o2)
                        .map(|c2| c2.then(s3.partial_cmp(&o3).unwrap_or(Ordering::Equal)))
                        .unwrap_or(Ordering::Equal),
                )
            })
            .unwrap_or(Ordering::Equal)
    }

    fn cmp_straight(&self, other: &Hand) -> Ordering {
        let s = if PokerHand::is_ace_low_straight(&self.cards) {
            5
        } else {
            self.cards[4].rank.value()
        };
        let o = if PokerHand::is_ace_low_straight(&other.cards) {
            5
        } else {
            other.cards[4].rank.value()
        };
        s.cmp(&o)
    }
}

impl fmt::Display for Hand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.hand_type.cmp(&other.hand_type).then_with(|| {
            use crate::PokerHand::*;
            match self.hand_type {
                HighCard => self.cmp_high_card(other, 4),
                OnePair => self.cmp_cascade_by_freq(other),
                TwoPair => self.cmp_cascade_by_freq(other),
                ThreeOfAKind => self.cmp_cascade_by_freq(other),
                Straight => self.cmp_straight(other),
                Flush => self.cmp_high_card(other, 4),
                FullHouse => self.cmp_cascade_by_freq(other),
                FourOfAKind => self.cmp_cascade_by_freq(other),
                StraightFlush => self.cmp_straight(other),
            }
        }))
    }
}

impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = &'static str;

    fn try_from(source: &'a str) -> Result<Self, Self::Error> {
        let mut cards = source
            .split_whitespace()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        cards.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
        if cards.len() == 5 {
            Ok(Hand {
                source,
                cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
                hand_type: PokerHand::analyze(&cards)?,
            })
        } else {
            Err("Invalid hand")
        }
    }
}
