pub mod food {
    use std::fmt;
    pub struct Food {
        location: Vec<f64>,
        nutrition: f64,
    }

    impl Food {
        pub fn new(loc: Vec<f64>) -> Food {
            Food {
                location: loc,
                nutrition: 1.0,
            }
        }

        pub fn get_location(&self) -> Vec<f64> {
            self.location.to_owned()
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
                self.location[0], self.location[1], self.nutrition
            )
        }
    }

    #[test]
    fn t_new() {
        let test_food = Food::new(vec![0.0,0.0]);
 
        let loc = test_food.get_location();
        assert_eq!(0.0,loc[0]);
        assert_eq!(0.0,loc[1]);
        
        // new food starts out with nutrition of 1
        let nut = test_food.get_nutrition();
        assert_eq!(1.0,nut);
    }
}

