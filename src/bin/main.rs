// use std::{cell::RefCell, path::PathBuf, rc::Rc};

use ca04::{
    engine::{InputHandler, World},
    App,
};
use piston_window::{
    AdvancedWindow,
    EventLoop,
    IdleEvent,
    MouseScrollEvent,
    OpenGL,
    PistonWindow,
    RenderEvent,
    ResizeEvent,
    UpdateEvent,
    WindowSettings,
};
use sdl2_window::Sdl2Window;
extern crate find_folder;

fn main() {
    let title = "CA04";
    let opengl = OpenGL::V4_5;
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new(title, [800, 600])
            .exit_on_esc(true)
            .samples(16)
            .graphics_api(opengl)
            .build()
            .unwrap();
    let mut app = init(title, opengl, &mut window);
    window.set_capture_cursor(app.capture_cursor);
    window.set_max_fps((app.ups * 2.) as u64);
    window.set_ups(app.ups as u64);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            app.draw(&c, g, device);
        });
        app.event(&e);

        e.mouse_scroll(|d| {
            app.size(d[1]);
            // app.focus[2] =
            //     app.focus[2] - d[1] * (2. * app.input.cursor[0] - app.w) /
            // 10.; app.focus[3] =
            //     app.focus[3] - d[1] * (2. * app.input.cursor[1] - app.h) /
            // 10.;
        });

        if let Some(_) = e.resize_args() {
            app.resize(&window);
        }
        if let Some(_args) = e.idle_args() {
            // println!("{}", args.dt);
        }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(_args) = e.update_args() {
            // app.ups = 1. / args.dt; // what is this???
            // println!("{}", args.dt);
            // println!("{}", fps.tick());
            // println!("{:?}", world.tiles);
            app.update();
        }
    }
}

fn init(
    title: &str,
    opengl: OpenGL,
    window: &mut PistonWindow<Sdl2Window>,
) -> App {
    let title = title.to_string();
    let fps = fps_counter::FPSCounter::new();
    let ups = 120.0;
    let capture_cursor = false;
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();
    let stats = false;
    // let dim = Rc::new(RefCell::new((0., 0.)));
    let focus = [0.0; 4];
    let w = 0.0;
    let h = 0.0;
    let ar = 0.0;
    let mut world = World::new();
    let mut input = InputHandler::new();
    let size = (1., 0.);

    input.populate();
    world.test();

    App {
        title,
        opengl,
        fps,
        ups,
        capture_cursor,
        assets,
        glyphs,
        focus,
        w,
        h,
        ar,
        stats,
        world,
        input,
        size,
    }
}
