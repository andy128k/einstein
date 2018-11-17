use std::rc::Rc;
use std::cell::Cell;
use crate::cell::RefCell;
use failure::err_msg;
use sdl2::mouse::MouseButton;
use crate::ui::context::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::ui::rule::{draw_rule};
use crate::ui::component::game::{GamePrivate};
use crate::resources::manager::ResourceManager;
use crate::resources::thing::EMPTY_TILE;
use crate::resources::audio::WHIZZ;
use crate::audio::Audio;

pub struct RuleWidget {
    size: Size,
    state: Rc<RefCell<GamePrivate>>,
    index: Option<usize>,
    highlighted: Cell<bool>,
}

impl RuleWidget {
    pub fn new(size: Size, state: Rc<RefCell<GamePrivate>>, index: Option<usize>) -> Self {
        RuleWidget {
            size,
            state,
            index,
            highlighted: Cell::new(false),
        }
    }

    fn get_rule(&self) -> Option<crate::rules::Rule> {
        let index = self.index?;
        let rule = self.state.borrow().rules.get(index)?.clone();
        if self.state.borrow().show_excluded == self.state.borrow().excluded.contains(&index) {
            Some(rule)
        } else {
            None
        }
    }
}

impl Widget<usize> for RuleWidget {
    fn get_size(&self) -> Size { self.size }

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &Audio) -> EventResult<usize> {
        match *event {
            Event::MouseButtonDown(MouseButton::Right, x, y) => {
                if self.get_size().to_rect().contains_point((x, y)) {
                    if self.index.map_or(false, |index| self.state.borrow_mut().toggle_rule(index).is_some()) {
                        audio.play(&*resource_manager.chunk(&WHIZZ)).map_err(err_msg)?;
                        Ok(EventReaction::update())
                    } else {
                        Ok(EventReaction::empty())
                    }
                } else {
                    Ok(EventReaction::empty())
                }
            },
            Event::MouseMove(x, y) => {
                if self.index.is_some() {
                    let inside = self.get_size().to_rect().contains_point((x, y));
                    if inside == self.highlighted.get() {
                        Ok(EventReaction::empty())
                    } else {
                        self.highlighted.set(inside);
                        Ok(EventReaction::update())
                    }
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        if let Some(rule) = self.get_rule() {
            draw_rule(&rule, self.highlighted.get())
        } else {
            Brick::new(self.size.width, self.size.height)
                .background(Background::Pattern(&EMPTY_TILE, self.highlighted.get()))
        }
    }
}
