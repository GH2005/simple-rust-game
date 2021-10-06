use specs::prelude::*;
use sdl2::rect::Rect;
use crate::components::*;

pub type CanvasRegion = Rect;
pub type FrameDuration = std::time::Duration;

pub struct Physics;

fn shift(mut pos: Rect, vel: &Velocity, dur: FrameDuration) -> Rect {
    match vel.direction {
        Direction::Left => pos.offset((-vel.speed * dur.as_millis() as f64) as i32, 0),
        Direction::Right => pos.offset((vel.speed * dur.as_millis() as f64) as i32, 0),
        Direction::Up => pos.offset(0, (-vel.speed * dur.as_millis() as f64) as i32),
        Direction::Down => pos.offset(0, (vel.speed * dur.as_millis() as f64) as i32),
    };
    pos
}

impl<'a> System<'a> for Physics {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        ReadExpect<'a, CanvasRegion>,
        ReadStorage<'a, Item>,
        ReadStorage<'a, KeyboardControlled>,
        ReadStorage<'a, Enemy>,
        ReadExpect<'a, FrameDuration>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let canvas_region = *data.3;
        let frame_duration = *data.7;

        use itertools::Itertools;
        use std::collections::{HashMap, HashSet};

        let destinations: HashMap<Entity, Rect> = (&data.0, &data.1, &data.2).join()
            .map( |(entity, &Position(pos), vel)| (entity, shift(pos, vel, frame_duration)) )
            .collect();

        let stalled_entities: HashSet<Entity> = destinations.iter()
            .combinations(2)
            .filter_map(
                |comb|
                    if comb[0].1.has_intersection(*comb[1].1) { Some( [*comb[0].0, *comb[1].0] ) }
                    else { None }
            )
            .flatten()
            .collect();

        for (ref entity, Position(pos), _) in (&data.0, &mut data.1, &data.2).join()
            .filter( |(entity, _, _)| !stalled_entities.contains(entity) )
            .filter( |(entity, _, _)| canvas_region.contains_rect(destinations[entity]) )
        {
            *pos = destinations[entity];
        }


        let item_entities: HashSet<Entity>  = (&data.0, &data.4).join().map( |(entity, _)| entity ).collect();
        let keyboard_controlled_entities: HashSet<Entity>  = (&data.0, &data.5).join().map( |(entity, _)| entity ).collect();

        for &entity in destinations.iter()
            .filter( |(entity, _)| item_entities.contains(entity) || keyboard_controlled_entities.contains(entity) )
            .combinations(2)
            .filter_map(
                |comb|
                    if comb[0].1.has_intersection(*comb[1].1) { Some( [*comb[0].0, *comb[1].0] ) }
                    else { None }
            )
            .flatten()
            .collect::<HashSet<Entity>>()
            .intersection(&item_entities)
        {
            data.0.delete(entity).unwrap_or(());
        }
    }
}