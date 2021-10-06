use specs::prelude::*;
use std::ops::Deref;

use crate::components::*;

const PLAYER_MOVEMENT_SPEED: f64 = 0.2;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let movement_command = match data.0.deref() {
            Some(movement_command) => movement_command,
            None => return, // no change
        };

        for (_, vel) in (&data.1, &mut data.2).join() {
            match movement_command {
                &MovementCommand::Move(direction) => {
                    vel.speed = PLAYER_MOVEMENT_SPEED;
                    vel.direction = direction;
                },
                MovementCommand::Stop => vel.speed = 0.0,
            }
        }
    }
}

pub enum MovementCommand {
    Stop,
    Move(Direction),
}