extern crate cards;

use cards::Rank;

// Rank arrays are used for kickers. They should be sorted descending.

enum Hand {
    HiCard        {ranks: [Rank, ..5]},
    Pair          {rank: Rank, kickers: [Rank, ..5]},
    TwoPair       {hiRank: Rank, loRank: Rank, kicker: Rank},
    Trips         {rank: Rank, kickers: [Rank, ..2]},
    Straight      {hiRank: Rank},
    Flush         {ranks: [Rank, ..5]},
    FullHouse     {threeOf: Rank, twoOf: Rank},
    Quads         {rank: Rank, kicker: Rank},
    StraightFlush {hiRank: Rank},
}
