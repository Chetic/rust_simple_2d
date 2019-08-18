use piston::window::WindowSettings;
use glutin_window::{OpenGL, GlutinWindow};
use piston::event_loop::{Events, EventSettings};
use opengl_graphics::GlGraphics;
use piston::input::{RenderEvent, UpdateEvent, PressEvent, Key, Button};
use graphics::*;
use crate::entity::Entity;

mod entity;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Hello 2D", [640, 480])
        .graphics_api(opengl).exit_on_esc(true).build().unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    let mut entities: [Entity; 100] = [Entity::new(50.0, 50.0, 5.0); 100];

    let mut counter: f64 = 0.0;

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, gl| {
                clear([0.0, 0.0, 0.0, 1.0], gl);

                for (i, entity) in entities.iter().enumerate() {
                    let transform = c.transform.trans(entity.x, entity.y).rot_rad(entity.rotation).trans(-entity.size / 2.0, -entity.size / 2.0);
                    let r = i as f32 / entities.len() as f32;
                    let g = 0.5;
                    let b = 0.5;
                    rectangle([r, g, b, 1.0], rectangle::square(0.0, 0.0, entity.size), transform, gl);
                }
            });
        }

        if let Some(_) = e.update_args() {
            let timescale = 0.01;
            for (i, entity) in entities.iter_mut().enumerate() {
                entity.x = (counter - i as f64 * timescale).tan() * 100.0 + 225.0;
                entity.y = (counter - i as f64 * timescale).cos() * 100.0 + 225.0;
                entity.rotation += 0.05;
            }
            counter += timescale;
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::C => {
                    for entity in entities.iter_mut() {
                        entity.size += 1.0
                    }
                }
                _ => ()
            }
        }
    }
}
