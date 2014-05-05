extern crate native;
extern crate rsfml;
extern crate quadtree;

use rsfml::graphics::{RenderWindow, Color, RectangleShape};
use rsfml::system::{Vector2f, Vector2i};
use rsfml::window::event;
use rsfml::window::keyboard;
use rsfml::window::{Close, ContextSettings, VideoMode};

use quadtree::{Bounded, Bounds, QuadTree};

pub struct Rect<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    sfml_shape: RectangleShape<'a>
}

impl<'a> Rect<'a> {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        let mut r = RectangleShape::new().expect("error creating rectangle");
        r.set_position(&Vector2f::new(x, y));
        r.set_size(&Vector2f::new(width, height));
        r.set_outline_color(&Color::new_RGB(255, 0, 0));
        r.set_outline_thickness(1.);
        Rect {x: x, y: y, width: width, height: height, sfml_shape: r}
    }

    fn draw(&self, w: &mut RenderWindow) {
        w.draw(&self.sfml_shape)
    }
}

impl<'a> std::fmt::Show for Rect<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f.buf, "Rect \\{ x: {}, y: {}, width: {}, height: {} \\}",
               self.x, self.y, self.width, self.height)
    }
}

impl<'a> Bounded for Rect<'a> {
    fn bounds(&self) -> Bounds {
        Bounds{ x: self.x, y: self.y, width: self.width, height: self.height }
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let window_size = 1024.0;
    let mut window = RenderWindow::new(VideoMode::new_init(window_size as uint, window_size as uint, 32),
                                       "quadtree demo",
                                       Close,
                                       &ContextSettings::default()).expect("error creating window");

    let mut rects = Vec::<Rect>::new();

    let max_depth = 7;
    for depth in range(1u, max_depth+1) {
        let rect_size = window_size / std::num::pow(2, depth) as f32 - 1.0;
        let rect_num = std::num::pow(2, depth);
        for i in range(0, rect_num) {
            for j in range(0, rect_num) {
                rects.push(Rect::new(rect_size * j as f32 + j as f32,
                                     rect_size * i as f32 + i as f32,
                                     rect_size, rect_size));
            }
        }
    }

    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::KeyPressed{ code: code, .. } => match code {
                    keyboard::Escape => window.close(),
                    _ => {}
                },
                _ => {}
            }
        }

        let mut qt = QuadTree::<Rect>::new(Bounds{ x: 0., y: 0., width: window_size, height: window_size });
        for rect in rects.iter() {
            qt.insert(rect);
        }

        let Vector2i{ x: mouse_x, y: mouse_y } = window.get_mouse_position();
        let mouse_rect = Rect::new(mouse_x as f32, mouse_y as f32, 0.9, 0.9);

        window.clear(&Color::new_RGB(255, 255, 255));
        for target_rect in qt.query(&mouse_rect) {
            target_rect.draw(&mut window);
        }
        window.display()
    }
}
