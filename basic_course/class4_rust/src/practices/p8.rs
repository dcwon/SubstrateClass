trait Timable {
    fn get_time(&self) -> u8;
}

enum TrafficLight {
    Green(u8),
    Red(u8),
    Yellow(u8)
}

impl Timable for TrafficLight {
    fn get_time(&self) -> u8 {
        match *self {
            Self::Green(t) => t,
            Self::Red(t) => t,
            Self::Yellow(t) => t,
        }
    }
}

pub fn test_p8() {
    println!("\n############ Practice 8 Start! ############\n");
    let tl_green = TrafficLight::Green(60);
    let tl_red = TrafficLight::Red(100);
    let tl_yellow = TrafficLight::Yellow(3);
    println!("绿灯持续时间是{}秒， 红灯持续时间是{}秒，黄灯持续时间是{}秒",
            tl_green.get_time(), tl_red.get_time(), tl_yellow.get_time());
    println!("\n############ Practice 8 End! ############");
}