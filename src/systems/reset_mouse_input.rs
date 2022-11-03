use bevy_ecs::prelude::ResMut;

use crate::{resources::mouse_inputs::MouseInputs, inputmanager::SharedInputBehaviour};

pub fn reset_mouse_inputs(mut input: ResMut<MouseInputs>) {
    input.reset_inputs();
}