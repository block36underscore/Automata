use std::sync::Arc;

use bevy::math::{bool, IVec2};

use crate::automata::rule::Change;

use super::{rule::RuleContext, FieldAccess};

pub fn gen_life_context() -> RuleContext<bool> {
    let mut ctx: RuleContext<bool> = RuleContext::with_capacity(0b1_000_000_000);
    ctx.with(Arc::new(life));   
    ctx
}

pub fn life(accessor: &FieldAccess<bool>, pos: IVec2) -> Option<Change<bool>> {
    let mut neighbors = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if accessor[pos + IVec2 {x: i, y: j}] {
                neighbors += 1;
            }
        }
    }

    let val = accessor[pos];
    if accessor[pos] {
        neighbors -= 1;
    }

    let output: bool;
    if val {
        output = neighbors == 2 || neighbors == 3;
    } else {
        output = neighbors == 3;
    }

    Some(Change::Set(output))
}
