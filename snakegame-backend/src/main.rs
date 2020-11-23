mod snake;
pub use crate::snake::snake::Snake;
const PI:f64 = 3.14159;

fn main() {
    let mut my_snake = Snake::new();
    println!("mySnake @ time 0: {} ",my_snake);
    my_snake.update(1.0);
    println!("mySnake @ time 1: {} ",my_snake);
    my_snake.turn(PI / 4.0,1.0);
    println!("mySnake @ time 1: {} ",my_snake);
    my_snake.update(2.0);
    println!("mySnake @ time 2: {} ",my_snake);
}
