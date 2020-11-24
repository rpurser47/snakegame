pub mod snake {
    use std::fmt;
    pub struct Snake {
        location: Vec<f64>,
        velocity: Vec<f64>,
        last_updated: f64,
    }

    impl Snake {
        pub fn new(event_time: f64) -> Snake {
            Snake {
                location: vec![0.0, 0.0],
                velocity: vec![0.0, 1.0],
                last_updated: event_time,
            }
        }

        pub fn turn(&mut self, rad_relative: f64, event_time: f64) {
            // change the clock for this snake to when the turn happened
            self.update(event_time);
            // rotate the velocity vector by rad_relative
            // https://matthew-brett.github.io/teaching/rotation_2d.html
            // x2 = x1 * cos(β) − y1 * sin(β)
            // y2 = x1 * sin(β) + y1 * cos(β)
            let x2 = self.velocity[0]  * rad_relative.cos() - self.velocity[1] * rad_relative.sin();
            let y2 = self.velocity[0]  * rad_relative.sin() + self.velocity[1] * rad_relative.cos();
            self.velocity[0] = x2;
            self.velocity[1] = y2;
        }

        pub fn update(&mut self, game_time: f64) {
            let delta_t = game_time - self.last_updated;
            self.last_updated = game_time;
            self.location[0] += self.velocity[0] * delta_t;
            self.location[1] += self.velocity[1] * delta_t;
        }

        pub fn get_location(&self) -> Vec<f64> {
            self.location.to_owned()
        }

        pub fn get_velocity(&self) -> Vec<f64> {
            self.velocity.to_owned()
        }
    }

    impl fmt::Display for Snake {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "loc = ({:.1}, {:.1}), vel = ({:.1},{:.1})",
                self.location[0], self.location[1], self.velocity[0], self.velocity[1]
            )
        }
    }

    #[test]
    fn t_new() {
        let test_snake = Snake::new(0.0);
        // new snakes start out at the center
        let loc = test_snake.get_location();
        assert_eq!(0.0,loc[0]);
        assert_eq!(0.0,loc[1]);
        
        // new snakes start out going north
        let vel = test_snake.get_velocity();
        assert_eq!(0.0,vel[0]);
        assert_eq!(1.0,vel[1]);
    }

    #[test]
    fn t_update() {
        let mut test_snake = Snake::new(0.0);

        // new snake should be at Y = 1 at clock 1, Y = 2 at clock 2, Y = .5 at clock .5
        test_snake.update(1.0);
        let loc1 = test_snake.get_location();
        assert_eq!(0.0,loc1[0]);
        assert_eq!(1.0,loc1[1]);

        test_snake.update(2.0);
        let loc2 = test_snake.get_location();
        assert_eq!(0.0,loc2[0]);
        assert_eq!(2.0,loc2[1]);

        test_snake.update(0.5);
        let loc3 = test_snake.get_location();
        assert_eq!(0.0,loc3[0]);
        assert_eq!(0.5,loc3[1]);

    }

    #[test]
    fn t_turn() {
        const PI:f64 = 3.14159;
        use assert_approx_eq::assert_approx_eq;
        let mut test_snake = Snake::new(0.0);

        // new snake should be at velY = 1 
        let vel1 = test_snake.get_velocity();
        assert_eq!(0.0,vel1[0]);
        assert_eq!(1.0,vel1[1]);

        // snake should be at velY = -1 after a turn of PI radians (180 degrees)
        test_snake.turn(PI,0.0);
        let vel2 = test_snake.get_velocity();
        assert_approx_eq!(0.0,vel2[0],1e-5);
        assert_approx_eq!(-1.0,vel2[1],1e-5);

        // snake should be at Y = -1 after one second
        test_snake.update(1.0);
        let loc1 = test_snake.get_location();
        assert_approx_eq!( 0.0,loc1[0],1e-5);
        assert_approx_eq!(-1.0,loc1[1],1e-5);

       // snake should be at vel (.71,-.71) after a turn of -PI/4.0 radians (-45 degrees)
       test_snake.turn(PI/4.0,1.0);
       let vel3 = test_snake.get_velocity();
       assert_approx_eq!( 0.7071,vel3[0],1e-5);
       assert_approx_eq!(-0.7071,vel3[1],1e-5);

        // snake should be at pos (.71,-1.71) after two seconds
        test_snake.update(2.0);
        let loc2 = test_snake.get_location();
        assert_approx_eq!( 0.7071,loc2[0],1e-5);
        assert_approx_eq!(-1.7071,loc2[1],1e-5);

    }

}