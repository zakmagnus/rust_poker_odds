pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

pub enum Rank {
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
}

pub struct Card { suit: Suit, rank: Rank }
