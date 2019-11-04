// You'll have to mark PADDLE_HEIGHT as public in gotem.rs
use crate::gotem::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

// use amethyst::core::Transform;
// use amethyst::derive::SystemDesc;
// use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
// use amethyst::input::{InputHandler, StringBindings};

use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};


pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let side_name = match paddle.side {
                        Side::Left => "left",
                        Side::Right => "right",
                    };
                    println!("Side {:?} moving {}", side_name, mv_amount);
                }
            }
        }
    }
}