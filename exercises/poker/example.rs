use std::fmt;
use std::cmp::Ordering;

#[macro_use]
extern crate try_opt;

extern crate counter;
use counter::Counter;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands = try_opt!(
        hands
            .iter()
            .map(|source| Hand::try_from(source))
            .collect::<Option<Vec<Hand>>>()
    );
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    hands.last().map(|last| {
        hands
            .iter()
            .rev()
            .take_while(|&item| item.partial_cmp(last) == Some(Ordering::Equal))
            .map(|hand| hand.source)
            .collect()
    })
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

impl Suit {
    fn try_from(source: &str) -> Option<Suit> {
        use Suit::*;
        match source {
            "S" => Some(Spades),
            "C" => Some(Clubs),
            "D" => Some(Diamonds),
            "H" => Some(Hearts),
            _ => None,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Suit::*;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn try_from(source: &str) -> Option<Rank> {
        use Rank::*;
        match source {
            "A" => Some(Ace),
            "K" => Some(King),
            "Q" => Some(Queen),
            "J" => Some(Jack),
            "10" => Some(Number(10)),
            "9" => Some(Number(9)),
            "8" => Some(Number(8)),
            "7" => Some(Number(7)),
            "6" => Some(Number(6)),
            "5" => Some(Number(5)),
            "4" => Some(Number(4)),
            "3" => Some(Number(3)),
            "2" => Some(Number(2)),
            _ => None,
        }
    }

    fn value(&self) -> usize {
        use Rank::*;
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
        use Rank::*;
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn try_from_split(source: &str, split: usize) -> Option<Card> {
        Some(Card {
            rank: try_opt!(Rank::try_from(&source[..split])),
            suit: try_opt!(Suit::try_from(&source[split..])),
        })
    }

    fn try_from(source: &str) -> Option<Card> {
        match source.len() {
            3 => Card::try_from_split(source, 2),
            2 => Card::try_from_split(source, 1),
            _ => None,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
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
        cards[0].rank.value() == 2 && cards[4].rank == Rank::Ace &&
            cards
                .windows(2)
                .take(3) // (0, 1), (1, 2), (2, 3) --> skips 4, ace
                .map(|pair| pair[1].rank.value() - pair[0].rank.value())
                .all(|diff| diff == 1)
    }

    fn analyze(cards: &[Card]) -> Option<PokerHand> {
        if cards.len() == 5 {
            let suit_counter = Counter::init(cards.iter().map(|c| c.suit));
            let is_flush = suit_counter
                .most_common()
                .map(|(_suit, count)| count)
                .next() == Some(5);
            // Note that `is_straight` depends on a precondition: it only works
            // if the input `cards` are sorted by rank value ascending.
            let is_straight = cards
                .windows(2)
                .map(|pair| pair[1].rank.value() - pair[0].rank.value())
                .all(|diff| diff == 1) ||
                PokerHand::is_ace_low_straight(cards);

            if is_flush && is_straight {
                return Some(PokerHand::StraightFlush);
            }

            let rank_counter = Counter::init(cards.iter().map(|c| c.rank));
            let mut rc_iter = rank_counter.most_common().map(|(_rank, count)| count);
            let rc_most = rc_iter.next();
            let rc_second = rc_iter.next();

            if rc_most == Some(4) {
                return Some(PokerHand::FourOfAKind);
            }
            if rc_most == Some(3) && rc_second == Some(2) {
                return Some(PokerHand::FullHouse);
            }
            if is_flush {
                return Some(PokerHand::Flush);
            }
            if is_straight {
                return Some(PokerHand::Straight);
            }
            if rc_most == Some(3) {
                return Some(PokerHand::ThreeOfAKind);
            }
            if rc_most == Some(2) && rc_second == Some(2) {
                return Some(PokerHand::TwoPair);
            }
            if rc_most == Some(2) {
                return Some(PokerHand::OnePair);
            }
            Some(PokerHand::HighCard)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    source: &'a str,
    cards: [Card; 5],
    hand_type: PokerHand,
}

impl<'a> Hand<'a> {
    fn try_from(source: &'a str) -> Option<Hand> {
        let mut cards = try_opt!(
            source
                .split_whitespace()
                .map(|s| Card::try_from(s))
                .collect::<Option<Vec<Card>>>()
        );
        cards.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
        if cards.len() == 5 {
            Some(Hand {
                source: source,
                cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
                hand_type: try_opt!(PokerHand::analyze(&cards)),
            })
        } else {
            None
        }
    }

    fn cmp_high_card(&self, other: &Hand, card: usize) -> Ordering {
        let mut ordering = self.cards[card].rank.value().cmp(
            &other.cards[card].rank.value(),
        );
        if card != 0 {
            ordering = ordering.then_with(|| self.cmp_high_card(other, card - 1));
        }
        ordering
    }

    fn value_by_frequency(&self) -> (Option<Rank>, Option<Rank>, Option<Rank>) {
        let rank_counter = Counter::init(self.cards.iter().map(|c| c.rank));
        let mut rc_iter = rank_counter
            .most_common_tiebreaker(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Less))
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

impl<'a> fmt::Display for Hand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.hand_type.cmp(&other.hand_type).then_with(|| {
            use PokerHand::*;
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
