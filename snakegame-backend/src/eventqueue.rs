pub mod eventqueue {
    use crate::food::food::Food;
    use crate::game::common::{Coordinates, Velocity};
    use crate::snake::snake::Snake;
    use std::collections::VecDeque;
    use std::fmt;

    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum EventKind {
        SnakeBorn {
            snake_id: usize,
            loc: Coordinates,
            vel: Velocity,
            size: f64,
        },
        SnakeKilled {
            snake_id: usize,
        },
        SnakeTurned {
            snake_id: usize,
            rad_relative: f64,
        },
        SnakeAte {
            snake_id: usize,
            food_id: usize,
        },
        FoodAdded {
            food_id: usize,
            loc: Coordinates,
            nutrition: f64,
        },
    }
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub struct Event {
        pub kind: EventKind,
        pub event_time: f64,
    }
    impl fmt::Display for Event {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.kind {
                EventKind::SnakeBorn {
                    snake_id,
                    loc,
                    vel,
                    size,
                } => write!(
                    f,
                    "{:3.1}: snake {} born at {} headed {} of size {:.1}",
                    self.event_time, snake_id, loc, vel, size
                ),
                EventKind::SnakeKilled { snake_id } => {
                    write!(f, "{:3.1}: snake {} killed", self.event_time, snake_id)
                }
                EventKind::SnakeTurned {
                    snake_id,
                    rad_relative,
                } => write!(
                    f,
                    "{:3.1}: snake {} turned {:+.3} rad",
                    self.event_time, snake_id, rad_relative
                ),
                EventKind::SnakeAte { snake_id, food_id } => write!(
                    f,
                    "{:3.1}: snake {} ate food {}",
                    self.event_time, snake_id, food_id
                ),
                EventKind::FoodAdded {
                    food_id,
                    loc,
                    nutrition,
                } => write!(
                    f,
                    "{:3.1}: food {} added at {} of nutrition {:.1}",
                    self.event_time, food_id, loc, nutrition
                ),
            }
        }
    }

    pub struct EventQueue {
        queue: VecDeque<Event>,
    }
    impl EventQueue {
        pub fn new() -> EventQueue {
            EventQueue {
                queue: VecDeque::new(),
            }
        }

        pub fn log_snake_born(&mut self, event_time: f64, snake_id: usize, new_snake: Snake) {
            self.log_event(Event {
                kind: EventKind::SnakeBorn {
                    snake_id: snake_id,
                    loc: new_snake.get_location(),
                    vel: new_snake.get_velocity(),
                    size: new_snake.get_size(),
                },
                event_time: event_time,
            });
        }

        pub fn _log_snake_killed(&mut self, event_time: f64, snake_id: usize) {
            self.log_event(Event {
                kind: EventKind::SnakeKilled { snake_id: snake_id },
                event_time: event_time,
            });
        }

        pub fn log_snake_turned(&mut self, event_time: f64, snake_id: usize, rad_relative: f64) {
            self.log_event(Event {
                kind: EventKind::SnakeTurned {
                    snake_id: snake_id,
                    rad_relative: rad_relative,
                },
                event_time: event_time,
            });
        }

        pub fn log_snake_ate(&mut self, event_time: f64, snake_id: usize, food_id: usize) {
            self.log_event(Event {
                kind: EventKind::SnakeAte {
                    snake_id: snake_id,
                    food_id: food_id,
                },
                event_time: event_time,
            });
        }

        pub fn log_food_added(&mut self, event_time: f64, food_id: usize, new_food: Food) {
            self.log_event(Event {
                kind: EventKind::FoodAdded {
                    food_id: food_id,
                    loc: new_food.get_location(),
                    nutrition: new_food.get_nutrition(),
                },
                event_time: event_time,
            });
        }

        pub fn log_event(&mut self, event: Event) {
            self.queue.push_back(event);
        }

        pub fn get_event(&mut self) -> Option<Event> {
            self.queue.pop_front()
        }

        pub fn _is_empty(&self) -> bool {
            self.queue.is_empty()
        }
    }

    #[test]
    fn t_new() {
        let test_event_queue = EventQueue::new();
        assert!(true, test_event_queue._is_empty());
        assert_eq!(0, test_event_queue.queue.len());
    }

    #[test]
    fn t_log_event() {
        let mut test_event_queue = EventQueue::new();

        // log one of each kind of event
        let event = Event {
            kind: EventKind::SnakeBorn {
                snake_id: 0,
                loc: Coordinates { x: 0.0, y: 0.0 },
                vel: Velocity {
                    delta_x: 0.0,
                    delta_y: 0.0,
                },
                size: 0.0,
            },
            event_time: 0.0,
        };
        test_event_queue.log_event(event);
        assert_eq!(1, test_event_queue.queue.len());

        let event = Event {
            kind: EventKind::SnakeKilled { snake_id: 0 },
            event_time: 0.0,
        };
        test_event_queue.log_event(event);
        assert_eq!(2, test_event_queue.queue.len());

        let event = Event {
            kind: EventKind::SnakeTurned {
                snake_id: 0,
                rad_relative: 0.0,
            },
            event_time: 0.0,
        };
        test_event_queue.log_event(event);
        assert_eq!(3, test_event_queue.queue.len());

        let event = Event {
            kind: EventKind::SnakeAte {
                snake_id: 0,
                food_id: 0,
            },
            event_time: 0.0,
        };
        test_event_queue.log_event(event);
        assert_eq!(4, test_event_queue.queue.len());

        let event = Event {
            kind: EventKind::FoodAdded {
                food_id: 0,
                loc: Coordinates { x: 0.0, y: 0.0 },
                nutrition: 0.0,
            },
            event_time: 0.0,
        };
        test_event_queue.log_event(event);
        assert_eq!(5, test_event_queue.queue.len());
    }

    #[test]
    fn tget_event() {
        let mut test_event_queue = EventQueue::new();
        assert_eq!(0, test_event_queue.queue.len());

        // log one event and see if we get it back
        let event_logged = Event {
            kind: EventKind::SnakeBorn {
                snake_id: 0,
                loc: Coordinates { x: 0.0, y: 0.0 },
                vel: Velocity {
                    delta_x: 0.0,
                    delta_y: 0.0,
                },
                size: 0.0,
            },
            event_time: 0.0,
        };
        test_event_queue.log_event(event_logged);
        assert_eq!(1, test_event_queue.queue.len());

        let event_gotten = test_event_queue.get_event().unwrap();
        assert_eq!(0, test_event_queue.queue.len());
        assert_eq!(event_logged, event_gotten);

        // log two events and see if we get them back in the same order
        let event_logged1 = Event {
            kind: EventKind::SnakeTurned {
                snake_id: 0,
                rad_relative: 0.0,
            },
            event_time: 0.0,
        };
        test_event_queue.log_event(event_logged1);

        let event_logged2 = Event {
            kind: EventKind::SnakeKilled { snake_id: 0 },
            event_time: 0.0,
        };
        test_event_queue.log_event(event_logged2);
        assert_eq!(2, test_event_queue.queue.len());

        let event_gotten1 = test_event_queue.get_event().unwrap();
        assert_eq!(1, test_event_queue.queue.len());
        assert_eq!(event_logged1, event_gotten1);

        let event_gotten2 = test_event_queue.get_event().unwrap();
        assert_eq!(0, test_event_queue.queue.len());
        assert_eq!(event_logged2, event_gotten2);

        let event_empty = test_event_queue.get_event();
        assert!(event_empty.is_none());
    }

    #[test]
    fn t_event_fmt() {
        let event = Event {
            kind: EventKind::SnakeBorn {
                snake_id: 0,
                loc: Coordinates { x: 0.0, y: 0.0 },
                vel: Velocity {
                    delta_x: 0.0,
                    delta_y: 0.0,
                },
                size: 0.0,
            },
            event_time: 0.0,
        };
        assert_eq!("0.0: snake 0 born at (0.0,0.0) headed <0.0,0.0> of size 0.0", format!("{}", event));

        let event = Event {
            kind: EventKind::SnakeKilled { snake_id: 0 },
            event_time: 0.0,
        };
        assert_eq!("0.0: snake 0 killed", format!("{}", event));

        let event = Event {
            kind: EventKind::SnakeTurned {
                snake_id: 0,
                rad_relative: 1.0,
            },
            event_time: 0.0,
        };
        assert_eq!("0.0: snake 0 turned +1.000 rad", format!("{}", event));

        let event = Event {
            kind: EventKind::SnakeAte {
                snake_id: 0,
                food_id: 0,
            },
            event_time: 0.0,
        };
        assert_eq!("0.0: snake 0 ate food 0", format!("{}", event));

        let event = Event {
            kind: EventKind::FoodAdded {
                food_id: 0,
                loc: Coordinates { x: 0.0, y: 0.0 },
                nutrition: 0.0,
            },
            event_time: 0.0,
        };
        assert_eq!("0.0: food 0 added at (0.0,0.0) of nutrition 0.0", format!("{}", event));
    }
}
