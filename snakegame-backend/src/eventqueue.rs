pub mod eventqueue {

    pub enum EventType {
        SnakeBorn {snakeid: usize, }
    }
    pub struct Event {
        pub start_loc: Vec<f64>,
        pub end_loc: Vec<f64>,
        pub width: f64,
    }

}
