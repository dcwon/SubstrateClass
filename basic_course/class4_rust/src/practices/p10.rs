use std::f64::consts::PI;

trait Graphics {
    fn get_name(&self) -> String;
    fn get_area(&self) -> f64;
}

struct Circle {
    name: String,
    radius: f64
}

struct Triangle {
    name: String,
    hight: f64,
    bottom: f64
}

struct Rectangle {
    name: String,
    length: f64,
    width: f64
}

impl Graphics for Circle {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_area(&self) -> f64 {
        PI * self.radius.sqrt()
    }
}

impl Graphics for Triangle {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_area(&self) -> f64 {
        self.bottom * self.hight * 0.5
    }
}

impl Graphics for Rectangle {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_area(&self) -> f64 {
        self.length * self.width
    }
}

fn print_area<T: Graphics>(gra: T) {
    println!("{}的面积是{}", gra.get_name(), gra.get_area());
}

pub fn test_p10() {
    println!("\n############ Practice 10 Start! ############\n");
    let circle = Circle {
        name: String::from("Circle 1"),
        radius: 1 as f64
    };
    let rectangle = Rectangle {
        name: String::from("Rectangle 1"),
        length: 2_f64,
        width: 3_f64
    };
    let triangle = Triangle {
        name: String::from("Triangle 1"),
        hight: 2_f64,
        bottom: 3_f64
    };
    print_area(circle);
    print_area(rectangle);
    print_area(triangle);
    println!("\n############ Practice 10 End! ############\n");
}