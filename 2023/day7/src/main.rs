use std::{fs, collections::HashMap, cmp::Ordering, iter::zip};


fn main() {
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    // let strength: HashMap<char, u32> = HashMap::from([
    //     ('A', 13),
    //     ('K', 12),
    //     ('Q', 11),
    //     ('J', 10),
    //     ('T', 9),
    //     ('9', 8),
    //     ('8', 7),
    //     ('7', 6),
    //     ('6', 5),
    //     ('5', 4),
    //     ('4', 3),
    //     ('3', 1),
    //     ('2', 0),
    // ]);

    let strength: HashMap<char, u32> = HashMap::from([
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
        ('J', 0),
    ]);

    let input = fs::read_to_string("input.txt").unwrap();
    let mut hands: Vec<_> = input.lines().map(|line| {
        let (hand, bid) = line.split_once(" ").unwrap();
        let hand_score = get_hand_score(hand);
        (hand, hand_score, bid.parse::<u32>().unwrap())
    }).collect();

    
    hands.sort_by(|(hand_1, hand_type_1, _), (hand_2, hand_type_2, _)| { 
        if *hand_type_1 != *hand_type_2 {
            let ord = hand_type_2.cmp(hand_type_1);
            ord
        } else {
            for (h1, h2) in zip(hand_1.chars(), hand_2.chars()) {
                if h1 != h2 {
                    let ord = strength.get(&h1).unwrap().cmp(strength.get(&h2).unwrap());
                    return ord
                }
            }
            panic!("no information on how to handle this case");
        }
    });
    
    dbg!(&hands);

    let total_winnings: u32 = hands.iter().enumerate().map(|(rank, (_, _, bid))| {(rank as u32 + 1) * bid}).sum();

    dbg!(&total_winnings);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType { 
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}


fn get_hand_score(hand: &str) -> HandType {
    let mut hand_map: HashMap<char, usize> = HashMap::new();
    for c in hand.chars() {
        *hand_map.entry(c).or_default() += 1;
    }

    // Apply joker rules
    let joker_count = hand_map.get(&'J').unwrap_or(&0).to_owned();

    let mut vec_of_map = hand_map.into_values().collect::<Vec<_>>();
    vec_of_map.sort_by(|c_1, c_2|(c_2.cmp(c_1)));

    let max_count_1 = *vec_of_map.get(0).unwrap();
    let max_count_2 = *vec_of_map.get(1).unwrap_or(&0);
    
    let morph = if max_count_1 == joker_count { max_count_2 } else { joker_count };
    let max_count_1 = max_count_1 + morph;

    if max_count_1 == 5 {
        HandType::FiveOfAKind
    } else if max_count_1 == 4 {
        HandType::FourOfAKind
    } else if max_count_1 == 3 && max_count_2 == 2 {
        HandType::FullHouse
    } else if max_count_1 == 3 {
        HandType::ThreeOfAKind
    } else if max_count_1 == 2 && max_count_2 == 2 {
        HandType::TwoPair
    } else if max_count_1 == 2 && max_count_2 == 1 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}