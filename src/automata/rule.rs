use std::{ops::Index, sync::Arc, usize};

use bevy::prelude::*;

use super::{FieldAccess, FieldAccessMut};

#[derive(Component)]
pub struct RuleContext<T> {
    pub rules: Vec<Rule<T>>,
}

impl <T: Copy> RuleContext<T> where T: PartialEq<T> {
    pub const fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    pub fn with_capacity(capactiy: usize) -> Self {
        Self {
            rules: Vec::with_capacity(capactiy),
        }
    }

    pub fn with(&mut self, rule: Rule<T>) {
        self.rules.push(rule);
    }

    pub fn apply(&self, source: &FieldAccess<T>, target: &mut FieldAccessMut<T>, pos: IVec2) {
        for rule in &self.rules {
            if rule.check(source, pos) {
                rule.apply(source, target, pos);
                return;
            }
        }
    }
}

pub struct FieldMod<T> {
    pub pos: IVec2,
    pub value: Change<T>,
}

pub enum Change<T> {
    Set(T),
    Clone(IVec2),
}

pub type FieldMods<T> = Vec<FieldMod<T>>;

pub struct Rule<T> {
    pub state: State<T>,
    pub result: Change<T>,
}

impl <T: Copy + PartialEq> Rule<T> where for<'a> &'a T: PartialEq<&'a T> {
    pub fn check(&self, accessor: &FieldAccess<T>, pos: IVec2) -> bool {
        for (index, value) in self.state.buffer.iter().enumerate() {
            if *value != accessor[self.state.shape.get_pos(index).expect("Index not in shape") + pos] {
                return false;
            }
        }
        true
    }

    pub fn apply(&self, source: &FieldAccess<T>, target: &mut FieldAccessMut<T>, pos: IVec2) {
        match &self.result {
            Change::Set(value) => target[pos] = *value,
            Change::Clone(clone_source) => target[pos] = source[*clone_source],
        }
    }
}

impl <T> Index<IVec2> for Rule<T> {
    type Output = T;

    fn index(&self, index: IVec2) -> &Self::Output {
        &self.state[index]
    }
}

pub struct State<T> {
    pub shape: Arc<dyn Shape>,
    pub buffer: Vec<T>,
}

impl <T> Index<IVec2> for State<T> {
    type Output = T;

    fn index(&self, index: IVec2) -> &Self::Output {
        &self.buffer[self.shape.get_index(index).unwrap()]
    }
}

pub trait Shape: Send + Sync {
    fn get_index(&self, pos: IVec2) -> Option<usize>;
    fn get_pos(&self, index: usize) -> Option<IVec2>;
    fn size(&self) -> usize;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Square(pub u32);

impl Shape for Square {
    fn get_index(&self, IVec2 {x, y}: IVec2) -> Option<usize> {
        let size = self.0;
        if x.abs() as u32 <= size - 1 && y.abs() as u32 <= size - 1 {
            Some((x + size as i32 - 1 + (((size << 1) - 1) as i32) * (y + size as i32 - 1)).try_into().unwrap())
        } else {
            None
        }
    }

    fn get_pos(&self, index: usize) -> Option<IVec2> {
        let size = self.0; 
        if index >= self.size() {
            None
        } else {
            let x = (index as i32 % ((size << 1) - 1) as i32) as i32 - size as i32 + 1;
            let y = ((index as i32 - x as i32) / (((size as i32) << 1) - 1)) as i32 - size as i32 + 1;
            Some(IVec2{x, y})
        }
    }

    fn size(&self) -> usize {
        let size = self.0;
        (((size as usize) << 1) - 1).pow(2)
    }
}
