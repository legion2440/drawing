use rand::Rng;
use raster::Color;
use std::f64::consts::{FRAC_PI_2, TAU};

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw<T: Displayable>(&self, image: &mut T);
    fn color(&self) -> Color;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        assert!(width > 0 && height > 0);

        let mut rng = rand::thread_rng();
        Self::new(rng.gen_range(0..width), rng.gen_range(0..height))
    }
}

impl Drawable for Point {
    fn draw<T: Displayable>(&self, image: &mut T) {
        let color = self.color();
        for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
            display_offset(image, *self, dx, dy, color.clone());
        }
    }

    fn color(&self) -> Color {
        Color::rgb(255, 215, 0)
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Self {
            start: *start,
            end: *end,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        assert!(width > 0 && height > 0);
        Self::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw<T: Displayable>(&self, image: &mut T) {
        draw_line(image, self.start, self.end, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(0, 220, 255)
    }
}

pub struct Triangle {
    vertices: [Point; 3],
}

impl Triangle {
    pub fn new(first: &Point, second: &Point, third: &Point) -> Self {
        Self {
            vertices: [*first, *second, *third],
        }
    }
}

impl Drawable for Triangle {
    fn draw<T: Displayable>(&self, image: &mut T) {
        draw_polygon(image, &self.vertices, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(255, 80, 120)
    }
}

pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(first: &Point, second: &Point) -> Self {
        Self {
            top_left: Point::new(first.x.min(second.x), first.y.min(second.y)),
            bottom_right: Point::new(first.x.max(second.x), first.y.max(second.y)),
        }
    }
}

impl Drawable for Rectangle {
    fn draw<T: Displayable>(&self, image: &mut T) {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);
        let vertices = [self.top_left, top_right, self.bottom_right, bottom_left];
        draw_polygon(image, &vertices, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(240, 240, 240)
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
    color: Color,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        assert!(radius >= 0);
        Self {
            center: *center,
            radius,
            color: Color::rgb(255, 140, 0),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        assert!(width > 0 && height > 0);

        let mut rng = rand::thread_rng();
        Self {
            center: Point::random(width, height),
            radius: rng.gen_range(1..=width.max(height)),
            color: random_bright_color(&mut rng),
        }
    }
}

impl Drawable for Circle {
    fn draw<T: Displayable>(&self, image: &mut T) {
        let mut x = i64::from(self.radius);
        let mut y = 0_i64;
        let mut decision = 1 - x;

        while x >= y {
            draw_circle_octants(image, self.center, x, y, self.color());
            y += 1;
            if decision <= 0 {
                decision += 2 * y + 1;
            } else {
                x -= 1;
                decision += 2 * (y - x) + 1;
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

pub struct Pentagon {
    vertices: [Point; 5],
}

impl Pentagon {
    pub fn new(center: &Point, radius: i32) -> Self {
        assert!(radius >= 0);

        let mut vertices = [*center; 5];
        for (index, vertex) in vertices.iter_mut().enumerate() {
            let angle = -FRAC_PI_2 + index as f64 * TAU / 5.0;
            *vertex = Point::new(
                (f64::from(center.x) + f64::from(radius) * angle.cos()).round() as i32,
                (f64::from(center.y) + f64::from(radius) * angle.sin()).round() as i32,
            );
        }

        Self { vertices }
    }
}

impl Drawable for Pentagon {
    fn draw<T: Displayable>(&self, image: &mut T) {
        draw_polygon(image, &self.vertices, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(80, 255, 120)
    }
}

pub struct Cube {
    front: [Point; 4],
    back: [Point; 4],
}

impl Cube {
    pub fn new(top_left: &Point, size: i32, offset: i32) -> Self {
        assert!(size >= 0);

        let right = top_left.x.saturating_add(size);
        let bottom = top_left.y.saturating_add(size);
        let back_left = top_left.x.saturating_add(offset);
        let back_top = top_left.y.saturating_add(offset);
        let back_right = back_left.saturating_add(size);
        let back_bottom = back_top.saturating_add(size);

        Self {
            front: [
                *top_left,
                Point::new(right, top_left.y),
                Point::new(right, bottom),
                Point::new(top_left.x, bottom),
            ],
            back: [
                Point::new(back_left, back_top),
                Point::new(back_right, back_top),
                Point::new(back_right, back_bottom),
                Point::new(back_left, back_bottom),
            ],
        }
    }
}

impl Drawable for Cube {
    fn draw<T: Displayable>(&self, image: &mut T) {
        let color = self.color();
        draw_polygon(image, &self.front, color.clone());
        draw_polygon(image, &self.back, color.clone());

        for (front_vertex, back_vertex) in self.front.iter().zip(self.back.iter()) {
            draw_line(image, *front_vertex, *back_vertex, color.clone());
        }
    }

    fn color(&self) -> Color {
        Color::rgb(190, 100, 255)
    }
}

fn draw_line<T: Displayable>(image: &mut T, start: Point, end: Point, color: Color) {
    let mut x = i64::from(start.x);
    let mut y = i64::from(start.y);
    let end_x = i64::from(end.x);
    let end_y = i64::from(end.y);
    let dx = (end_x - x).abs();
    let direction_x = if x < end_x { 1 } else { -1 };
    let dy = -(end_y - y).abs();
    let direction_y = if y < end_y { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        image.display(x as i32, y as i32, color.clone());
        if x == end_x && y == end_y {
            break;
        }

        let doubled_error = 2 * error;
        if doubled_error >= dy {
            error += dy;
            x += direction_x;
        }
        if doubled_error <= dx {
            error += dx;
            y += direction_y;
        }
    }
}

fn draw_polygon<T: Displayable>(image: &mut T, vertices: &[Point], color: Color) {
    for edge in vertices.windows(2) {
        draw_line(image, edge[0], edge[1], color.clone());
    }

    if let (Some(first), Some(last)) = (vertices.first(), vertices.last()) {
        draw_line(image, *last, *first, color);
    }
}

fn display_offset<T: Displayable>(image: &mut T, center: Point, dx: i64, dy: i64, color: Color) {
    let x = i64::from(center.x) + dx;
    let y = i64::from(center.y) + dy;
    if let (Ok(x), Ok(y)) = (i32::try_from(x), i32::try_from(y)) {
        image.display(x, y, color);
    }
}

fn draw_circle_octants<T: Displayable>(image: &mut T, center: Point, x: i64, y: i64, color: Color) {
    for (dx, dy) in [
        (x, y),
        (y, x),
        (-y, x),
        (-x, y),
        (-x, -y),
        (-y, -x),
        (y, -x),
        (x, -y),
    ] {
        display_offset(image, center, dx, dy, color.clone());
    }
}

fn random_bright_color<R: Rng + ?Sized>(rng: &mut R) -> Color {
    let mut channels = [
        rng.gen_range(80..=255),
        rng.gen_range(80..=255),
        rng.gen_range(80..=255),
    ];
    channels[rng.gen_range(0..channels.len())] = rng.gen_range(200..=255);
    Color::rgb(channels[0], channels[1], channels[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockImage {
        pixels: Vec<(i32, i32, Color)>,
    }

    impl MockImage {
        fn contains(&self, x: i32, y: i32) -> bool {
            self.pixels
                .iter()
                .any(|(pixel_x, pixel_y, _)| *pixel_x == x && *pixel_y == y)
        }
    }

    impl Displayable for MockImage {
        fn display(&mut self, x: i32, y: i32, color: Color) {
            self.pixels.push((x, y, color));
        }
    }

    #[test]
    fn random_points_stay_inside_dimensions() {
        for _ in 0..500 {
            let point = Point::random(37, 19);
            assert!((0..37).contains(&point.x));
            assert!((0..19).contains(&point.y));
        }
    }

    #[test]
    fn random_line_points_stay_inside_dimensions() {
        for _ in 0..500 {
            let line = Line::random(41, 23);
            for point in [line.start, line.end] {
                assert!((0..41).contains(&point.x));
                assert!((0..23).contains(&point.y));
            }
        }
    }

    #[test]
    fn random_circle_has_valid_center_and_radius() {
        for _ in 0..500 {
            let circle = Circle::random(43, 29);
            assert!((0..43).contains(&circle.center.x));
            assert!((0..29).contains(&circle.center.y));
            assert!((1..=43).contains(&circle.radius));
        }
    }

    #[test]
    fn rectangle_normalizes_points_in_either_order() {
        let first = Point::new(20, 40);
        let second = Point::new(5, 10);
        let forward = Rectangle::new(&first, &second);
        let reverse = Rectangle::new(&second, &first);

        assert_eq!(forward.top_left, Point::new(5, 10));
        assert_eq!(forward.bottom_right, Point::new(20, 40));
        assert_eq!(forward.top_left, reverse.top_left);
        assert_eq!(forward.bottom_right, reverse.bottom_right);
    }

    #[test]
    fn degenerate_line_draws_its_only_coordinate() {
        let point = Point::new(7, 11);
        let line = Line::new(&point, &point);
        let mut image = MockImage::default();

        line.draw(&mut image);

        assert_eq!(image.pixels.len(), 1);
        assert!(image.contains(7, 11));
    }

    #[test]
    fn point_draws_a_five_pixel_cross() {
        let mut image = MockImage::default();

        Point::new(8, 9).draw(&mut image);

        assert_eq!(image.pixels.len(), 5);
        for coordinate in [(8, 9), (7, 9), (9, 9), (8, 8), (8, 10)] {
            assert!(image.contains(coordinate.0, coordinate.1));
        }
    }

    #[test]
    fn zero_radius_circle_draws_its_center() {
        let mut image = MockImage::default();

        Circle::new(&Point::new(4, 6), 0).draw(&mut image);

        assert!(image.contains(4, 6));
    }

    #[test]
    fn pentagon_draws_pixels() {
        let mut image = MockImage::default();

        Pentagon::new(&Point::new(50, 50), 20).draw(&mut image);

        assert!(!image.pixels.is_empty());
    }

    #[test]
    fn cube_draws_pixels() {
        let mut image = MockImage::default();

        Cube::new(&Point::new(10, 10), 20, 5).draw(&mut image);

        assert!(!image.pixels.is_empty());
    }

    #[test]
    #[should_panic]
    fn random_rejects_non_positive_dimensions() {
        Point::random(0, 10);
    }

    #[test]
    #[should_panic]
    fn circle_rejects_negative_radius() {
        Circle::new(&Point::new(0, 0), -1);
    }

    #[test]
    #[should_panic]
    fn pentagon_rejects_negative_radius() {
        Pentagon::new(&Point::new(0, 0), -1);
    }

    #[test]
    #[should_panic]
    fn cube_rejects_negative_size() {
        Cube::new(&Point::new(0, 0), -1, 5);
    }
}
