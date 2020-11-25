mod snake;
mod food;
mod game;
mod eventqueue;
use crate::game::game::Game;
use crate::eventqueue::eventqueue::EventQueue;
const PI:f64 = 3.14159;

fn print_event_log (event_queue: &mut EventQueue) {
    loop {
        let this_event = event_queue.get_event();
        if this_event.is_none() {break};
        println!("{}",this_event.unwrap());
    }

}

fn main() {
    let mut my_game = Game::new();
    let snake_id = my_game.create_snake(0.0);
    my_game.add_food(0.0, 1.0, 0.0);
    my_game.add_food(-0.7, 1.7, 0.0);

    print_event_log(my_game.get_event_queue_mut());

    //println!("mySnake @ time {:.1}: {} ",my_game.get_time(),my_game.get_snake(snake_id).unwrap());
    
    my_game.advance_clock(1.0);
    print_event_log(my_game.get_event_queue_mut());
   
   //println!("mySnake @ time {:.1}: {} ",my_game.get_time(),my_game.get_snake(snake_id).unwrap());

    my_game.turn_snake(snake_id, PI / 4.0,1.0).expect("Can't find snake!");
    print_event_log(my_game.get_event_queue_mut());
    //println!("mySnake @ time {:.1}: {} ",my_game.get_time(),my_game.get_snake(snake_id).unwrap());
     
    my_game.advance_clock(1.0);
    print_event_log(my_game.get_event_queue_mut());
   //println!("mySnake @ time {:.1}: {} ",my_game.get_time(),my_game.get_snake(snake_id).unwrap());

}
