use crate::{ProblemInput, Solution};
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct Deck {
    pub cards: VecDeque<i64>,
}

impl Deck {
    pub fn new(size: usize) -> Self {
        let mut cards = VecDeque::with_capacity(size);

        for i in 0..size {
            cards.push_back(i as i64);
        }

        Self { cards }
    }

    pub fn collect<T: FromIterator<i64>>(&self) -> T {
        self.cards.iter().cloned().collect()
    }

    #[inline(always)]
    pub fn deal_into_new_stack(&mut self) {
        // reverse the order of cards in the deck
        let mut reversed_cards = VecDeque::with_capacity(self.cards.len());

        while let Some(card) = self.cards.pop_front() {
            reversed_cards.push_front(card);
        }

        self.cards = reversed_cards;
    }

    #[inline(always)]
    pub fn cut(&mut self, n: i64) {
        if n > 0 {
            // if N > 0, then pop the first N cards from the front of the queue
            for _ in 0..n {
                let card = self.cards.pop_front().unwrap();
                self.cards.push_back(card);
            }
        } else if n < 0 {
            // otherwise pop the last N cards from the back of the queue
            for _ in 0..n.abs() {
                let card = self.cards.pop_back().unwrap();
                self.cards.push_front(card);
            }
        }
    }

    #[inline(always)]
    pub fn deal_with_increment(&mut self, n: usize) {
        // find the permutation
        let mut indices = (0..self.cards.len())
            .cycle()
            .step_by(n)
            .take(self.cards.len())
            .enumerate()
            .collect::<Vec<_>>();

        indices.sort_by_key(|(_, key)| *key);

        // apply it
        self.cards = indices
            .into_iter()
            .map(|(value, _)| self.cards[value])
            .collect();
    }
}

#[inline(always)]
fn puzzle_shuffle(deck: &mut Deck) {
    deck.cut(-135);
    deck.deal_with_increment(38);
    deck.deal_into_new_stack();
    deck.deal_with_increment(29);
    deck.cut(120);
    deck.deal_with_increment(30);
    deck.deal_into_new_stack();
    deck.cut(-7198);
    deck.deal_into_new_stack();
    deck.deal_with_increment(59);
    deck.cut(-8217);
    deck.deal_with_increment(75);
    deck.cut(4868);
    deck.deal_with_increment(29);
    deck.cut(4871);
    deck.deal_with_increment(2);
    deck.deal_into_new_stack();
    deck.deal_with_increment(54);
    deck.cut(777);
    deck.deal_with_increment(40);
    deck.cut(-8611);
    deck.deal_with_increment(3);
    deck.cut(-5726);
    deck.deal_with_increment(57);
    deck.deal_into_new_stack();
    deck.deal_with_increment(41);
    deck.deal_into_new_stack();
    deck.cut(-5027);
    deck.deal_with_increment(12);
    deck.cut(-5883);
    deck.deal_with_increment(45);
    deck.cut(9989);
    deck.deal_with_increment(14);
    deck.cut(6535);
    deck.deal_with_increment(18);
    deck.cut(-5544);
    deck.deal_with_increment(29);
    deck.deal_into_new_stack();
    deck.deal_with_increment(64);
    deck.deal_into_new_stack();
    deck.deal_with_increment(41);
    deck.deal_into_new_stack();
    deck.deal_with_increment(6);
    deck.cut(4752);
    deck.deal_with_increment(8);
    deck.deal_into_new_stack();
    deck.deal_with_increment(26);
    deck.cut(-6635);
    deck.deal_with_increment(10);
    deck.deal_into_new_stack();
    deck.cut(-3830);
    deck.deal_with_increment(48);
    deck.deal_into_new_stack();
    deck.deal_with_increment(39);
    deck.cut(-4768);
    deck.deal_with_increment(65);
    deck.deal_into_new_stack();
    deck.cut(-5417);
    deck.deal_with_increment(15);
    deck.cut(-4647);
    deck.deal_into_new_stack();
    deck.cut(-3596);
    deck.deal_with_increment(17);
    deck.cut(-3771);
    deck.deal_with_increment(50);
    deck.cut(1682);
    deck.deal_into_new_stack();
    deck.deal_with_increment(20);
    deck.deal_into_new_stack();
    deck.deal_with_increment(22);
    deck.deal_into_new_stack();
    deck.deal_with_increment(3);
    deck.cut(8780);
    deck.deal_with_increment(52);
    deck.cut(7478);
    deck.deal_with_increment(9);
    deck.cut(-8313);
    deck.deal_into_new_stack();
    deck.cut(742);
    deck.deal_with_increment(19);
    deck.cut(9982);
    deck.deal_into_new_stack();
    deck.deal_with_increment(68);
    deck.cut(9997);
    deck.deal_with_increment(23);
    deck.cut(-240);
    deck.deal_with_increment(54);
    deck.cut(-7643);
    deck.deal_into_new_stack();
    deck.deal_with_increment(6);
    deck.cut(-3493);
    deck.deal_with_increment(74);
    deck.deal_into_new_stack();
    deck.deal_with_increment(75);
    deck.deal_into_new_stack();
    deck.deal_with_increment(40);
    deck.cut(596);
    deck.deal_with_increment(6);
    deck.cut(-4957);
    deck.deal_into_new_stack();
}

pub struct Q22;

impl Solution for Q22 {
    fn part1(&self, _lines: &ProblemInput) -> i64 {
        let mut deck = Deck::new(10007);
        puzzle_shuffle(&mut deck);

        // find the location of 2019
        deck.cards
            .iter()
            .enumerate()
            .find(|(_, value)| **value == 2019)
            .unwrap()
            .0 as i64
    }

    fn part2(&self, _lines: &ProblemInput) -> i64 {
        0
    }
}
