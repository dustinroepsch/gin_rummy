use gin_rummy::game::card::Deck;

fn main() {
    let mut deck = Deck::new();
    println!("Orderd deck: {deck}");
    deck.shuffle();
    println!("Shuffled deck {deck}");
    println!(
        "Dealt cards: {}, {}",
        deck.deal().unwrap(),
        deck.deal().unwrap()
    );
    print!("{deck}");
}
