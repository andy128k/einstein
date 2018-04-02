use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
use sdl::video::Surface;
use sdl::event::{Mouse};
use sdl2::rect::{Rect, Point};
use ui::context::Context;
use ui::widget::widget::*;
use ui::rule::{draw_rule};
use ui::utils::load_image;
use ui::component::game::{GamePrivate};
use resources::thing::ThingImages;
use error::*;

const TILE_BG: &[u8] = include_bytes!("./tile.bmp");

pub struct VerticalRules {
    state: Rc<RefCell<GamePrivate>>,
    highlighted: Cell<Option<usize>>,
    tile: Surface,
    thing_images: Rc<ThingImages>,
}

const TILE_NUM: usize =   15;
const TILE_GAP: i32 =      4;
const TILE_X: i32 =       12;
const TILE_Y: i32 =      495;
const TILE_WIDTH: i32 =   48;
const TILE_HEIGHT: i32 =  48;

impl VerticalRules {
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

        if let Some(vertical_rule) = self.state.borrow().vertical_rules.get(index) {
            if self.state.borrow().show_excluded == vertical_rule.is_excluded {
                let rule = &self.state.borrow().rules[vertical_rule.original_index];
                draw_rule(&self.thing_images, &rule, &c, self.highlighted.get() == Some(index))?;
                return Ok(());
            }
        }

        c.image(&self.tile, 0, 0);
        c.image(&self.tile, 0, TILE_HEIGHT);
        Ok(())
    }

    fn get_rule_index(&self, x: i32, y: i32) -> Option<usize> {
        if !self.get_rect().contains_point(Point::new(x, y)) {
            return None;
        }
        if (x - TILE_X) % (TILE_WIDTH + TILE_GAP) < TILE_WIDTH {
            let index = (x - TILE_X) / (TILE_WIDTH + TILE_GAP);
            Some(index as usize)
        } else {
            None
        }
    }

    fn rect(&self, index: usize) -> Rect {
        Rect::new(TILE_X + (index as i32) * (TILE_WIDTH + TILE_GAP), TILE_Y, TILE_WIDTH as u32, TILE_HEIGHT as u32 * 2)
    }

    fn get_rect(&self) -> Rect {
        Rect::new(
            TILE_X,
            TILE_Y,
            (TILE_WIDTH as u32) * (TILE_NUM as u32) + (TILE_GAP as u32) * (TILE_NUM as u32 - 1),
            (TILE_HEIGHT * 2) as u32
        )
    }
}

impl Widget<Nothing> for VerticalRules {
    fn on_event(&self, event: &Event) -> EventReaction<Nothing> {
        match *event {
            Event::MouseButtonDown(Mouse::Right, x, y) => {
                match self.get_rule_index(x, y) {
                    Some(no) => {
                        if self.state.borrow_mut().toggle_vertical_rule(no).is_some() {
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
                    if let Some(index) = self.highlighted.get() { // && isActive
                        rects.push(self.rect(index));
                    }
                    if let Some(index) = no { // && isActive
                        self.highlighted.set(Some(index));
                        rects.push(self.rect(index));
                    } else {
                        self.highlighted.set(None);
                    }
                    EventReaction::Redraw // rects
                } else {
                    EventReaction::NoOp
                }
            },
            _ => EventReaction::NoOp,
        }
    }

    fn draw(&self, context: &Context) -> Result<()> {
        for i in 0..TILE_NUM {
            self.draw_cell(context, i)?;
        }
        Ok(())
    }
}
