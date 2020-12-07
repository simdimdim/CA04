// use std::{cell::RefCell, path::PathBuf, rc::Rc};

use std::rc::Rc;

use ca04::engine::{world::block::Block, World};
// use fps_counter::FPSCounter;
// use gfx_device_gl::{CommandBuffer, Factory, Resources};
// use gfx_graphics::{Texture, TextureContext};
// use piston_window::{
//     clear,
//     glyph_cache::rusttype::GlyphCache,
//     text,
//     AdvancedWindow,
//     Button,
//     Event,
//     EventLoop,
//     IdleEvent,
//     Key,
//     MouseButton,
//     MouseCursorEvent,
//     MouseScrollEvent,
//     OpenGL,
//     PistonWindow,
//     PressEvent,
//     ResizeEvent,
//     Size,
//     Transformed,
//     UpdateEvent,
//     Window,
//     WindowSettings,
// };
// use sdl2_window::Sdl2Window;
extern crate find_folder;

fn main() {
    // let mut app = init();
    // let window = app.window();
    // let glyphs = app.glyphs();
    let mut world = World::new();

    let mut a = Block {
        coords:  Rc::new((0, 0)),
        types:   [0; 64],
        values:  [0; 64],
        members: 7,
    };
    a.types[0..7].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7]);
    a.values[0..7].copy_from_slice(&[10, 10, 10, 10, 10, 11, 17]);
    let mut b = a.clone();
    b.types[0..7].copy_from_slice(&[1, 2, 3, 4, 7, 0, 0]);
    b.values[0..7].copy_from_slice(&[2, 20, 10, 7, 2, 0, 0]);
    b.members = 5;

    world.insert_mut(a.clone());
    world.insert_mut(b.clone());
    // println!("{:?}", world.tiles);
    let mut c = a.clone();
    println!("{} {:?}", c.members, c);
    println!("{} {:?}", b.members, b);
    c += b.clone();
    b.types[0..7].copy_from_slice(&[1, 2, 3, 4, 7, 0, 0]);
    b.values[0..7].copy_from_slice(&[2, 20, 10, 7, 2, 0, 0]);
    println!("{} {:?}", c.members, c);
    println!("{} {:?}", b.members, b);

    // while let Some(e) = window.next() {
    //     window.draw_2d(&e, |c, g, device| {
    //         clear([0.0, 0.0, 0.0, 1.0], g);
    //         // world.draw(world.grid.size() as f64, dim.clone(), c, g);
    //         // let d = &app.dim.borrow();
    //         // if app.stats {
    //         //     text::Text::new_color([0.6, 0.6, 0.6, 0.6], 20)
    //         //         .draw(
    //         //             &app.fps.tick().to_string(),
    //         //             &mut app.glyphs,
    //         //             &c.draw_state,
    //         //             c.transform.trans(d.0 - 34., 17.0),
    //         //             g,
    //         //         )
    //         //         .unwrap();
    //         //     text::Text::new_color([0.6, 0.6, 0.6, 0.6], 20)
    //         //         .draw(
    //         //             &(app.ups as u32).to_string(),
    //         //             &mut app.glyphs,
    //         //             &c.draw_state,
    //         //             c.transform.trans(d.0 - 34., 36.0),
    //         //             g,
    //         //         )
    //         //         .unwrap();
    //         // }
    //         // Update glyphs before rendering.
    //         glyphs.factory.encoder.flush(device);
    //     });
    //     e.mouse_cursor(|pos| {
    //         app.cursor = pos;
    //     });
    //     e.mouse_scroll(|d| {
    //         // &world.grid.set_size(&world.grid.size() + 2 * d[1] as i32);
    //     });
    //     if let Some(Button::Mouse(button)) = e.press_args() {
    //         if button == MouseButton::Left {
    //             // world.add(Grid::get_pos(&world.grid, cursor[0],
    // cursor[1]));         }
    //         if button == MouseButton::Right {
    //             // world.remove(world.grid.get_pos(cursor[0], cursor[1]));
    //         }
    //     }
    //     if let Some(button) = e.press_args() {
    //         use piston_window::Button::Keyboard;

    //         if button == Keyboard(Key::W) {
    //             // &world.grid.toggle();
    //         } else if button == Keyboard(Key::R) {
    //             // &world.grid.set_size(20);
    //             app.toggle_stats();
    //         } else if button == Keyboard(Key::S) {
    //             app.toggle_stats();
    //         }
    //     }
    //     if let Some(_) = e.resize_args() {
    //         app.resize();
    //         // &world.grid.set_ar(ar);
    //     }
    //     if let Some(_args) = e.idle_args() {
    //         // println!("{}", args.dt);
    //     }
    //     if let Some(args) = e.update_args() {
    //         app.ups = 1. / args.dt;
    //         // println!("{}", args.dt);
    //         // println!("{}", fps.tick());
    //         // world.update();
    //     }
    // }
}
// struct App {
//     pub title:          String,
//     pub opengl:         OpenGL,
//     pub fps:            FPSCounter,
//     pub ups:            f64,
//     pub cursor:         [f64; 2],
//     pub capture_cursor: bool,
//     pub assets:         PathBuf,
//     window:             PistonWindow<Sdl2Window>,
//     glyphs: GlyphCache<
//         'static,
//         TextureContext<Factory, Resources, CommandBuffer>,
//         Texture<Resources>,
//     >,
//     dim:                Rc<RefCell<(f64, f64)>>,
//     ar:                 f64,
//     stats:              bool,
// }
// fn init() -> App {
//     let title = "CA04".to_string();
//     let opengl = OpenGL::V4_5;
//     let fps = fps_counter::FPSCounter::new();
//     let ups = 60.0;
//     let capture_cursor = false;
//     let cursor = [0.0, 0.0];
//     let assets = find_folder::Search::ParentsThenKids(3, 3)
//         .for_folder("assets")
//         .unwrap();

//     let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(&title,
// [800, 600])         .exit_on_esc(true)
//         // .samples(16)
//         .graphics_api(opengl)
//         .build()
//         .unwrap();
//     window.set_capture_cursor(capture_cursor);
//     window.set_max_fps((ups * 2.) as u64);
//     window.set_ups(ups as u64);
//     let mut glyphs = window
//         .load_font(assets.join("FiraSans-Regular.ttf"))
//         .unwrap();
//     let dim = Rc::new(RefCell::new({
//         let Size { width, height } = window.window.draw_size();
//         (width, height)
//     }));
//     let ar = dim.borrow().0 / dim.borrow().1;
//     let stats = false;

//     App {
//         title,
//         opengl,
//         fps,
//         ups,
//         cursor,
//         capture_cursor,
//         assets,
//         window,
//         glyphs,
//         dim,
//         ar,
//         stats,
//     }
// }
// impl App {
//     fn toggle_stats(&mut self) { self.stats = !self.stats; }

//     fn resize(&mut self) {
//         let Size { width, height } = self.window.window.draw_size();
//         self.dim.replace((width, height));
//         self.ar = width / height;
//     }

//     pub fn tick(&mut self) { self.fps.tick(); }

//     pub fn window<'a>(&'a mut self) -> &'a mut PistonWindow<Sdl2Window> {
//         &mut self.window
//     }

//     pub fn glyphs<'b>(
//         &'b mut self
//     ) -> &'b mut GlyphCache<
//         'static,
//         TextureContext<Factory, Resources, CommandBuffer>,
//         Texture<Resources>,
//     > {
//         &mut self.glyphs
//     }
// }
