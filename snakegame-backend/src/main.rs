mod snake;
mod food;
mod game;
use crate::game::game::Game;
//use crate::snake::snake::Snake;
const PI:f64 = 3.14159;

fn main() {
    let mut my_game = Game::new();
    let snake_id = my_game.add_snake(0.0);
    my_game.add_food(vec![0.0,1.0]);
    my_game.add_food(vec![-0.7,1.7]);
    my_game.get_size();

    println!("mySnake @ time {:.1}: {} ",my_game.get_time(),my_game.get_snake(snake_id).unwrap());
    
    my_game.advance_clock(1.0);
    println!("mySnake @ time {:.1}: {} ",my_game.get_time(),my_game.get_snake(snake_id).unwrap());

    my_game.turn_snake(snake_id, PI / 4.0,1.0).expect("Can't find snake!");
    println!("mySnake @ time {:.1}: {} ",my_game.get_time(),my_game.get_snake(snake_id).unwrap());
     
    my_game.advance_clock(1.0);
    println!("mySnake @ time {:.1}: {} ",my_game.get_time(),my_game.get_snake(snake_id).unwrap());

}
