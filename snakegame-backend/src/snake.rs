pub mod snake {
    use std::fmt;
    use crate::game::common::{Coordinates,Velocity};

    pub struct SweepArea {
        pub start_loc: Coordinates,
        pub end_loc: Coordinates,
        pub width: f64,
    }
    impl fmt::Display for SweepArea {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "sweep area from {} to {}, {:.1}",
                self.start_loc, self.end_loc, self.width
            )
        }
    }

    #[derive(Copy, Clone)]
    pub struct Snake {
        location: Coordinates,
        velocity: Velocity,
        last_updated: f64,
        size: f64,
    }
    impl fmt::Display for Snake {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Snake at {}, vel {}, size {:.1}",
                self.location, self.velocity, self.size
            )
        }
    }

    impl Snake {
        pub fn new(event_time: f64) -> Snake {
            Snake {
                location: Coordinates {x:0.0, y:0.0},
                velocity: Velocity {delta_x:0.0, delta_y:1.0},
                last_updated: event_time,
                size: 1.0,
            }
        }

        pub fn turn(&mut self, rad_relative: f64, event_time: f64) -> SweepArea {
            // change the clock for this snake to when the turn happened
            let covered = self.update(event_time);
            // rotate the velocity vector by rad_relative
            // https://matthew-brett.github.io/teaching/rotation_2d.html
            // x2 = x1 * cos(β) − y1 * sin(β)
            // y2 = x1 * sin(β) + y1 * cos(β)
            let x2 = self.velocity.delta_x  * rad_relative.cos() - self.velocity.delta_y * rad_relative.sin();
            let y2 = self.velocity.delta_x  * rad_relative.sin() + self.velocity.delta_y * rad_relative.cos();
            self.velocity.delta_x = x2;
            self.velocity.delta_y = y2;
            covered
        }

        pub fn update(&mut self, game_time: f64) -> SweepArea {
            let mut covered = SweepArea {
                start_loc: self.location,
                end_loc: Coordinates {x:0.0, y:0.0},
                width: self.size,
            };
            let delta_t = game_time - self.last_updated;
            self.last_updated = game_time;
            self.location.x += self.velocity.delta_x * delta_t;
            self.location.y += self.velocity.delta_y * delta_t;
            covered.end_loc = self.location;
            covered
        }

        pub fn eat(&mut self, nutrition: f64) {
            self.size += nutrition / 10.0;
        }

        pub fn get_location(&self) -> Coordinates {
            self.location
        }

        pub fn get_velocity(&self) -> Velocity {
            self.velocity
        }

        pub fn get_size(&self) -> f64 {
            self.size
        }
    }

    #[test]
    fn t_new() {
        let test_snake = Snake::new(0.0);
        // new snakes start out at the center
        let loc = test_snake.get_location();
        assert_eq!(0.0,loc.x);
        assert_eq!(0.0,loc.y);
        
        // new snakes start out going north
        let vel = test_snake.get_velocity();
        assert_eq!(0.0,vel.delta_x);
        assert_eq!(1.0,vel.delta_y);

        assert_eq!(1.0,test_snake.get_size());
    }

    #[test]
    fn t_update() {
        let mut test_snake = Snake::new(0.0);

        // new snake should be at Y = 1 at clock 1, Y = 2 at clock 2, Y = .5 at clock .5
        let coverage = test_snake.update(1.0);
        assert_eq!(0.0,coverage.start_loc.x);
        assert_eq!(0.0,coverage.start_loc.y);
        assert_eq!(0.0,coverage.end_loc.x);
        assert_eq!(1.0,coverage.end_loc.y);
        assert_eq!(1.0,coverage.width);

        let loc = test_snake.get_location();
        assert_eq!(0.0,loc.x);
        assert_eq!(1.0,loc.y);

        let coverage = test_snake.update(2.0);
        assert_eq!(0.0,coverage.start_loc.x);
        assert_eq!(1.0,coverage.start_loc.y);
        assert_eq!(0.0,coverage.end_loc.x);
        assert_eq!(2.0,coverage.end_loc.y);
        assert_eq!(1.0,coverage.width);

        let loc = test_snake.get_location();
        assert_eq!(0.0,loc.x);
        assert_eq!(2.0,loc.y);

        let coverage = test_snake.update(0.5);
        assert_eq!(0.0,coverage.start_loc.x);
        assert_eq!(2.0,coverage.start_loc.y);
        assert_eq!(0.0,coverage.end_loc.x);
        assert_eq!(0.5,coverage.end_loc.y);
        assert_eq!(1.0,coverage.width);

        let loc = test_snake.get_location();
        assert_eq!(0.0,loc.x);
        assert_eq!(0.5,loc.y);

    }

    #[test]
    fn t_turn() {
        const PI:f64 = 3.14159;
        use assert_approx_eq::assert_approx_eq;
        let mut test_snake = Snake::new(0.0);

        // new snake should be at velY = 1 
        let vel = test_snake.get_velocity();
        assert_eq!(0.0,vel.delta_x);
        assert_eq!(1.0,vel.delta_y);

        // snake should be at velY = -1 after a turn of PI radians (180 degrees)
        let coverage = test_snake.turn(PI,0.0);
        assert_approx_eq!(0.0,coverage.start_loc.x,1e-5);
        assert_approx_eq!(0.0,coverage.start_loc.y,1e-5);
        assert_approx_eq!(0.0,coverage.end_loc.x,1e-5);
        assert_approx_eq!(0.0,coverage.end_loc.y,1e-5);
        assert_eq!(1.0,coverage.width);

        let vel = test_snake.get_velocity();
        assert_approx_eq!( 0.0,vel.delta_x,1e-5);
        assert_approx_eq!(-1.0,vel.delta_y,1e-5);

        // snake should be at Y = -1 after one second
        let coverage = test_snake.update(1.0);
        assert_approx_eq!( 0.0,coverage.start_loc.x,1e-5);
        assert_approx_eq!( 0.0,coverage.start_loc.y,1e-5);
        assert_approx_eq!( 0.0,coverage.end_loc.x,1e-5);
        assert_approx_eq!(-1.0,coverage.end_loc.y,1e-5);
        assert_eq!(1.0,coverage.width);
        let loc = test_snake.get_location();
        assert_approx_eq!( 0.0,loc.x,1e-5);
        assert_approx_eq!(-1.0,loc.y,1e-5);

    // snake should be at vel (.71,-.71) after a turn of -PI/4.0 radians (-45 degrees)
    let coverage = test_snake.turn(PI/4.0,1.0);
    assert_approx_eq!( 0.0,coverage.start_loc.x,1e-5);
    assert_approx_eq!(-1.0,coverage.start_loc.y,1e-5);
    assert_approx_eq!( 0.0,coverage.end_loc.x,1e-5);
    assert_approx_eq!(-1.0,coverage.end_loc.y,1e-5);
    assert_eq!(1.0,coverage.width);
    let vel = test_snake.get_velocity();
    assert_approx_eq!( 0.7071,vel.delta_x,1e-5);
    assert_approx_eq!(-0.7071,vel.delta_y,1e-5);

        // snake should be at pos (.71,-1.71) after two seconds
        let coverage = test_snake.update(2.0);
        assert_approx_eq!( 0.0,coverage.start_loc.x,1e-5);
        assert_approx_eq!(-1.0,coverage.start_loc.y,1e-5);
        assert_approx_eq!( 0.7071,coverage.end_loc.x,1e-5);
        assert_approx_eq!(-1.7071,coverage.end_loc.y,1e-5);
        assert_eq!(1.0,coverage.width);
        let loc = test_snake.get_location();
        assert_approx_eq!( 0.7071,loc.x,1e-5);
        assert_approx_eq!(-1.7071,loc.y,1e-5);

    }

    #[test]
    fn t_eat() {
        let mut test_snake = Snake::new(0.0);
        assert_eq!(1.0,test_snake.get_size());

        test_snake.eat(10.0);
        assert_eq!(2.0,test_snake.get_size());
    }

    #[test]
    fn t_snake_fmt() {
        let test_snake = Snake::new(0.0);
        assert_eq!("Snake at (0.0,0.0), vel <0.0,1.0>, size 1.0", format!("{}",test_snake));
    }

}
