use std::path::PathBuf;

use fps_counter::FPSCounter;
use gfx_device_gl::{CommandBuffer, Device, Factory, Resources};
use gfx_graphics::{GfxGraphics, Texture, TextureContext};
use graphics::{
    clear,
    glyph_cache::rusttype::GlyphCache,
    rectangle,
    text,
    Context,
    Transformed,
};
use piston_window::{Event, OpenGL, PistonWindow, RenderArgs, Size, Window};
use sdl2_window::Sdl2Window;

use crate::engine::{
    input::{Action::*, MouseA::*},
    InputHandler,
    World,
};

pub struct App {
    pub title:          String,
    pub opengl:         OpenGL,
    pub fps:            FPSCounter,
    pub ups:            f64,
    pub focus:          [f64; 4],
    pub capture_cursor: bool,
    pub assets:         PathBuf,
    pub glyphs: GlyphCache<
        'static,
        TextureContext<Factory, Resources, CommandBuffer>,
        Texture<Resources>,
    >,
    // dim:                Rc<RefCell<(f64, f64)>>,
    pub w:              f64,
    pub h:              f64,
    pub ar:             f64,
    pub stats:          bool,
    pub world:          World,
    pub input:          InputHandler,
    pub size:           (f64, f64),
}
type T = u16;
impl App {
    pub fn toggle_stats(&mut self) { self.stats = !self.stats; }

    pub fn resize(
        &mut self,
        window: &PistonWindow<Sdl2Window>,
    ) {
        let Size { width, height } = window.window.draw_size();
        // self.dim.replace((w, h));
        self.w = width;
        self.h = height;
        self.ar = width / height;
    }

    pub fn tick(&mut self) -> usize { self.fps.tick() }

    pub fn size(
        &mut self,
        size: f64,
    ) {
        const FACTOR: f64 = 2.3;
        self.size.1 = size * self.size.0.abs().ln_1p().exp() * FACTOR;
    }

    pub fn draw_tiles(
        &mut self,
        c: &Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    ) {
        use graphics::Line;

        let size = if self.size.0 > 0.1 { self.size.0 } else { 0.1 };
        let transform = c.transform.trans(self.focus[0], self.focus[1]);
        for b in self.world.tiles.values() {
            let square =
                rectangle::square(b.x as f64 * size, b.y as f64 * size, size);
            rectangle(
                [
                    (7 * b.members % 16) as f32 / 16.,
                    (5 * b.members % 16) as f32 / 16.,
                    (9 * b.members % 16) as f32 / 16.,
                    1.,
                ],
                square,
                transform,
                g,
            );
        }

        let cell_edge = Line::new([1., 1., 1., 1.], size);
        let x = -1.;
        let x2 = u16::MAX as f64 * size;
        cell_edge.draw([x, x, x, x2], &c.draw_state, transform, g);
        cell_edge.draw([x, x, x2, x], &c.draw_state, transform, g);
        cell_edge.draw([x, x2, x2, x2], &c.draw_state, transform, g);
        cell_edge.draw([x2, x, x2, x2], &c.draw_state, transform, g);
    }

    pub fn draw(
        &mut self,
        c: &Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
        device: &mut Device,
    ) {
        self.draw_tiles(c, g);

        clear([0.0, 0.0, 0.0, 1.0], g);
        if self.stats {
            let fps = &self.tick();
            // let d = &self.dim.borrow();
            text::Text::new_color([0.6, 0.6, 0.6, 0.6], 20)
                .draw(
                    &fps.to_string(),
                    &mut self.glyphs,
                    &c.draw_state,
                    // c.transform.trans(d.0 - 34., 17.0),
                    c.transform.trans(self.w - 34., 17.0),
                    g,
                )
                .unwrap();
            text::Text::new_color([0.6, 0.6, 0.6, 0.6], 20)
                .draw(
                    &(self.ups as u32).to_string(),
                    &mut self.glyphs,
                    &c.draw_state,
                    // c.transform.trans(d.0 - 34., 36.0),
                    c.transform.trans(self.w - 34., 36.0),
                    g,
                )
                .unwrap();
            // Update glyphs before rendering.
            self.glyphs.factory.encoder.flush(device);
        }
    }

    pub fn render(
        &mut self,
        _args: &RenderArgs,
    ) {
    }

    pub fn update(&mut self) {
        let step = 4. * self.size.1 / self.ups;
        self.size.0 = if self.size.0 + step > 0.5 {
            self.size.0 + step
        } else {
            0.5
        };
        self.size.1 = if step.abs() > 4. / self.ups {
            self.size.1 - step
        } else {
            0.
        };
        self.world.update()
    }

    /*pub fn glyphs<'b>(
        &'b mut self
    ) -> &'b mut GlyphCache<
        'static,
        TextureContext<Factory, Resources, CommandBuffer>,
        Texture<Resources>,
    > {
        &mut self.glyphs
    }*/
    pub fn event(
        &mut self,
        e: &Event,
    ) {
        match self.input.event(e) {
            Pass => {}
            Exit => self.exit(),
            Stats => self.stats = self.input.repeat(),
            N => self.focus[1] = self.focus[1] + self.size.0,
            NE => {
                self.focus[1] = self.focus[1] + self.size.0;
                self.focus[0] = self.focus[0] - self.size.0;
            }
            E => self.focus[0] = self.focus[0] - self.size.0,
            SE => {
                self.focus[1] = self.focus[1] - self.size.0;
                self.focus[0] = self.focus[0] - self.size.0;
            }
            S => self.focus[1] = self.focus[1] - self.size.0,
            SW => {
                self.focus[1] = self.focus[1] - self.size.0;
                self.focus[0] = self.focus[0] + self.size.0;
            }
            W => self.focus[0] = self.focus[0] + self.size.0,
            NW => {
                self.focus[1] = self.focus[1] + self.size.0;
                self.focus[0] = self.focus[0] + self.size.0;
            }
            ResetZoom => {
                self.size = (1., 0.);
                self.focus = [
                    -(u16::MAX as f64) + self.w,
                    -(u16::MAX as f64) + self.h,
                    0.0,
                    0.0,
                ];
            }
        };

        #[allow(unused_variables)]
        for button in self.input.mouse() {
            match button {
                LMB(x, y) => self.world.put(self.get_pos(x, y)),
                RMB(x, y) => self.world.remove(&self.get_pos(x, y)),
                MMB(x, y) => {}
                Drag(x1, y1, x2, y2) => {}
            }
        }
    }

    fn get_pos(
        &self,
        x: &f64,
        y: &f64,
    ) -> (T, T) {
        //TODO: This
        let pos = (
            ((-self.focus[0] + x) / self.size.0) as T,
            ((-self.focus[1] + y) / self.size.0) as T,
        );
        println!("{:?}", self.focus);
        println!("{:?}", pos);
        pos
    }

    pub fn exit(&mut self) { self.world.end(); }
}
