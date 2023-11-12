use rand::Rng;
use raster::{Color, Image};

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color() -> Color;
}

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn random(max_x: i32, max_y: i32) -> Self {
        let x: i32 = rand::thread_rng().gen_range(0..max_x);
        let y: i32 = rand::thread_rng().gen_range(0..max_y);
        Self{x, y}
	}

    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image)  {
        let color = Self::color();
        image.display(self.x, self.y, color.clone());
    }
    fn color() -> Color {random_color()}
}

pub struct Line {
    p1: Point, 
    p2: Point
}

impl Line {
    pub fn random(max_x: i32, max_y: i32) -> Self {
        let x: i32 = rand::thread_rng().gen_range(0..max_x);
        let y: i32 = rand::thread_rng().gen_range(0..max_y);
        let x2: i32 = rand::thread_rng().gen_range(0..max_x);
        let y2: i32 = rand::thread_rng().gen_range(0..max_y);
        let p1 = Point{x: x, y: y};
        let p2 = Point{x: x2, y: y2};
        Self::new(&p1, &p2)
    }

    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            p1: p1.clone(), 
            p2: p2.clone()
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let color = Self::color();
        draw_line(image, self.p1.x, self.p1.y, self.p2.x, self.p2.y, color.clone());
    }
    fn color() -> Color {random_color()}
}

pub struct Circle {
    center: Point,
    radius: i32
}

impl Circle {
    pub fn random(max_x: i32, max_y: i32) -> Self {
        let x: i32 = rand::thread_rng().gen_range(0..max_x);
        let y: i32 = rand::thread_rng().gen_range(0..max_y);
        let accx: i32;
        let accy: i32;
        let max_r: i32;
        if max_x - x > 500 {
            accx = x;
        } else {
            accx = max_x - x;
        }
        if max_y - y > 500 {
            accy = y;
        } else {
            accy = max_y - y;
        }
        if accx < accy {
            max_r = accx;
        } else {
            max_r = accy;
        }
        let radius: i32 = rand::thread_rng().gen_range(0..max_r);
        let p = Point{x: x, y: y};
        Self::new(&p, radius)
	}

    pub fn new(p: &Point, radius: i32) -> Self {
        Self {
            center: p.clone(),
            radius: radius
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = Self::color();
        let mut x: i32 = 0;
        let mut y: i32 = self.radius;
        let mut delta: i32 = 1 - 2 * self.radius;
        while y >= x {
            image.display(self.center.x + x, self.center.y + y, color.clone());
            image.display(self.center.x + x, self.center.y - y, color.clone());
            image.display(self.center.x - x, self.center.y + y, color.clone());
            image.display(self.center.x - x, self.center.y - y, color.clone());
            image.display(self.center.x + y, self.center.y + x, color.clone());
            image.display(self.center.x + y, self.center.y - x, color.clone());
            image.display(self.center.x - y, self.center.y + x, color.clone());
            image.display(self.center.x - y, self.center.y - x, color.clone());
            let error = 2 * (delta + y) - 1;
            if delta < 0 && error <= 0 {
                x += 1;
                delta += 2 * x + 1;
                continue;
            }
            if delta > 0 && error > 0 {
                y -= 1;
                delta -= 2 * y + 1;
                continue;
            }
            x += 1;
            y -= 1;
            delta += 2 * (x - y)
        }
    }
    fn color() -> Color {random_color()}
}

pub struct Rectangle {
    p1: Point,
    p2: Point
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone()
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let color = Self::color();
        let max_x: i32 = std::cmp::max(self.p1.x, self.p2.x);
        let max_y: i32 = std::cmp::max(self.p1.y, self.p2.y);
        let min_x: i32 = std::cmp::min(self.p1.x, self.p2.x);
        let min_y: i32 = std::cmp::min(self.p1.y, self.p2.y);
        draw_line(image, min_x, min_y, min_x, max_y, color.clone());
        draw_line(image, min_x, max_y, max_x, max_y, color.clone());
        draw_line(image, max_x, max_y, max_x, min_y, color.clone());
        draw_line(image, max_x, min_y, min_x, min_y, color.clone());
    }
    fn color() -> Color {random_color()}
}

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
            p3: p3.clone()
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = Self::color();
        draw_line(image, self.p1.x, self.p1.y, self.p2.x, self.p2.y, color.clone());
        draw_line(image, self.p2.x, self.p2.y, self.p3.x, self.p3.y, color.clone());
        draw_line(image, self.p3.x, self.p3.y, self.p1.x, self.p1.y, color.clone());
    }
    fn color() -> Color {random_color()}
}

fn draw_line(image: &mut Image, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
let mut x0 = x0;
let mut y0 = y0;
let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };
let sx = if x0 < x1 { 1 } else { -1 };
let sy = if y0 < y1 { 1 } else { -1 };
let mut err = if dx > dy { dx } else {-dy} / 2;
let mut err2;

loop {
    image.display(x0, y0, color.clone());
    if x0 == x1 && y0 == y1 { break };
    err2 = 2 * err;
    if err2 > -dx { err -= dy; x0 += sx; }
    if err2 < dy { err += dx; y0 += sy; }
}
}

fn random_color() -> Color {
    let r = rand::thread_rng().gen_range(20..=255);
    let g = rand::thread_rng().gen_range(20..=255);
    let b = rand::thread_rng().gen_range(20..=255);
    Color::rgb(r, g, b)
}