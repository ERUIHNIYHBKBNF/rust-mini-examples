mod car_factory {
    pub fn build_car() {
        println!("Honk honk!");
    }
}

pub fn main() {
    car_factory::build_car();
}