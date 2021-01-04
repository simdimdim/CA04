use crate::engine::{
    input::{Action::*, MouseB::*, MouseM::*},
    world::Point,
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
        size: &f64,
    ) {
        const FACTOR: f64 = 1.3;
        self.size.1 = size * self.size.0.abs().ln_1p().exp() * FACTOR;
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
        let transform = c
            .transform
            .trans(self.focus[0] * size * 32., self.focus[1] * size * 32.);

        // let con = Line::new([1., 1., 1., 0.8], 0.5);
        // let mut loc = [0f64; 4];
        let camera = (
            (-self.focus[0]).trunc(),
            (-self.focus[1]).trunc(),
            ((self.w / size) / 31.).trunc(),
            ((self.h / size) / 31.).trunc(),
        );
        self.world
            .chunks
            .iter()
            .filter(|&(_, t)| t.on_screen(camera))
            .for_each(|(&p, chunk)| {
                chunk.tiles.iter().filter(|&t| t.members > 0).for_each(|t| {
                    let tile = (p * 32. + t.pos) * size;
                    let rect = rectangle::square(tile.0, tile.1, size);
                    rectangle(t.color(), rect, transform, g);
                    // loc[2] = tile.0;
                    // loc[3] = tile.1;
                    // con.draw(loc, &c.draw_state, transform, g);
                    // loc.rotate_left(2);
                })
            });

        let cell_edge = Line::new([1., 0.3, 0., 1.], 1.);
        const TOP: f64 = 0.;
        let x2 = (u16::MAX as f64 + 1.) * size * 32. - 1.;
        cell_edge.draw([TOP, TOP, TOP, x2], &c.draw_state, transform, g);
        cell_edge.draw([TOP, TOP, x2, TOP], &c.draw_state, transform, g);
        cell_edge.draw([TOP, x2, x2, x2], &c.draw_state, transform, g);
        cell_edge.draw([x2, TOP, x2, x2], &c.draw_state, transform, g);
        let chu = size * 32.;
        cell_edge.draw([TOP, TOP, TOP, chu], &c.draw_state, transform, g);
        cell_edge.draw([TOP, TOP, chu, TOP], &c.draw_state, transform, g);
        cell_edge.draw([TOP, chu, chu, chu], &c.draw_state, transform, g);
        cell_edge.draw([chu, TOP, chu, chu], &c.draw_state, transform, g);
    }

    pub fn update(&mut self) {
        let step = self.size.1 / self.ups;
        self.size.0 = if self.size.0 + step > 1. {
            self.size.0 + step
        } else {
            1.
        };
        self.size.1 = if step.abs() > self.ups.recip() {
            self.size.1 - step
        } else {
            0.
        };

        self.focus[0] += self.focus[2];
        self.focus[1] += self.focus[3];
        self.focus[2] -= self.focus[2];
        self.focus[3] -= self.focus[3];

        // dbg!(self.size);
        // let xrate = self.focus[2].abs().acos() / 10.;
        // let yrate = self.focus[3].abs().acos() / 10.;
        // if self.focus[2] != 0. {
        //     self.focus[0] += xrate.copysign(self.focus[2]);
        //     self.focus[2] = if xrate > 0.1 {
        //         self.focus[2] - xrate.copysign(self.focus[2])
        //     } else {
        //         self.focus[0] += self.focus[2];
        //         0.
        //     };
        // }
        // if self.focus[3] != 0. {
        //     self.focus[1] += yrate.copysign(self.focus[3]);
        //     self.focus[3] = if yrate > self.focus[3].abs() / self.ups {
        //         self.focus[3] - yrate.copysign(self.focus[3])
        //     } else {
        //         self.focus[1] += self.focus[3];
        //         0.
        //     };
        // }
        // dbg!(self.focus[2]);

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
            N => self.focus[3] += 0.02,
            NE => {
                self.focus[3] += 0.02;
                self.focus[2] -= 0.02;
            }
            E => self.focus[2] -= 0.02,
            SE => {
                self.focus[3] -= 0.02;
                self.focus[2] -= 0.02;
            }
            S => self.focus[3] -= 0.02,
            SW => {
                self.focus[3] -= 0.02;
                self.focus[2] += 0.02;
            }
            W => self.focus[2] += 0.02,
            NW => {
                self.focus[3] += 0.02;
                self.focus[2] += 0.02;
            }
            ResetZoom => {
                self.size = (20., 0.);
                if self.input.repeat() {
                    self.focus = [0.0, 0.0, 0.0, 0.0];
                } else {
                    self.focus = [
                        -(u16::MAX as f64) + (self.w / self.size.0 / 32. - 1.),
                        -(u16::MAX as f64) + (self.h / self.size.0 / 32. - 1.),
                        0.0,
                        0.0,
                    ];
                }
            }
        };

        #[allow(unused_variables)]
        for button in self.input.mouse() {
            match button {
                LMB(x, y) => self.world.put(&self.get_pos(x, y)),
                RMB(x, y) => self.world.remove(&self.get_pos(x, y)),
                MMB(x, y) => {
                    self.world.end();
                    self.focus = [
                        -(u16::MAX as f64 - self.w) / 2.,
                        -(u16::MAX as f64 - self.h) / 2.,
                        0.0,
                        0.0,
                    ];
                }
            }
        }
        #[allow(unused_variables)]
        match self.input.motion() {
            [Some(Scroll(scroll)), _] => {
                const FACTOR: f64 = 1.3;
                self.size.1 = scroll * self.size.0.abs().ln_1p().exp() * FACTOR;
            }
            // [None, Some(Drag(x1, y1, x2, y2))] => {}
            _ => {}
        }
    }

    fn get_pos(
        &self,
        x: &f64,
        y: &f64,
    ) -> Point<Point<u16>, usize> {
        let x1 = self.size.0.recip() * x / 32. - self.focus[0];
        let y1 = self.size.0.recip() * y / 32. - self.focus[1];
        Point(
            Point(x1.trunc() as u16, y1.trunc() as u16),
            ((x1.fract() * 32.).trunc() + ((y1.fract() * 32.).trunc() * 32.))
                .trunc() as usize,
        )
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

    pub fn render(
        &mut self,
        _args: &RenderArgs,
    ) {
    }

    pub fn exit(&mut self) { self.world.end(); }
}
