use gin_rummy::game::{deck::Deck, Game};

fn main() {
    let game = Game::new();

    println!("{}", serde_json::to_string_pretty(&game).unwrap());
}
