use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash, PartialOrd, Ord)]
enum Card {
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
    Ace,
}

impl Card {
    pub fn value(&self) -> i32 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            // Some(v @ digit) => Err(()),
            // v.to_digit() => {Err(())},
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Card
// S: Into<Card>,
{
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let card_str = value.chars().nth(0).ok_or(())?;
        let card = Card::try_from(card_str)?;

        Ok(card)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Hand(pub [Card; 5]);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum HandScore {
    HighCard([Card; 5]),
    OnePair([Card; 4]),
    TwoPair([Card; 3]),
    ThreeOfAKind([Card; 3]),
    // a straight is always uniquely identified by its start or end card. it's equivalent either way but we have to decide which it is
    // so in this case / program / context it is the high card.
    Straight(Card),
    FullHouse(Card, Card),   // 3, 2
    FourOfAKind(Card, Card), // 4, 1
}

impl Hand {
    pub fn score(&self) -> HandScore {
        let card_rank_bins = self.0.iter().counts_by(|e| *e);

        let mut as_vec = card_rank_bins.iter().collect::<Vec<(_, _)>>();
        // sort by rank value first
        as_vec.sort_by_key(|e| e.0.value());
        // then by count
        // thus, ranks that occur with equal count will still be ordered properly so that the highest rank is at the end
        as_vec.sort_by_key(|e| e.1);

        // soundness: a well-formed hand (of 5 cards) will always have at least two unique ranks within it, as a deck only has 4 cards of the same rank
        let (highest_count, second_highest_count) = (as_vec.pop().unwrap(), as_vec.pop().unwrap());
        // println!("{:?} {:?}", highest_count, second_highest_count);

        let mut current_match = match (highest_count.1, second_highest_count.1) {
            (3, 2) => HandScore::FullHouse(*highest_count.0, *second_highest_count.0),
            // soundness: in the 3,1 case, we know that there must be one more card that is distinct from the 3 and the 1, which is the other 1
            (3, 1) => HandScore::ThreeOfAKind([
                *highest_count.0,
                *second_highest_count.0,
                *as_vec.pop().unwrap().0,
            ]),
            (2, 2) => HandScore::TwoPair([
                *highest_count.0,
                *second_highest_count.0,
                *as_vec.pop().unwrap().0,
            ]),
            (4, 1) => HandScore::FourOfAKind(*highest_count.0, *second_highest_count.0),
            (2, 1) => HandScore::OnePair([
                *highest_count.0,
                *second_highest_count.0,
                *as_vec.pop().unwrap().0,
                *as_vec.pop().unwrap().0,
            ]),
            (1, 1) => HandScore::HighCard([
                *highest_count.0,
                *second_highest_count.0,
                *as_vec.pop().unwrap().0,
                *as_vec.pop().unwrap().0,
                *as_vec.pop().unwrap().0,
            ]),
            _ => panic!("degenerate hand, this should not be possible."),
        };

        let mut ranks = card_rank_bins
            .clone()
            .into_iter()
            .map(|e| e.0)
            .collect::<Vec<_>>();
        ranks.sort();

        // full house trumps flush and straight, so don't override if we already detected a full house
        if ranks.len() == 5 && !matches!(current_match, HandScore::FullHouse(_, _)) {
            if ranks
                .windows(2)
                .all(|slice| (slice[1].value() - slice[0].value()) == 1)
            {
                // straight

                current_match = HandScore::Straight(*highest_count.0);
            } else if ranks.contains(&Card::Ace) {
                let value_with_remapped_ace = |e: Card| if e == Card::Ace { 1 } else { e.value() };
                ranks.sort_by_key(|e| value_with_remapped_ace(*e));
                if ranks.windows(2).all(|slice| {
                    (value_with_remapped_ace(slice[1]) - value_with_remapped_ace(slice[0])) == 1
                }) {
                    current_match = HandScore::Straight(*ranks.last().unwrap());
                }
            }
        }

        current_match
    }
}

impl TryFrom<&str> for Hand {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut hand_inner = [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace];
        let split = value.chars();
        let five = split.take(5).collect::<Vec<_>>();
        if five.len() != 5 {
            return Err(());
        }

        hand_inner[0] = Card::try_from(five[0])?;
        hand_inner[1] = Card::try_from(five[1])?;
        hand_inner[2] = Card::try_from(five[2])?;
        hand_inner[3] = Card::try_from(five[3])?;
        hand_inner[4] = Card::try_from(five[4])?;

        Ok(Hand(hand_inner))
    }
}

fn main() {
    let file = File::open("samples/day07.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {}
}
