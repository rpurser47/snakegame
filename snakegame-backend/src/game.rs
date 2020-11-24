pub mod game {
    use crate::snake::snake::Snake;
    use std::collections::HashMap;
    pub struct Game {
        size: f64,
        time: f64,
        snake: HashMap<usize,Snake>,
        next_snake_id: usize,
    }

     use std::fmt;
    pub struct NotFound;
    impl std::error::Error for NotFound {}
    impl fmt::Display for NotFound {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "No such snake!")
        }
    }
    impl fmt::Debug for NotFound {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "No such snake!")
        }
    }

    impl Game {
        pub fn new() -> Game {
            Game {
                size: 100.0,
                time: 0.0,
                snake: HashMap::new(),
                next_snake_id: 0,
            }
        }

        pub fn advance_clock(&mut self, delta_t: f64) {
            // Ignore attempts to turn back time
            if delta_t >= 0.0 {
                self.time += delta_t;
                for this_snake in self.snake.values_mut() {
                    this_snake.update(self.time);
                }
            }
        }

        pub fn get_size(&self) -> f64 {
            self.size
        }

        pub fn get_time(&self) -> f64 {
            self.time
        }

        pub fn add_snake(&mut self,event_time: f64) -> usize {
            let id = self.next_snake_id;
            self.next_snake_id += 1;
            self.snake.insert(id,Snake::new(event_time));
            id
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
                the_snake.turn(rad_relative, event_time);
                Ok(())
            } else {
                Err(NotFound)
            }
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
    fn t_get_time() {
        let mut test_game = Game::new();
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
        let mut test_game = Game::new();

        // Can't get a snake that isn't there
        let result = test_game.get_snake(0);
        assert!(result.is_err());
        // SHould really test for the error type, but don't know how.
        //assert!(result.map_err(|e| e.kind()));

        let snake_id = test_game.add_snake(0.0);
        assert!(test_game.get_snake(snake_id).is_ok());

        assert!(test_game.get_snake(snake_id + 1).is_err());
    }

    #[test]
    fn t_turn_snake() {
        use assert_approx_eq::assert_approx_eq;
        const PI:f64 = 3.14159;
        let mut test_game = Game::new();

        // Can't turn a snake that isn't there
        let result = test_game.turn_snake(0, 0.0, 0.0);
        assert!(result.is_err());
        // SHould really test for the error type, but don't know how.
        //assert!(result.map_err(|e| e.kind()));

        let snake_id = test_game.add_snake(0.0);
        // new snake should be at velY = 1 
        {
            let test_snake = test_game.get_snake(snake_id).unwrap();
            let vel = test_snake.get_velocity();
            assert_eq!(0.0,vel[0]);
            assert_eq!(1.0,vel[1]);
        }

        // snake should be at velY = -1 after a turn of PI radians (180 degrees)
        let result = test_game.turn_snake(snake_id, PI, 0.0);
        assert!(result.is_ok());
        {
            let test_snake = test_game.get_snake(snake_id).unwrap();
            let vel = test_snake.get_velocity();
            assert_approx_eq!(0.0,vel[0],1e-5);
            assert_approx_eq!(-1.0,vel[1],1e-5);
        }

        // snake should be at Y = -1 after one second
        test_game.advance_clock(1.0);
        {
            let test_snake = test_game.get_snake(snake_id).unwrap();
            let loc = test_snake.get_location();
            assert_approx_eq!( 0.0,loc[0],1e-5);
            assert_approx_eq!(-1.0,loc[1],1e-5);
        }

        // snake should be at vel (.71,-.71) after a turn of -PI/4.0 radians (-45 degrees)
        let result = test_game.turn_snake(snake_id, PI/4.0, 1.0);
        assert!(result.is_ok());
        {
            let test_snake = test_game.get_snake(snake_id).unwrap();
            let vel = test_snake.get_velocity();
            assert_approx_eq!( 0.7071,vel[0],1e-5);
            assert_approx_eq!(-0.7071,vel[1],1e-5);
        }

        // snake should be at pos (.71,-1.71) after two seconds
        test_game.advance_clock(1.0);
        {
            let test_snake = test_game.get_snake(snake_id).unwrap();
            let loc = test_snake.get_location();
            assert_approx_eq!( 0.7071,loc[0],1e-5);
            assert_approx_eq!(-1.7071,loc[1],1e-5);
            }
        
    }

    #[test]
    fn t_add_snake() {
 
        let mut test_game = Game::new();
        let snake1_id = test_game.add_snake(0.0);
        let snake2_id = test_game.add_snake(0.0);
        assert_ne!(snake1_id, snake2_id);

        // make sure the two snakes are independent
        test_game.turn_snake(snake2_id, 1.0, 0.0).expect("Expected to access 2nd snake");
        let snake1 = test_game.get_snake(snake1_id).unwrap();
        let snake2 = test_game.get_snake(snake2_id).unwrap();
        let vel = snake1.get_velocity();
        let vel2 = snake2.get_velocity();
        assert_ne!(vel[0],vel2[0]);
        assert_ne!(vel[1],vel2[1]);
    }
}