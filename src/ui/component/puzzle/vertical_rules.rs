use std::rc::Rc;
use std::cell::{Cell};
use cell::RefCell;
use sdl2::mouse::MouseButton;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::common::*;
use ui::widget::brick::*;
use ui::rule::{draw_rule};
use ui::component::game::{GamePrivate};
use resources::manager::ResourceManager;
use resources::thing::EMPTY_TILE;
use error::*;

pub struct VerticalRules {
    rect: Rect,
    state: Rc<RefCell<GamePrivate>>,
    highlighted: Cell<Option<usize>>,
}

const TILE_GAP: i32 =      4;
const TILE_WIDTH: i32 =   48;
const TILE_HEIGHT: i32 =  48;

impl VerticalRules {
    pub fn new(rect: Rect, state: Rc<RefCell<GamePrivate>>) -> Result<Self> {
        Ok(Self {
            rect,
            state,
            highlighted: Cell::new(None),
        })
    }

    fn draw_cell(&self, index: usize) -> Brick {
        let rect = self.rect(index);

        if let Some(vertical_rule) = self.state.borrow().vertical_rules.get(index) {
            if self.state.borrow().show_excluded == vertical_rule.is_excluded {
                let rule = &self.state.borrow().rules[vertical_rule.original_index];
                return Brick::new(rect)
                    .add(draw_rule(&rule, self.highlighted.get() == Some(index)));
            }
        }

        Brick::new(rect)
            .background(BackgroundPattern::Custom("EMPTY_TILE", EMPTY_TILE, false))
    }

    fn get_rule_index(&self, x: i32, y: i32) -> Option<usize> {
        if !self.get_client_rect().contains_point((x, y)) {
            return None;
        }
        if x % (TILE_WIDTH + TILE_GAP) < TILE_WIDTH {
            let index = x / (TILE_WIDTH + TILE_GAP);
            Some(index as usize)
        } else {
            None
        }
    }

    fn rect(&self, index: usize) -> Rect {
        Rect::new((index as i32) * (TILE_WIDTH + TILE_GAP), 0, TILE_WIDTH as u32, TILE_HEIGHT as u32 * 2)
    }
}

impl Widget<Nothing> for VerticalRules {
    fn is_relative(&self) -> bool { true }
    fn get_rect(&self) -> Rect { self.rect }

    fn on_event(&mut self, event: &Event) -> EventResult<Nothing> {
        match *event {
            Event::MouseButtonDown(MouseButton::Right, x, y) => {
                match self.get_rule_index(x, y) {
                    Some(no) => {
                        if self.state.borrow_mut().toggle_vertical_rule(no).is_some() {
                            // sound->play(L"whizz.wav");
                            Ok(EventReaction::update(self.rect(no)))
                        } else {
                            Ok(EventReaction::empty())
                        }
                    },
                    None => Ok(EventReaction::empty())
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
                    Ok(EventReaction::updates(&rects))
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        let mut brick = Brick::new(self.get_rect());
        let num = ((self.get_client_rect().width() as i32 + TILE_GAP) / (TILE_WIDTH + TILE_GAP)) as usize;
        for i in 0..num {
            let b = self.draw_cell(i);
            brick.push(b);
        }
        brick
    }
}
