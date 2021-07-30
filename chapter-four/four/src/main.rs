#![allow(unused)]

use std::error::Error;
fn main() {
    // 1. 交通信号灯
    let red_tl = TrafficLight::Red;
    print!("read traffic light time: {}\n", red_tl.time());

    let green_tl = TrafficLight::Green;
    print!("green traffic light time: {}\n", green_tl.time());

    // 2. 计算 u32 集合的和，结果溢出u32返回None
    // normal
    let mut v_normal: Vec<u32> = Vec::new();
    v_normal.push(1);
    v_normal.push(1);
    match my_sum(&v_normal) {
        None => println!("Vec sum: overflow"),
        _ => print!("Vec sum: {}\n", my_sum(&v_normal).unwrap()),
    };

    // overflow
    let mut v_overflow: Vec<u32> = Vec::new();
    v_overflow.push(1);
    v_overflow.push(u32::max_value());
    match my_sum(&v_overflow) {
        None => println!("Vec sum: overflow"),
        _ => print!("Vec sum: {}\n", my_sum(&&v_overflow).unwrap()),
    };

    // 3. 计算图形的面积
    // 长方形
    let rectangle = Rectangle {
        length: 1.0,
        height: 2.0,
    };
    print_area(&rectangle);
    // 三角形
    let triangle = Triangle {
        length: 1.0,
        height: 2.0,
    };
    print_area(&triangle);
    // 没有实现约束trait Area 的图形
    let no_area = NoArea {};
    // print_area(&no_area); // error[E0277]: the trait bound `NoArea: Area` is not satisfied
}

// 一、信号灯

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TLTime for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            &TrafficLight::Red => 100,
            &TrafficLight::Green => 90,
            &TrafficLight::Yellow => 3,
        }
    }
}

pub trait TLTime {
    fn time(&self) -> u8;
}

// 二、求和

fn my_sum(v: &Vec<u32>) -> Option<u32> {
    let mut sum: u32 = 0;
    for (i, &item) in v.iter().enumerate() {
        sum = match sum.checked_add(item) {
            None => return None,
            _ => sum.checked_add(item).unwrap(),
        };
    }

    Some(sum)
}

// 三、计算面积

struct Rectangle {
    length: f64,
    height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

struct Triangle {
    length: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.length * self.height / 2.0
    }
}

struct NoArea {}

trait Area {
    fn area(&self) -> f64;
}

fn print_area<T: Area>(t: &T) {
    print!("area: {}\n", t.area());
}
