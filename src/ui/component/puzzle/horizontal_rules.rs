use std::rc::Rc;
use std::cell::Cell;
use debug_cell::RefCell;
use sdl::video::Surface;
use sdl::event::{Mouse};
use sdl2::rect::{Rect, Point};
use ui::context::Context;
use ui::widget::widget::*;
use ui::rule::{draw_rule};
use resources::thing::ThingImages;
use ui::utils::load_image;
use ui::component::game::{GamePrivate};
use error::*;

const TILE_BG: &[u8] = include_bytes!("./tile.bmp");

pub struct HorizontalRules {
    state: Rc<RefCell<GamePrivate>>,
    highlighted: Cell<Option<usize>>,
    tile: Surface,
    thing_images: Rc<ThingImages>,
}

const HINTS_COLS: usize =  3;
const HINTS_ROWS: usize =  8;
const TILE_GAP_X: i32 =    4;
const TILE_GAP_Y: i32 =    4;
const TILE_X: i32 =      348;
const TILE_Y: i32 =       68;
const TILE_WIDTH: i32 =   48;
const TILE_HEIGHT: i32 =  48;

impl HorizontalRules {
    pub fn new(state: Rc<RefCell<GamePrivate>>) -> Result<Self> {
        let tile = load_image(TILE_BG)?;
        Ok(Self {
            state,
            highlighted: Cell::new(None),
            tile,
            thing_images: ThingImages::new()?
        })
    }

    fn draw_cell(&self, context: &Context, index: usize) -> Result<()> {
        let rect = self.rect(index);
        let c = context.relative(rect);

        if let Some(horizontal_rule) = self.state.borrow().horizontal_rules.get(index) {
            if self.state.borrow().show_excluded == horizontal_rule.is_excluded {
                let rule = &self.state.borrow().rules[horizontal_rule.original_index];
                draw_rule(&self.thing_images, &rule, &c, self.highlighted.get() == Some(index))?;
                return Ok(());
            }
        }

        c.image(&self.tile, 0, 0);
        c.image(&self.tile, TILE_WIDTH, 0);
        c.image(&self.tile, TILE_WIDTH * 2, 0);
        Ok(())
    }

    fn get_rule_index(&self, x: i32, y: i32) -> Option<usize> {
        if !self.get_rect().contains_point(Point::new(x, y)) {
            return None;
        }

        let x = x - TILE_X;
        let y = y - TILE_Y;

        let col: usize = (x / (TILE_WIDTH*3 + TILE_GAP_X)) as usize;
        if (col as i32) * (TILE_WIDTH*3 + TILE_GAP_X) + TILE_WIDTH*3 < x {
            return None;
        }

        let row: usize = (y / (TILE_HEIGHT + TILE_GAP_Y)) as usize;
        if (row as i32) * (TILE_HEIGHT + TILE_GAP_Y) + TILE_HEIGHT < y {
            return None;
        }
    
        let no = row * HINTS_COLS + col;
        if no >= self.state.borrow().horizontal_rules.len() {
            return None;
        }

        Some(no)
    }

    fn rect(&self, index: usize) -> Rect {
        let row = index / HINTS_COLS;
        let col = index % HINTS_COLS;
        Rect::new(
            TILE_X + (col as i32) * (TILE_WIDTH*3 + TILE_GAP_X),
            TILE_Y + (row as i32) * (TILE_HEIGHT + TILE_GAP_Y),
            TILE_WIDTH as u32 * 3,
            TILE_HEIGHT as u32
        )
    }
}

impl Widget<Nothing> for HorizontalRules {
    fn get_rect(&self) -> Rect {
        Rect::new(
            TILE_X,
            TILE_Y,
            ((TILE_WIDTH as u32) * 3 + (TILE_GAP_X as u32)) * (HINTS_COLS as u32),
            ((TILE_HEIGHT + TILE_GAP_Y) as u32) * (HINTS_ROWS as u32)
        )
    }

    fn on_event(&self, event: &Event) -> EventReaction<Nothing> {
        match *event {
            Event::MouseButtonDown(Mouse::Right, x, y) => {
                match self.get_rule_index(x, y) {
                    Some(no) => {
                        if self.state.borrow_mut().toggle_horizontal_rule(no).is_some() {
                            // sound->play(L"whizz.wav");
                            EventReaction::Redraw // self.rect(no)
                        } else {
                            EventReaction::NoOp
                        }
                    },
                    None => EventReaction::NoOp
                }
            },
            Event::MouseMove(x, y) => {
                let no = self.get_rule_index(x, y);
                if no != self.highlighted.get() {
                    let mut rects = Vec::new();
                    if let Some(index) = self.highlighted.get() {
                        rects.push(self.rect(index));
                    }
                    if let Some(index) = no {
                        self.highlighted.set(Some(index));
                        rects.push(self.rect(index));
                    } else {
                        self.highlighted.set(None);
                    }
                    EventReaction::Redraw // Some(Effect::Redraw(rects))
                } else {
                    EventReaction::NoOp
                }
            },
            _ => EventReaction::NoOp,
        }
    }

    fn draw(&self, context: &Context) -> Result<()> {
        for i in 0..(HINTS_ROWS * HINTS_COLS) {
            self.draw_cell(context, i)?;
        }
        Ok(())
    }
}
