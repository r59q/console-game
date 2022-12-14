use bevy_ecs::query::With;
use bevy_ecs::system::{Query, Res};
use crate::components::controllable::Controllable;
use crate::components::velocity::Velocity;
use crate::input_manager::axis::Axis::{Horizontal, Vertical};
use crate::resources::inputs::axis_inputs::AxisInputs;

pub fn axis_velocity(mut query: Query<&mut Velocity, With<Controllable>>, axis_inputs :Res<AxisInputs>) {

    // Todo: should find movement speed from component instead
    //  - Maybe add speed multiplier component
    let movement_speed = 30.;

    for mut velocity in &mut query {
        velocity.x = axis_inputs.get(&Horizontal) * movement_speed;
        velocity.y = axis_inputs.get(&Vertical) * movement_speed;
    }
}