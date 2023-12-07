pub fn solve_part2(input: &str) -> usize {
    let mut hands = input
        .lines()
        .filter_map(Hand::try_parse)
        .collect::<Vec<_>>();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(n, hand)| hand.bid * (n + 1))
        .sum()
}

#[derive(Eq, PartialEq)]
struct Hand<'a> {
    cards: HandType<'a>,
    bid: usize,
}

impl<'a> Hand<'a> {
    fn try_parse(line: &'a str) -> Option<Self> {
        let (hand, bid) = line.split_once(' ')?;
        Some(Self {
            cards: HandType::from(hand),
            bid: bid.parse().ok()?,
        })
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self.cards, &other.cards) {
            (HandType::FiveOfAKind(a), HandType::FiveOfAKind(b)) => cmp_cards(a, b),
            (HandType::FiveOfAKind(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::FiveOfAKind(_)) => core::cmp::Ordering::Less,

            (HandType::FourOfAKind(a), HandType::FourOfAKind(b)) => cmp_cards(a, b),
            (HandType::FourOfAKind(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::FourOfAKind(_)) => core::cmp::Ordering::Less,

            (HandType::FullHouse(a), HandType::FullHouse(b)) => cmp_cards(a, b),
            (HandType::FullHouse(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::FullHouse(_)) => core::cmp::Ordering::Less,

            (HandType::ThreeOfAKind(a), HandType::ThreeOfAKind(b)) => cmp_cards(a, b),
            (HandType::ThreeOfAKind(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::ThreeOfAKind(_)) => core::cmp::Ordering::Less,

            (HandType::TwoPair(a), HandType::TwoPair(b)) => cmp_cards(a, b),
            (HandType::TwoPair(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::TwoPair(_)) => core::cmp::Ordering::Less,

            (HandType::OnePair(a), HandType::OnePair(b)) => cmp_cards(a, b),
            (HandType::OnePair(_), _) => core::cmp::Ordering::Greater,
            (_, HandType::OnePair(_)) => core::cmp::Ordering::Less,

            (HandType::HighCard(a), HandType::HighCard(b)) => cmp_cards(a, b),
        }
    }
}

fn cmp_cards(lhs: &str, rhs: &str) -> std::cmp::Ordering {
    lhs.chars()
        .zip(rhs.chars())
        .find_map(|(lhs, rhs)| match (lhs, rhs) {
            ('J', 'J') => None,
            ('J', _) => Some(core::cmp::Ordering::Less),
            (_, 'J') => Some(core::cmp::Ordering::Greater),

            ('A', 'A') => None,
            ('A', _) => Some(core::cmp::Ordering::Greater),
            (_, 'A') => Some(core::cmp::Ordering::Less),

            ('K', 'K') => None,
            ('K', _) => Some(core::cmp::Ordering::Greater),
            (_, 'K') => Some(core::cmp::Ordering::Less),

            ('Q', 'Q') => None,
            ('Q', _) => Some(core::cmp::Ordering::Greater),
            (_, 'Q') => Some(core::cmp::Ordering::Less),

            ('T', 'T') => None,
            ('T', _) => Some(core::cmp::Ordering::Greater),
            (_, 'T') => Some(core::cmp::Ordering::Less),

            _ => match lhs.cmp(&rhs) {
                std::cmp::Ordering::Equal => None,
                x => Some(x),
            },
        })
        .unwrap_or(core::cmp::Ordering::Equal)
}

#[derive(Eq, PartialEq)]
enum HandType<'a> {
    FiveOfAKind(&'a str),
    FourOfAKind(&'a str),
    FullHouse(&'a str),
    ThreeOfAKind(&'a str),
    TwoPair(&'a str),
    OnePair(&'a str),
    HighCard(&'a str),
}

impl<'a> HandType<'a> {
    fn from(hand: &'a str) -> Self {
        let counts = count_cards(hand);
        let jokers = counts[0];
        match (counts.iter().skip(1).max(), jokers) {
            (_, 5) | (_, 4) | (Some(5), 0) | (Some(4), 1) | (Some(3), 2) | (Some(2), 3) => {
                Self::FiveOfAKind(hand)
            }
            (_, 3) | (Some(4), 0) | (Some(3), 1) | (Some(2), 2) => Self::FourOfAKind(hand),
            (Some(3), 0) => {
                if counts.into_iter().any(|x| x == 2) {
                    Self::FullHouse(hand)
                } else {
                    Self::ThreeOfAKind(hand)
                }
            }
            (Some(1), 2) => Self::ThreeOfAKind(hand),
            (Some(2), 1) => {
                if counts.into_iter().filter(|x| *x == 2).count() == 2 {
                    Self::FullHouse(hand)
                } else {
                    Self::ThreeOfAKind(hand)
                }
            }
            (Some(2), 0) => {
                if counts.into_iter().filter(|x| *x == 2).count() == 2 {
                    Self::TwoPair(hand)
                } else {
                    Self::OnePair(hand)
                }
            }
            (Some(1), 1) => Self::OnePair(hand),
            _ => Self::HighCard(hand),
        }
    }
}

type CardCounts = [u8; 13];

fn count_cards(hand: &str) -> CardCounts {
    let mut result = [0; 13];
    for card in hand.chars() {
        match card {
            'J' => result[0] += 1,
            'A' => result[1] += 1,
            'K' => result[2] += 1,
            'Q' => result[3] += 1,
            'T' => result[4] += 1,
            '9' => result[5] += 1,
            '8' => result[6] += 1,
            '7' => result[7] += 1,
            '6' => result[8] += 1,
            '5' => result[9] += 1,
            '4' => result[10] += 1,
            '3' => result[11] += 1,
            '2' => result[12] += 1,
            _ => panic!("unknown card: {}", card),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn part2_example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 5905);
    }

    #[test]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 251224870);
    }
}
