mod structs;
mod utils;
use structs::*;
use utils::*;

fn main() {
    let mut deck: Deck = Deck::new(1);

    deck.shuffle();

    let mut a: Player = Player::new("Alice".to_string());
    let mut b: Player = Player::new("Bob".to_string());

    for _ in 0..13 {
        a.hand.push(deck.cards.pop().unwrap());
        b.hand.push(deck.cards.pop().unwrap());
    }

    let joker: Card = deck.cards.pop().unwrap();

    println!("{:?} \n", a.hand);
    a.sort();
    println!("{:?} \n", a.hand);
    get_suits(&a.hand);
}
