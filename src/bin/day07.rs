use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

trait HasValue {
    fn value(&self) -> i32;
}

trait HasHandType {
    fn score(&self) -> HandType;
}

#[derive(PartialEq, Eq, Default, Debug, Copy, Clone, Hash, PartialOrd, Ord)]
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
    #[default]
    Ace,
}

impl HasValue for Card {
    fn value(&self) -> i32 {
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

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash, PartialOrd, Ord)]
enum WildCard {
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

impl HasValue for WildCard {
    fn value(&self) -> i32 {
        match self {
            WildCard::Jack => 1,
            WildCard::Two => 2,
            WildCard::Three => 3,
            WildCard::Four => 4,
            WildCard::Five => 5,
            WildCard::Six => 6,
            WildCard::Seven => 7,
            WildCard::Eight => 8,
            WildCard::Nine => 9,
            WildCard::Ten => 10,
            WildCard::Queen => 11,
            WildCard::King => 12,
            WildCard::Ace => 13,
        }
    }
}

impl From<Card> for WildCard {
    fn from(value: Card) -> Self {
        match value {
            Card::Two => WildCard::Two,
            Card::Three => WildCard::Three,
            Card::Four => WildCard::Four,
            Card::Five => WildCard::Five,
            Card::Six => WildCard::Six,
            Card::Seven => WildCard::Seven,
            Card::Eight => WildCard::Eight,
            Card::Nine => WildCard::Nine,
            Card::Ten => WildCard::Ten,
            Card::Jack => WildCard::Jack,
            Card::Queen => WildCard::Queen,
            Card::King => WildCard::King,
            Card::Ace => WildCard::Ace,
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

impl TryFrom<&str> for Card {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let card_str = value.chars().nth(0).ok_or(())?;
        let card = Card::try_from(card_str)?;

        Ok(card)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Hand<T>(pub [T; 5]);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    // a straight is always uniquely identified by its start or end card. it's equivalent either way but we have to decide which it is
    // so in this case / program / context it is the high card.
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HasHandType for Hand<Card> {
    fn score(&self) -> HandType {
        let card_rank_bins = self.0.iter().counts_by(|e| *e);

        let mut as_vec = card_rank_bins.iter().collect::<Vec<(_, _)>>();
        // sort by rank value first
        as_vec.sort_by_key(|e| e.0.value());
        // then by count
        // thus, ranks that occur with equal count will still be ordered properly so that the highest rank is at the end
        as_vec.sort_by_key(|e| e.1);

        let (highest_count, second_highest_count) = (as_vec.pop().unwrap(), as_vec.pop());
        // println!("{:?} {:?}", highest_count, second_highest_count);

        let current_match = match (
            highest_count.1,
            second_highest_count.map(|e| *e.1).unwrap_or(0),
        ) {
            (3, 2) => HandType::FullHouse,
            // soundness: in the 3,1 case, we know that there must be one more card that is distinct from the 3 and the 1, which is the other 1
            (3, 1) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (4, 1) => HandType::FourOfAKind,
            (5, 0) => HandType::FiveOfAKind,
            (2, 1) => HandType::OnePair,
            (1, 1) => HandType::HighCard,
            _ => panic!("degenerate hand, this should not be possible."),
        };

        current_match
    }
}

impl HasHandType for Hand<WildCard> {
    fn score(&self) -> HandType {
        let card_rank_bins = self.0.iter().counts_by(|e| *e);

        let mut as_vec = card_rank_bins.iter().collect::<Vec<(_, _)>>();
        // sort by rank value first
        as_vec.sort_by_key(|e| e.0.value());
        // then by count
        // thus, ranks that occur with equal count will still be ordered properly so that the highest rank is at the end
        as_vec.sort_by_key(|e| e.1);

        let (highest_count, second_highest_count) = (as_vec.pop().unwrap(), as_vec.pop());
        // println!("{:?} {:?}", highest_count, second_highest_count);

        let current_match = match (
            highest_count.1,
            second_highest_count.map(|e| *e.1).unwrap_or(0),
        ) {
            (3, 2) => HandType::FullHouse,
            // soundness: in the 3,1 case, we know that there must be one more card that is distinct from the 3 and the 1, which is the other 1
            (3, 1) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (4, 1) => HandType::FourOfAKind,
            (5, 0) => HandType::FiveOfAKind,
            (2, 1) => HandType::OnePair,
            (1, 1) => HandType::HighCard,
            _ => panic!("degenerate hand, this should not be possible."),
        };

        current_match
    }
}

impl<T> TryFrom<&str> for Hand<T>
where
    T: Default + TryFrom<char> + Copy,
    (): From<<T as TryFrom<char>>::Error>,
{
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut hand_inner = [T::default(); 5];
        let split = value.chars();
        let five = split.take(5).collect::<Vec<_>>();
        if five.len() != 5 {
            return Err(());
        }

        hand_inner[0] = T::try_from(five[0])?;
        hand_inner[1] = T::try_from(five[1])?;
        hand_inner[2] = T::try_from(five[2])?;
        hand_inner[3] = T::try_from(five[3])?;
        hand_inner[4] = T::try_from(five[4])?;

        Ok(Hand(hand_inner))
    }
}

impl<T> PartialOrd for Hand<T>
where
    T: HasValue + Eq + PartialEq + PartialOrd,
    Hand<T>: HasHandType,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left_score = self.score();
        let right_score = other.score();
        let Some(ord) = left_score.partial_cmp(&right_score) else {
            return None;
        };
        match ord {
            a @ Ordering::Less | a @ Ordering::Greater => Some(a),
            Ordering::Equal => {
                let mut current_order = Ordering::Equal;
                for i in 0..5 {
                    if current_order != Ordering::Equal {
                        break;
                    }
                    current_order = self.0[i].partial_cmp(&other.0[i]).unwrap();
                }

                Some(current_order)
            }
        }
    }
}

impl<T> Ord for Hand<T>
where
    T: HasValue + Eq + PartialEq + PartialOrd,
    Hand<T>: HasHandType,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let file = File::open("data/day07.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut hands_and_bids = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let mut split = line.split_whitespace();
        let hand: Hand<Card> = split.next().unwrap().try_into().unwrap();
        let bid = split.next().unwrap().parse::<usize>().unwrap();

        hands_and_bids.push((hand, bid));
    }

    hands_and_bids.sort_by_key(|e| e.0);

    let mut winnings = 0;
    for (rank, (_, bid)) in hands_and_bids.iter().enumerate() {
        winnings += (1 + rank) * bid;
    }
    println!("{winnings}");

    let mut new_hands_and_bids = hands_and_bids
        .into_iter()
        .map(|(hand, bid)| (Hand(hand.0.map(|e| WildCard::from(e))), bid))
        .collect::<Vec<_>>();

    new_hands_and_bids.sort_by_key(|e| e.0);
}
