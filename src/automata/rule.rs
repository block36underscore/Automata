use std::{sync::Arc, usize};

use bevy::prelude::*;

use super::{FieldAccess, FieldAccessMut};

#[derive(Component)]
pub struct RuleContext<T> {
    pub rules: Vec<Arc<Rule<T>>>,
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

    pub fn with(&mut self, rule: Arc<Rule<T>>) {
        self.rules.push(rule);
    }

    pub fn apply(&self, source: &FieldAccess<T>, target: &mut FieldAccessMut<T>, pos: IVec2) {
        for rule in &self.rules {
            if let Some(change) = rule(source, pos) {
                change.apply(source, target, pos);
                return;
            }
        }
    }
}

pub enum Change<T> {
    Set(T),
    Clone(IVec2),
}

impl <T: Copy> Change<T> {
    pub fn apply(&self, source: &FieldAccess<T>, target: &mut FieldAccessMut<T>, pos: IVec2) {
        match self {
            Self::Set(value)        => target[pos] = *value,
            Self::Clone(source_pos) => target[pos] = source[*source_pos],
        }
    }
}

pub type Rule<T> = fn(&FieldAccess<T>, IVec2) -> Option<Change<T>>;
