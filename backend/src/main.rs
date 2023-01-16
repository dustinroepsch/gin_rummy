use gin_rummy::game::card::Deck;

fn main() {
    let mut deck = Deck::new();
    println!(
        "Ordered deck {}\n\n",
        serde_json::to_string(&deck).unwrap()
    );
    deck.shuffle();
    println!(
        "Shuffled deck {}\n\n",
        serde_json::to_string(&deck).unwrap()
    );
    println!(
        "Dealing cards: {}, {}",
        deck.deal().unwrap(),
        deck.deal().unwrap()
    );
}
