use crate::engine::{
    input::{Action::*, MouseA::*},
    world::tile::Tile,
    InputHandler,
    World,
};

use fps_counter::FPSCounter;
use gfx_device_gl::{CommandBuffer, Device, Factory, Resources};
use gfx_graphics::{GfxGraphics, Texture, TextureContext};
use graphics::{
    clear,
    glyph_cache::rusttype::GlyphCache,
    rectangle,
    text,
    Context,
    Line,
    Transformed,
};
use piston_window::{Event, OpenGL, PistonWindow, RenderArgs, Size, Window};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sdl2_window::Sdl2Window;
use std::path::PathBuf;

pub struct App {
    pub title:          String,
    pub opengl:         OpenGL,
    pub fps:            FPSCounter,
    pub ups:            f64,
    pub focus:          [f64; 4],
    pub capture_cursor: bool,
    pub assets:         PathBuf,
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
        self.w = width;
        self.h = height;
        self.ar = width / height;
    }

    pub fn tick(&mut self) -> usize { self.fps.tick() }

    pub fn size(
        &mut self,
        size: f64,
    ) {
        const FACTOR: f64 = 1.3;
        self.size.1 = size * self.size.0.abs().ln_1p().exp() * FACTOR;
    }

    fn _on_screen(
        &self,
        size: f64,
    ) {
        let _a: Vec<(u16, u16)> = self
            .world
            .tiles
            .par_iter()
            .filter(|&a| {
                (a.x as f64 * size > self.focus[0] &&
                    a.y as f64 * size > self.focus[1]) &&
                    (a.x as f64 * size < self.focus[0] + self.w &&
                        a.y as f64 * size < self.focus[1] + self.h)
            })
            .map(|a| a.xy())
            .collect();
    }

    pub fn draw(
        &mut self,
        c: &Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
        _device: &mut Device,
        glyphs: &mut GlyphCache<
            'static,
            TextureContext<Factory, Resources, CommandBuffer>,
            Texture<Resources>,
        >,
    ) {
        clear([0.0, 0.0, 0.0, 1.0], g);
        self.draw_tiles(c, g);
        if self.stats {
            self.stats(c, g, glyphs);
        }
    }

    pub fn draw_tiles(
        &mut self,
        c: &Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    ) {
        let size = self.size.0;
        let transform = c.transform.trans(self.focus[0], self.focus[1]);

        let con = Line::new([1., 1., 1., 1.], 0.5);
        let mut loc = [0f64; 4];
        self.world
            .tiles
            .iter()
            .filter(|&t| {
                t.on_screen(self.w, self.h, self.focus[0], self.focus[1], size)
            })
            .for_each(|t| {
                let rect =
                    rectangle::square(t.x as f64 * size, t.y as f64 * size, size);
                rectangle([1., 0., 0.2, 1.], rect, transform, g);
                loc[2] = size * t.x as f64;
                loc[3] = size * t.y as f64;
                if ((loc[0].powf(2.) + loc[1].powf(2.)) -
                    (loc[2].powf(2.) + loc[3].powf(2.)))
                .abs() >
                    size.powf(2.) * 2.
                {
                    con.draw(loc, &c.draw_state, transform, g);
                }
                loc.rotate_left(2);
            });

        let cell_edge = Line::new([1., 1., 1., 1.], 1.);
        let x = 1.;
        let x2 = size * u16::MAX as f64;
        cell_edge.draw([x, x, x, x2], &c.draw_state, transform, g);
        cell_edge.draw([x, x, x2, x], &c.draw_state, transform, g);
        cell_edge.draw([x, x2, x2, x2], &c.draw_state, transform, g);
        cell_edge.draw([x2, x, x2, x2], &c.draw_state, transform, g);
        // TODO: Fix lower borders scaling with tile size.
    }

    pub fn render(
        &mut self,
        _args: &RenderArgs,
    ) {
    }

    pub fn update(&mut self) {
        let step = 4. * self.size.1 / self.ups;
        self.size.0 = if self.size.0 + step > 1. {
            self.size.0 + step
        } else {
            1.
        };
        self.size.1 = if step.abs() > 4. / self.ups {
            self.size.1 - step
        } else {
            0.
        };
        self.world.update()
    }

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
                // self.size = (1., 0.);
                if self.input.repeat() {
                    self.focus = [
                        -(u16::MAX as f64) * self.size.0 + self.w,
                        -(u16::MAX as f64) * self.size.0 + self.h,
                        0.0,
                        0.0,
                    ];
                } else {
                    self.focus = [0.0, 0.0, 0.0, 0.0];
                }
            }
        };

        #[allow(unused_variables)]
        for button in self.input.mouse() {
            match button {
                LMB(x, y) => self.world.put(self.get_pos(x, y)),
                RMB(x, y) => self.world.remove(&self.get_pos(x, y)),
                MMB(x, y) => self.world.end(),
                Drag(x1, y1, x2, y2) => {}
            }
        }
    }

    fn get_pos(
        &self,
        x: &f64,
        y: &f64,
    ) -> Tile {
        let tile = Tile::new(
            ((x - self.focus[0]) / self.size.0) as T,
            ((y - self.focus[1]) / self.size.0) as T,
        );
        tile
    }

    fn stats<'a>(
        &mut self,
        c: &Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
        glyphs: &mut GlyphCache<
            'static,
            TextureContext<Factory, Resources, CommandBuffer>,
            Texture<Resources>,
        >,
    ) {
        let fps = &self.tick();
        text::Text::new_color([0.6, 0.6, 0.6, 0.6], 20)
            .draw(
                &fps.to_string(),
                glyphs,
                &c.draw_state,
                c.transform.trans(self.w - 34., 17.0),
                g,
            )
            .unwrap();
        text::Text::new_color([0.6, 0.6, 0.6, 0.6], 20)
            .draw(
                &(self.ups as u32).to_string(),
                glyphs,
                &c.draw_state,
                c.transform.trans(self.w - 34., 36.0),
                g,
            )
            .unwrap();
    }

    pub fn exit(&mut self) {
        self.world.end();
        self.world.test();
    }
}
