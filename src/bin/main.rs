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
    Size,
    UpdateEvent,
    Window,
    WindowSettings,
};
use sdl2_window::Sdl2Window;

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
    let ref mut glyphs = window
        .load_font(app.assets.join("FiraSans-Regular.ttf"))
        .unwrap();
    window.set_capture_cursor(app.capture_cursor);
    window.set_max_fps((app.ups * 4.) as u64);
    window.set_ups(app.ups as u64);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            app.draw(&c, g, device, glyphs);
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
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
            // println!("{}", args.dt);
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
    let ups = 60.0;
    let capture_cursor = false;
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let stats = false;
    let focus = [0.0; 4];
    let Size {
        width: w,
        height: h,
    } = window.window.draw_size();
    let ar = w / h;
    let world = World::new();
    let mut input = InputHandler::new();
    let size = (1., 0.);

    input.load_keymap();

    App {
        title,
        opengl,
        fps,
        ups,
        capture_cursor,
        assets,
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
