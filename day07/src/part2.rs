use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .filter_map(Hand::try_parse)
        .sorted()
        .enumerate()
        .map(|(n, hand)| hand.bid * (n + 1))
        .sum()
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: HandType,
    bid: usize,
}

impl Hand {
    fn try_parse(line: &str) -> Option<Self> {
        let (hand, bid) = line.split_once(' ')?;
        Some(Self {
            cards: HandType::from(hand),
            bid: bid.parse().ok()?,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self.cards, &other.cards) {
            (HandType::FiveOfAKind(a), HandType::FiveOfAKind(b)) => a.cmp(b),
            (HandType::FiveOfAKind(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::FiveOfAKind(_)) => core::cmp::Ordering::Less,

            (HandType::FourOfAKind(a), HandType::FourOfAKind(b)) => a.cmp(b),
            (HandType::FourOfAKind(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::FourOfAKind(_)) => core::cmp::Ordering::Less,

            (HandType::FullHouse(a), HandType::FullHouse(b)) => a.cmp(b),
            (HandType::FullHouse(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::FullHouse(_)) => core::cmp::Ordering::Less,

            (HandType::ThreeOfAKind(a), HandType::ThreeOfAKind(b)) => a.cmp(b),
            (HandType::ThreeOfAKind(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::ThreeOfAKind(_)) => core::cmp::Ordering::Less,

            (HandType::TwoPair(a), HandType::TwoPair(b)) => a.cmp(b),
            (HandType::TwoPair(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::TwoPair(_)) => core::cmp::Ordering::Less,

            (HandType::OnePair(a), HandType::OnePair(b)) => a.cmp(b),
            (HandType::OnePair(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::OnePair(_)) => core::cmp::Ordering::Less,

            (HandType::HighCard(a), HandType::HighCard(b)) => a.cmp(b),
        }
    }
}

#[derive(Eq, PartialEq)]
enum HandType {
    FiveOfAKind(u32),
    FourOfAKind(u32),
    FullHouse(u32),
    ThreeOfAKind(u32),
    TwoPair(u32),
    OnePair(u32),
    HighCard(u32),
}

impl HandType {
    fn from(hand: &str) -> Self {
        let (counts, cards_number) = count_cards(hand);
        let jokers = counts[0];
        match (counts.iter().skip(1).max(), jokers) {
            (_, 5) | (_, 4) | (Some(5), 0) | (Some(4), 1) | (Some(3), 2) | (Some(2), 3) => {
                Self::FiveOfAKind(cards_number)
            }
            (_, 3) | (Some(4), 0) | (Some(3), 1) | (Some(2), 2) => Self::FourOfAKind(cards_number),
            (Some(3), 0) => {
                if counts.into_iter().any(|x| x == 2) {
                    Self::FullHouse(cards_number)
                } else {
                    Self::ThreeOfAKind(cards_number)
                }
            }
            (Some(1), 2) => Self::ThreeOfAKind(cards_number),
            (Some(2), 1) => {
                if counts.into_iter().filter(|x| *x == 2).count() == 2 {
                    Self::FullHouse(cards_number)
                } else {
                    Self::ThreeOfAKind(cards_number)
                }
            }
            (Some(2), 0) => {
                if counts.into_iter().filter(|x| *x == 2).count() == 2 {
                    Self::TwoPair(cards_number)
                } else {
                    Self::OnePair(cards_number)
                }
            }
            (Some(1), 1) => Self::OnePair(cards_number),
            _ => Self::HighCard(cards_number),
        }
    }
}

type CardCounts = [u8; 13];

fn count_cards(hand: &str) -> (CardCounts, u32) {
    let mut counts = [0; 13];
    let mut cards_number = 0;
    for card in hand.chars() {
        cards_number *= 13;
        match card {
            'J' => {
                counts[0] += 1;
            }
            'A' => {
                counts[1] += 1;
                cards_number += 12;
            }
            'K' => {
                counts[2] += 1;
                cards_number += 11;
            }
            'Q' => {
                counts[3] += 1;
                cards_number += 10;
            }
            'T' => {
                counts[4] += 1;
                cards_number += 9;
            }
            '9' => {
                counts[5] += 1;
                cards_number += 8;
            }
            '8' => {
                counts[6] += 1;
                cards_number += 7;
            }
            '7' => {
                counts[7] += 1;
                cards_number += 6;
            }
            '6' => {
                counts[8] += 1;
                cards_number += 5;
            }
            '5' => {
                counts[9] += 1;
                cards_number += 4;
            }
            '4' => {
                counts[10] += 1;
                cards_number += 3;
            }
            '3' => {
                counts[11] += 1;
                cards_number += 2;
            }
            '2' => {
                counts[12] += 1;
                cards_number += 1;
            }
            _ => panic!("unknown card: {}", card),
        }
    }
    (counts, cards_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 5905);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
