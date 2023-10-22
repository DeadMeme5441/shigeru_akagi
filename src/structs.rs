extern crate rand;
use std::fmt::Debug;

use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct Card {
    pub suit: String,
    pub value: usize,
    pub face: String,
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&self.suit);
        s.push_str("");
        s.push_str(&self.value.to_string());
        write!(f, "{}", s)
    }
}

impl Deck {
    pub fn new(no: i32) -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for _ in 0..no {
            let suits = ["H", "D", "C", "S"];
            let faces = [
                "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
                "Jack", "Queen", "King",
            ];
            for suit in suits.iter() {
                for face in faces.iter() {
                    cards.push(Card {
                        suit: suit.to_string(),
                        value: faces.iter().position(|&r| r == *face).unwrap(),
                        face: face.to_string(),
                    });
                }
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            hand: Vec::new(),
        }
    }
    pub fn sort(&mut self) {
        self.hand.sort_by(|a, b| a.value.cmp(&b.value));
        self.hand.sort_by(|a, b| a.suit.cmp(&b.suit));
    }
}
