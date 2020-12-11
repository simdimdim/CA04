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
    pub zoom:           (f64, f64),
}
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

    pub fn draw_tiles(
        &mut self,
        c: &Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
    ) {
        for b in self.world.tiles.values() {
            let transform = c
                .transform
                .trans(self.focus[0] + self.w / 2., self.focus[1] + self.h / 2.);
            let square = rectangle::square(
                b.x as f64 * (self.zoom.0 + 1.) as f64,
                b.y as f64 * (self.zoom.0 + 1.) as f64,
                if self.zoom.0 > 0.1 { self.zoom.0 } else { 0.1 },
            );
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
    }

    pub fn zoom(
        &mut self,
        zoom: f64,
    ) {
        const FACTOR: f64 = 2.3;
        self.zoom.1 = zoom * self.zoom.0.abs().ln_1p().exp() * FACTOR;
    }

    pub fn update(&mut self) {
        let step = 4. * self.zoom.1 / self.ups;
        self.zoom.0 = if self.zoom.0 + step > 0.5 {
            self.zoom.0 + step
        } else {
            0.5
        };
        self.zoom.1 = if step.abs() > 4. / self.ups {
            self.zoom.1 - step
        } else {
            0.
        };

        // println!("{:?}", self.zoom);

        self.world.update()
    }

    pub fn draw(
        &mut self,
        c: &Context,
        g: &mut GfxGraphics<Resources, CommandBuffer>,
        device: &mut Device,
    ) {
        self.draw_tiles(c, g);
        // // Generate and draw the lines for the Sudoku Grid.
        // use graphics::{Line, Rectangle};
        // let cell_edge = Line::new(cell_edge_color, cell_edge_radius);
        // let section_edge = Line::new(section_edge_color, section_edge_radius;
        // for i in 0..9 {
        //     let x = position[0] + i as f64 / 9.0 * size;
        //     let y = position[1] + i as f64 / 9.0 * size;
        //     let x2 = position[0] + size;
        //     let y2 = position[1] + size;
        //     let vline = [x, position[1], x, y2];
        //     let hline = [position[0], y, x2, y];
        //     // Draw Section Lines instead of Cell Lines
        //     if (i % 3) == 0 {
        //         section_edge.draw(vline, &c.draw_state, c.transform, g);
        //         section_edge.draw(hline, &c.draw_state, c.transform, g);
        //     }
        //     // Draw the regular cell Lines
        //     else {
        //         cell_edge.draw(vline, &c.draw_state, c.transform, g);
        //         cell_edge.draw(hline, &c.draw_state, c.transform, g);
        //     }
        // }
        // // Draw board edge.
        // Rectangle::new_border(board_edge_color, board_edge_radius).draw(
        //     board_rect,
        //     &c.draw_state,
        //     c.transform,
        //     g,
        // );
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
            N => self.focus[1] = self.focus[1] - self.zoom.0,
            E => self.focus[0] = self.focus[0] + self.zoom.0,
            S => self.focus[1] = self.focus[1] + self.zoom.0,
            W => self.focus[0] = self.focus[0] - self.zoom.0,
            NE => {
                self.focus[1] = self.focus[1] - self.zoom.0;
                self.focus[0] = self.focus[0] + self.zoom.0;
            }
            SE => {
                self.focus[1] = self.focus[1] + self.zoom.0;
                self.focus[0] = self.focus[0] + self.zoom.0;
            }
            SW => {
                self.focus[1] = self.focus[1] + self.zoom.0;
                self.focus[0] = self.focus[0] - self.zoom.0;
            }
            NW => {
                self.focus[1] = self.focus[1] - self.zoom.0;
                self.focus[0] = self.focus[0] - self.zoom.0;
            }
            ResetZoom => {
                self.zoom = (1., 0.);
                self.focus =
                    [(u16::MAX / 2) as f64, (u16::MAX / 2) as f64, 0., 0.];
            }
        };
        #[allow(unused_variables)]
        for button in self.input.mouse() {
            match button {
                LMB(x, y) => {
                    self.get_pos(x, y);
                }
                RMB(x, y) => {}
                MMB(x, y) => {}
                Drag(x1, y1, x2, y2) => {}
            }
        }
    }

    pub fn exit(&mut self) { self.world.end(); }

    fn get_pos(
        &self,
        x: &f64,
        y: &f64,
    ) -> (u16, u16) {
        (
            ((self.focus[0] + x - self.w / 2.) / self.zoom.0) as u16,
            ((self.focus[1] + y - self.h / 2.) / self.zoom.0) as u16,
        )
    }
}
