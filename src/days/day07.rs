use crate::{Solution, SolutionPair};
use std::{char, cmp::Ordering, collections::HashMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day07.txt").unwrap();
    let mut hands = Vec::new();
    let mut joker_hands = Vec::new();
    for line in input.lines() {
        if let Some((cards_input, bet)) = line.split_once(" ") {
            let cards: Vec<Card> = cards_input
                .chars()
                .map(|c| Card::from_char(c).unwrap())
                .collect();

            let bet = bet.parse::<u32>().unwrap();
            
            let mut joker_cards = cards.clone();
            joker_cards.iter_mut().for_each(|c| {
                if *c == Card::Jack {
                    *c = Card::Joker;
                }
            });
            let hand = Hand::new(cards, bet);
            let joker_hand = Hand::new(joker_cards, bet);
            joker_hands.push(joker_hand);
            hands.push(hand);
        }
    }
    hands.sort();
    joker_hands.sort();

    let mut sol1 = 0;
    for i in 0..hands.len() {
        sol1 += hands[i].bet * (i as u32 + 1);
    }

    let mut sol2 = 0;
    for i in 0..joker_hands.len() {
        sol2 += joker_hands[i].bet * (i as u32 + 1);
    }

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bet: u32,
    hand_type: HandType,
}
impl Hand {
    fn new(cards: Vec<Card>, bet: u32) -> Hand {
        let hand_type = Hand::get_hand_type(cards.clone());
        Hand {
            cards,
            bet,
            hand_type: hand_type,
        }
    }
    fn get_hand_type(cards: Vec<Card>) -> HandType {
        let mut card_counts: HashMap<&Card, u32> = HashMap::new();
        let mut num_jokers = 0;
        for card in cards.iter() {
            if *card == Card::Joker {
                num_jokers += 1;
                continue;
            }
            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }
        let mut counts: Vec<u32> = card_counts.values().map(|v| *v).collect();
        if counts.len() == 0 {
            return HandType::FiveOfAKind;
        }
        counts.sort();
        counts.reverse();
        let hand_type = match (counts[0], num_jokers) {
            (5, 0) => HandType::FiveOfAKind,
            (4, 1) => HandType::FiveOfAKind,
            (3, 2) => HandType::FiveOfAKind,
            (2, 3) => HandType::FiveOfAKind,
            (1, 4) => HandType::FiveOfAKind,
            (4, 0) => HandType::FourOfAKind,
            (3, 1) => HandType::FourOfAKind,
            (2, 2) => HandType::FourOfAKind,
            (1, 3) => HandType::FourOfAKind,
            (3, 0) => {
                if counts[1] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            (2, 0) => {
                if counts[1] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            (1, 0) => HandType::HighCard,
            (2, 1) => {
                if counts[1] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            (1, 2) => HandType::ThreeOfAKind,
            (1, 1) => {
                if counts[1] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            (_, _) => match num_jokers {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 => HandType::ThreeOfAKind,
                2 => HandType::TwoPair,
                1 => HandType::OnePair,
                _ => HandType::HighCard,
            },
        };
        hand_type
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type)
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
            for i in 0..self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    return Some(self.cards[i].cmp(&other.cards[i]));
                }
            }
            return Some(self.cmp(other));
        }

        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

#[derive(Eq, Hash, Clone, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn value(&self) -> u32 {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
#[derive(Eq, Hash, Clone, Debug)]
enum Card {
    Ace,
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
    Joker,
}
impl Card {
    fn value(&self) -> u32 {
        match self {
            Card::Joker => 1,
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
    fn from_char(input: char) -> Option<Card> {
        match input {
            'A' => Some(Card::Ace),
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            _ => None,
        }
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
