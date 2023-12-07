use std::cmp::Ordering;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    let mut cards_and_bids: Vec<_> = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut split = line.split(' ');
            let hand = Hand::from_str(split.next().unwrap());
            let bid: u32 = split.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect();

    cards_and_bids.sort_by(|a, b| a.0.cmp(&b.0));

    let answer: u32 = cards_and_bids
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * bid)
        .sum();

    println!("{answer}");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(input: &str) -> Result<Card, Self::Err> {
        match input {
            "A" => Ok(Card::CA),
            "K" => Ok(Card::CK),
            "Q" => Ok(Card::CQ),
            "J" => Ok(Card::CJ),
            "T" => Ok(Card::CT),
            "9" => Ok(Card::C9),
            "8" => Ok(Card::C8),
            "7" => Ok(Card::C7),
            "6" => Ok(Card::C6),
            "5" => Ok(Card::C5),
            "4" => Ok(Card::C4),
            "3" => Ok(Card::C3),
            "2" => Ok(Card::C2),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        if input.len() != 5 {
            return Err(());
        }

        let mut cards = [Card::C2; 5];
        let mut card_counts = [(None, 0); 5];
        let mut empty_entry = 0;

        for i in 0..5 {
            cards[i] = Card::from_str(&input[i..i + 1]).unwrap();
            if let Some(p) = card_counts.iter().position(|e| e.0 == Some(cards[i])) {
                card_counts[p].1 += 1;
            } else {
                card_counts[empty_entry] = (Some(cards[i]), 1);
                empty_entry += 1;
            }
        }

        let hand_type = if empty_entry == 1 {
            HandType::FiveOfAKind
        } else if empty_entry == 2 {
            if card_counts[0].1 == 4 || card_counts[0].1 == 1 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        } else if empty_entry == 3 {
            if card_counts[0].1 == 3 || card_counts[1].1 == 3 || card_counts[2].1 == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        } else if empty_entry == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        Ok(Hand { cards, hand_type })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        self.cards
            .iter()
            .zip(other.cards.iter())
            .find_map(|(sc, oc)| if sc == oc { None } else { Some(sc.cmp(oc)) })
            .unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type
    }
}
