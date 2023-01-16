use gin_rummy::game::Game;

fn main() {
    let game = Game::new();

    println!("{}", serde_json::to_string_pretty(&game).unwrap());
}
