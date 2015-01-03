extern crate cards;

use cards::Rank;

// Rank tuples are used for kickers. They should be sorted descending.

struct FiveOrderedRanks(Rank, Rank, Rank, Rank, Rank);
enum Hands {
    HiCard        {ranks: FiveOrderedRanks},
    Pair          {rank: Rank, kickers: (Rank, Rank, Rank)},
    TwoPair       {hiRank: Rank, loRank: Rank, kicker: Rank},
    Trips         {rank: Rank, kickers: (Rank, Rank)},
    Straight      {hiRank: Rank},
    Flush         {ranks: FiveOrderedRanks},
    FullHouse     {threeOf: Rank, twoOf: Rank},
    Quads         {rank: Rank, kicker: Rank},
    StraightFlush {hiRank: Rank},
}
