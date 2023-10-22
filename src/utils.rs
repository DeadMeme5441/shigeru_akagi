use crate::structs::*;

pub fn get_suits(hand: &Vec<Card>) -> bool {
    let hand = hand.clone();

    let suits = ["H", "D", "C", "S"];

    for suit in suits {
        println!("{:?}", suit);
        let values = hand
            .iter()
            .filter(|c| c.suit == suit)
            .map(|c| c.value)
            .collect::<Vec<usize>>();
        println!("{:?}", values);
    }

    return true;
}
