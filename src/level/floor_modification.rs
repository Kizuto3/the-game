use bevy::{ecs::observer::TriggerTargets, prelude::*};
use bevy_rapier2d::prelude::{CollisionEvent, Sensor, Velocity};

use crate::{movement::Jumper, npc::NPC, Cweampuff};

use super::level_layout::{DoorCollider, JumpPad};

const JUMP_PAD_VELOCITY_DELTA: f32 = 2.;

pub fn jump_pad_collision_reader(
    jump_pads: Query<Entity, (With<Sensor>, With<JumpPad>, Without<NPC>, Without<DoorCollider>)>,
    mut cweampuff: Single<(Entity, &mut Velocity, &Jumper), With<Cweampuff>>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    let (cweampuff_entity, cweampuff_velocity, cweampuff_jumper) = &mut *cweampuff;
    for event in contact_events.read() {    
        if let CollisionEvent::Started(h1, h2, _flags) = event {
            for jump_pad_entity in jump_pads.iter() {
                if h1.entities().iter().any(|f| *f == jump_pad_entity || *f == *cweampuff_entity) && 
                   h2.entities().iter().any(|f| *f == jump_pad_entity || *f == *cweampuff_entity) {
                    cweampuff_velocity.linvel.y = cweampuff_jumper.jump_impulse * JUMP_PAD_VELOCITY_DELTA;

                    return;
                }
            }
        }
    }
}