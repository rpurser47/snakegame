pub mod common {
    use std::fmt;

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Coordinates {pub x:f64, pub y:f64}
    impl fmt::Display for Coordinates {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "({:.1},{:.1})",
                self.x, self.y
            )
        }
    }

    #[test]
    fn t_coordinate_fmt() {
        let test_coordinates = Coordinates{x: 0.0, y:0.0};
        assert_eq!("(0.0,0.0)", format!("{}",test_coordinates));
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Velocity {pub delta_x:f64, pub delta_y:f64}
    impl fmt::Display for Velocity {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "<{:.1},{:.1}>",
                self.delta_x, self.delta_y
            )
        }
    }

    #[test]
    fn t_velocity_fmt() {
        let test_velocity = Velocity{delta_x: 0.0, delta_y:0.0};
        assert_eq!("<0.0,0.0>", format!("{}",test_velocity));
    }

}

pub mod game {
    use crate::snake::snake::Snake;
    use crate::snake::snake::SweepArea;
    use crate::food::food::Food;
    use crate::eventqueue::eventqueue::EventQueue;
    //use crate::game::common::{Coordinates,Velocity};
    use std::collections::HashMap;

    pub struct Game {
        size: f64,
        time: f64,
        snake: HashMap<usize,Snake>,
        next_snake_id: usize,
        food: HashMap<usize,Food>,
        next_food_id: usize,
        eventqueue: EventQueue,
    }

    use std::fmt;
    pub struct NotFound;
    impl std::error::Error for NotFound {}
    impl fmt::Display for NotFound {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "No such item!")
        }
    }
    impl fmt::Debug for NotFound {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "No such item!")
        }
    }

    impl Game {
        pub fn new() -> Game {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let mut new_game = Game::new_blank_game();
            
            for _i in 1..10 {
                let x = rng.gen_range(new_game.size / -2.0 , new_game.size / 2.0);
                let y = rng.gen_range(new_game.size / -2.0 , new_game.size / 2.0);
                new_game.add_food(x, y, 0.0);
            }
            new_game
        }

        fn _new_test_game() -> Game {
            let mut new_game = Game::new_blank_game();
            // lay out the food in a predicatible pattern
            new_game.add_food(-50.0, 50.0, 0.0);
            new_game.add_food(-40.0, 40.0, 0.0);
            new_game.add_food(-30.0, 30.0, 0.0);
            new_game.add_food(-20.0, 20.0, 0.0);
            new_game.add_food(-10.0, 10.0, 0.0);
            new_game.add_food( 10.0,-10.0, 0.0);
            new_game.add_food( 20.0,-20.0, 0.0);
            new_game.add_food( 30.0,-30.0, 0.0);
            new_game.add_food( 40.0,-40.0, 0.0);
            new_game.add_food( 50.0,-50.0, 0.0);
            new_game
        }

        fn new_blank_game() -> Game {
            Game {
                size: 100.0,
                time: 0.0,
                snake: HashMap::new(),
                next_snake_id: 0,
                food: HashMap::new(),
                next_food_id: 0,
                eventqueue: EventQueue::new(),
            }
        }

        pub fn advance_clock(&mut self, delta_t: f64) {
            // Ignore attempts to turn back time
            if delta_t < 0.0 {return};

            self.time += delta_t;
            let mut all_snake_coverage:HashMap<usize,SweepArea> = HashMap::new(); 
            {
                for (snake_id,this_snake) in self.snake.iter_mut() {
                    all_snake_coverage.insert(*snake_id,this_snake.update(self.time));
                }
            }
            for (snake_id,coverage) in all_snake_coverage { 
                let mut food_idxs_to_eat:Vec<usize> = Vec::new();
                for (food_id, this_food) in self.food.iter() {
                    // simple algorithm - if food location is inside rectangle enclosing coverage area, it's a hit
                    let foodloc = this_food.get_location();
                    if foodloc.x > coverage.start_loc.x.min(coverage.end_loc.x) - coverage.width / 2.0 &&
                        foodloc.x < coverage.start_loc.x.max(coverage.end_loc.x) + coverage.width / 2.0 &&
                        foodloc.y > coverage.start_loc.y.min(coverage.end_loc.y) - coverage.width / 2.0 &&
                        foodloc.y < coverage.start_loc.y.max(coverage.end_loc.y) +  coverage.width / 2.0 {
                            food_idxs_to_eat.push(*food_id);
                    }
                }
                self.snake_eats_food(snake_id,food_idxs_to_eat,self.time);
            }
            
        }

        fn snake_eats_food(&mut self, snake_id:usize, food_ids:Vec<usize>, event_time:f64) {
            for food_to_eat_id in food_ids {
                let food_to_eat = self.food.remove(&food_to_eat_id).unwrap();
                self.feed_snake(snake_id, food_to_eat.get_nutrition()).expect("Invalid snake ID");
                self.eventqueue.log_snake_ate(event_time, snake_id, food_to_eat_id);
            }
        }

        fn feed_snake(&mut self, snake_id:usize, nutrition:f64) -> Result<(), NotFound> {
            if let Some(the_snake) = self.snake.get_mut(&snake_id) {
                let _coverage = the_snake.eat(nutrition);
                Ok(())
            } else {
                Err(NotFound)
            }

        }

        pub fn get_size(&self) -> f64 {
            self.size
        }

        pub fn get_time(&self) -> f64 {
            self.time
        }

        pub fn create_snake(&mut self,event_time: f64) -> usize {
            let snake_id = self.next_snake_id;
            self.next_snake_id += 1;
            let new_snake = Snake::new(event_time);
            self.eventqueue.log_snake_born(event_time, snake_id, new_snake);
            self.snake.insert(snake_id,new_snake);
            snake_id
        }

        pub fn get_snake(&self, snake_id:usize) -> Result<&Snake, NotFound> {
            if let Some(the_snake) = self.snake.get(&snake_id) {
                Ok(the_snake)
            } else {
                Err(NotFound)
            }
        }

        pub fn turn_snake(&mut self, snake_id:usize,rad_relative: f64, event_time: f64) -> Result<(), NotFound> {
            if let Some(the_snake) = self.snake.get_mut(&snake_id) {
                self.eventqueue.log_snake_turned(event_time, snake_id, rad_relative);
                let _coverage = the_snake.turn(rad_relative, event_time);
                Ok(())
            } else {
                Err(NotFound)
            }
        }

        pub fn add_food(&mut self, x:f64, y:f64, event_time: f64) -> usize {
            let food_id = self.next_food_id;
            self.next_food_id += 1;
            let new_food = Food::new(x,y);
            self.eventqueue.log_food_added(event_time, food_id, new_food);
            self.food.insert(food_id,new_food);
            food_id
        }

        pub fn _get_food(&self, food_id: usize) -> Result<&Food, NotFound> {
            if let Some(the_food) = self.food.get(&food_id) {
                Ok(the_food)
            } else {
                Err(NotFound)
            }
        }

        pub fn _get_food_count(&self) -> usize {
            self.food.len()
        }

        pub fn get_event_queue_mut(&mut self) -> &mut EventQueue {
            &mut self.eventqueue
        }

    }

    #[test]
    fn t_new() {
        // new games start out at zero game time and default size of 100
        let test_game = Game::new();
        let time = test_game.get_time();
        assert_eq!(0.0,time);
        
        let size1 = test_game.get_size();
        assert_eq!(100.0,size1);
        
    }

    #[test]
    fn t_new_test_game() {
        // new games start out at zero game time and default size of 100
        let test_game = Game::_new_test_game();
        let time = test_game.get_time();
        assert_eq!(0.0,time);
        
        let size1 = test_game.get_size();
        assert_eq!(100.0,size1);

        assert_eq!(test_game._get_food_count(),10);
        
        let this_food = test_game._get_food(0).expect("Didn't get food as expected!");
        let loc = this_food.get_location();
        assert_eq!(loc.x,-50.0);
        assert_eq!(loc.y, 50.0);
        
        let this_food = test_game._get_food(8).expect("Didn't get food as expected!");
        let loc = this_food.get_location();
        assert_eq!(loc.x, 40.0);
        assert_eq!(loc.y,-40.0);
    }

    #[test]
    fn t_get_time() {
        let mut test_game = Game::new_blank_game();
        let time = test_game.get_time();
        assert_eq!(0.0,time);
    
        test_game.advance_clock(1.0);
        let time = test_game.get_time();
        assert_eq!(1.0,time);
    
        test_game.advance_clock(0.5);
        let time = test_game.get_time();
        assert_eq!(1.5,time);
    
        // Advancing the clock 0 is OK (but not useful)
        test_game.advance_clock(0.0);
        let time = test_game.get_time();
        assert_eq!(1.5,time);

        // attempts to turn back the clock are ignored
        test_game.advance_clock(-1.0);
        let time = test_game.get_time();
        assert_eq!(1.5,time);
    }

    #[test]
    fn t_get_snake() {
        let mut test_game = Game::new_blank_game();

        // Can't get a snake that isn't there
        let result = test_game.get_snake(0);
        assert!(result.is_err());
        // SHould really test for the error type, but don't know how.
        //assert!(result.map_err(|e| e.kind()));

        let snake_id = test_game.create_snake(0.0);
        assert!(test_game.get_snake(snake_id).is_ok());

        assert!(test_game.get_snake(snake_id + 1).is_err());
    }

    #[test]
    fn t_turn_snake() {
        use assert_approx_eq::assert_approx_eq;
        const PI:f64 = 3.14159;
        let mut test_game = Game::new_blank_game();

        // Can't turn a snake that isn't there
        let result = test_game.turn_snake(0, 0.0, 0.0);
        assert!(result.is_err());
        // SHould really test for the error type, but don't know how.
        //assert!(result.map_err(|e| e.kind()));

        let snake_id = test_game.create_snake(0.0);
        // new snake should be at velY = 1 
        {
            let test_snake = test_game.get_snake(snake_id).unwrap();
            let vel = test_snake.get_velocity();
            assert_eq!(0.0,vel.delta_x);
            assert_eq!(1.0,vel.delta_y);
        }

        // snake should be at velY = -1 after a turn of PI radians (180 degrees)
        let result = test_game.turn_snake(snake_id, PI, 0.0);
        assert!(result.is_ok());
        {
            let test_snake = test_game.get_snake(snake_id).unwrap();
            let vel = test_snake.get_velocity();
            assert_approx_eq!(0.0,vel.delta_x,1e-5);
            assert_approx_eq!(-1.0,vel.delta_y,1e-5);
        }

        // snake should be at Y = -1 after one second
        test_game.advance_clock(1.0);
        {
            let test_snake = test_game.get_snake(snake_id).unwrap();
            let loc = test_snake.get_location();
            assert_approx_eq!( 0.0,loc.x,1e-5);
            assert_approx_eq!(-1.0,loc.y,1e-5);
        }

        // snake should be at vel (.71,-.71) after a turn of -PI/4.0 radians (-45 degrees)
        let result = test_game.turn_snake(snake_id, PI/4.0, 1.0);
        assert!(result.is_ok());
        {
            let test_snake = test_game.get_snake(snake_id).unwrap();
            let vel = test_snake.get_velocity();
            assert_approx_eq!( 0.7071,vel.delta_x,1e-5);
            assert_approx_eq!(-0.7071,vel.delta_y,1e-5);
        }

        // snake should be at pos (.71,-1.71) after two seconds
        test_game.advance_clock(1.0);
        {
            let test_snake = test_game.get_snake(snake_id).unwrap();
            let loc = test_snake.get_location();
            assert_approx_eq!( 0.7071,loc.x,1e-5);
            assert_approx_eq!(-1.7071,loc.y,1e-5);
            }
        
    }

    #[test]
    fn t_create_snake() {

        let mut test_game = Game::new_blank_game();
        let snake1_id = test_game.create_snake(0.0);
        let snake2_id = test_game.create_snake(0.0);
        assert_ne!(snake1_id, snake2_id);

        // make sure the two snakes are independent
        test_game.turn_snake(snake2_id, 1.0, 0.0).expect("Expected to access 2nd snake");
        let snake1 = test_game.get_snake(snake1_id).unwrap();
        let snake2 = test_game.get_snake(snake2_id).unwrap();
        let vel = snake1.get_velocity();
        let vel2 = snake2.get_velocity();
        assert_ne!(vel.delta_x,vel2.delta_x);
        assert_ne!(vel.delta_y,vel2.delta_y);
    }

    #[test]
    fn t_add_food() {
        let mut test_game = Game::new_blank_game();

        assert_eq!(test_game._get_food_count(),0);

        let food_id = test_game.add_food(23.0, 34.0, 0.0);
        assert_eq!(test_game._get_food_count(),1);
        
        let this_food = test_game._get_food(food_id).expect("Didn't get food as expected!");
        let loc = this_food.get_location();
        assert_eq!(loc.x,23.0);
        assert_eq!(loc.y,34.0);

    }
}
