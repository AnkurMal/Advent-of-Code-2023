use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Type {
    HighCard, OnePair, TwoPair, ThreeKind, FullHouse, FourKind, FiveKind
}

#[derive(Debug)]
struct CamelCard<'a> {
    card: &'a str,
    bid: i32,
    ty: Type
}

impl<'a> CamelCard<'a> {
    fn new(card: &'a str, bid: i32, ty:Type) -> Self{
        Self {card, bid, ty}
    }
}

fn get_value(ch: char) -> u32 {
     match ch {
         'A' => 13,
         'K' => 12,
         'Q' => 11,
         'T' => 10,
         'J' => 1,
          _ => ch.to_digit(10).unwrap()
     }
}

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut camel_card = Vec::new();
    let mut sum = 0;

    for line in lines {
        let card: Vec<&str> = line.split_whitespace().collect();
        let mut hand = card[0].to_string();
        let mut pair = Vec::new();
        let mut j_count = 0;

        for i in card[0].chars() {
            let mut curr_char= ' ';
            let mut count = 0;

            while let Some(x) = hand.find(i) {
                count += 1;
                curr_char = hand.remove(x);
                if curr_char=='J' {
                    j_count += 1;
                }
            }
            if !(curr_char==' ') && count!=1 {
                pair.push(count);
            }
        }
        
        let ty = match pair[..] {
                [2]             => Type::OnePair,
                [2, 2]          => Type::TwoPair,
                [3]             => Type::ThreeKind,
                [2, 3] | [3, 2] => Type::FullHouse,
                [4]             => Type::FourKind,
                [5]             => Type::FiveKind,
                _               => Type::HighCard
        };

        let ty = match (ty, j_count) {
            (Type::HighCard, 1)                            => Type::OnePair,
            (Type::OnePair, 1|2)                           => Type::ThreeKind,
            (Type::TwoPair, 1)                             => Type::FullHouse,
            (Type::ThreeKind, 1|3) | (Type::TwoPair, 2)    => Type::FourKind,
            (Type::FullHouse, 2|3) | (Type::FourKind, 1|4) => Type::FiveKind,
            _                                              => ty
        };
        
        camel_card.push(CamelCard::new(card[0], card[1].parse().unwrap(), ty));
    }

    camel_card.sort_by(|a, b| {
        match a.ty.cmp(&b.ty) {
            Ordering::Equal => {
                let str1 = a.card;
                let str2 = b.card;
                for (char1, char2) in str1.chars().zip(str2.chars()) {
                    match get_value(char1).cmp(&get_value(char2)) {
                        Ordering::Equal => continue,
                        non_eq => return non_eq,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    });

    camel_card.iter().enumerate().for_each(|card| sum += (card.0 as i32+1)*card.1.bid);
    println!("{}", sum);
}
