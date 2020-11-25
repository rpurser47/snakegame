pub mod food {
    use std::fmt;
    use crate::game::common::Coordinates;

    pub struct Food {
        location: Coordinates,
        nutrition: f64,
    }

    impl Food {
        pub fn new(x_placement:f64, y_placement:f64) -> Food {
            Food {
                location: Coordinates{x: x_placement, y: y_placement},
                nutrition: 1.0,
            }
        }

        pub fn get_location(&self) -> Coordinates {
            self.location
        }

        pub fn get_nutrition(&self) -> f64 {
            self.nutrition
        }
    }

    impl fmt::Display for Food {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "loc = ({:.1}, {:.1}), nutrition = {:.1}",
                self.location.x, self.location.y, self.nutrition
            )
        }
    }

    #[test]
    fn t_new() {
        let test_food = Food::new(0.0,0.0);
 
        let loc = test_food.get_location();
        assert_eq!(0.0,loc.x);
        assert_eq!(0.0,loc.y);
        
        // new food starts out with nutrition of 1
        let nut = test_food.get_nutrition();
        assert_eq!(1.0,nut);
    }
}

