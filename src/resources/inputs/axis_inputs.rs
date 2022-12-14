use std::collections::HashMap;
use crate::input_manager::axis::Axis;
use crate::resources::inputs::SharedInputBehaviour;

pub struct AxisInputs {
    input_map: HashMap<Axis, f64>,
}

impl AxisInputs {
    pub fn new() -> Self {
        return Self { /* fields */ input_map: HashMap::new() }
    }

    pub fn get(&self, axis: &Axis) -> f64 {
        if self.input_map.contains_key(axis) {
            return self.input_map.get(axis).unwrap().clone();
        }
        return 0.;
    }

    pub fn set(&mut self, axis: Axis, mut value: f64) {
        if value < -1. {
            value = -1.;
        }
        if value > 1. {
            value = 1.;
        }
        self.input_map.entry(axis)
            .and_modify(|val| *val = value)
            .or_insert(value);
    }
}

impl SharedInputBehaviour for AxisInputs {
    fn reset_inputs(&mut self) {
        for (_, val) in self.input_map.iter_mut() {
            *val = 0.;
        }
    }
}

#[cfg(test)]
mod test {
    use super::AxisInputs;
    use crate::input_manager::axis::Axis;
    use strum::{IntoEnumIterator};
    use crate::resources::inputs::SharedInputBehaviour;

    #[test]
    fn can_get_axial_input() {
        let axis_inputs = AxisInputs::new();

        let horizontal_input_value = axis_inputs.get(&Axis::Horizontal);
        assert_eq!(horizontal_input_value, 0.);
    }

    #[test]
    fn can_reset_input() {
        let mut axis_inputs = AxisInputs::new();
        axis_inputs.set(Axis::Horizontal, 1.);
        axis_inputs.set(Axis::Vertical, -1.);
        axis_inputs.reset_inputs();
        for axis in Axis::iter() {
            assert_eq!(axis_inputs.get(&axis), 0.);
        }
    }

    #[test]
    fn can_set_input() {
        let mut axis_inputs = AxisInputs::new();
        axis_inputs.set(Axis::Horizontal, 1.);
        axis_inputs.set(Axis::Vertical, -1.);

        assert_eq!(axis_inputs.get(&Axis::Horizontal), 1.);
        assert_eq!(axis_inputs.get(&Axis::Vertical), -1.);
    }

    #[test]
    fn set_is_clamped() {
        let mut axis_inputs = AxisInputs::new();
        axis_inputs.set(Axis::Horizontal, 10.);
        axis_inputs.set(Axis::Vertical, -10.);

        assert_eq!(axis_inputs.get(&Axis::Horizontal), 1.);
        assert_eq!(axis_inputs.get(&Axis::Vertical), -1.);
    }
}