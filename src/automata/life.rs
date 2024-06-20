use std::sync::Arc;

use bevy::math::bool;

use crate::automata::rule::{Change, Shape, State, Rule};

use super::rule::{RuleContext, Square};

const SQUARE: Square = Square(2);

pub fn gen_life_context() -> RuleContext<bool> {
    let mut ctx: RuleContext<bool> = RuleContext::with_capacity(0b1_000_000_000);
    let mut i: u32 = 0;
    println!("{}", SQUARE.size());
    while i < 0b1_000_000_000_u32 {
        let value =  (i >> 4) & 1;
        let neighbors = i.count_ones() - value;
        let mut output = false;
        if value == 1 {
            if neighbors == 2 || neighbors == 3 {
                output = true;
            }
        } else {
            if neighbors == 3 {
                output = true;
            }
        }

        println!("{}", output);

        let mut vec = Vec::with_capacity(9);
        let mut j = 0;
        while j < 9 {
            vec.push(((i >> j) & 1) != 0);
            j += 1;
        }

        ctx.with(Rule::<bool>{ 
            state: State {
                shape: Arc::new(SQUARE),
                buffer: vec
            }, 
            result: Change::Set(output)
        });
        i += 1;
    }
    
    ctx
}
