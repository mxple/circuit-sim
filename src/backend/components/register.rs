use crate::{component::Component, pin_enum, value::Value};

#[derive(Debug, Clone)]
pub struct Register {
    pub stored: Value,
    pub prev_lo: bool,
}

impl Register {
    pin_enum!(I { DATA, EN, CLK, CLR });
    pin_enum!(O { OUT });
}

impl Component for Register {
    fn init(&mut self) -> Vec<Value> {
        let mut ret = [Value::default(); Self::O_TOTAL as usize];
        ret[Self::O_OUT] = self.stored;
        ret.into()
    }

    fn update(&mut self, inputs: &[Value]) -> Vec<Value> {
        let mut ret = [Value::default(); Self::O_TOTAL as usize];
        ret[Self::O_OUT] = self.stored;
        if inputs[Self::I_CLR].is_hi() {
            self.prev_lo = false;
            self.stored = inputs[Self::I_DATA];
            ret[Self::O_OUT] = inputs[Self::I_DATA];
        } else if inputs[Self::I_EN].is_hi() && self.prev_lo && inputs[Self::I_CLK].is_hi() {
            self.prev_lo = false;
            self.stored = inputs[Self::I_DATA];
        }
        ret.into()
    }
    
    fn num_inputs(&self) -> usize {
        Self::I_TOTAL
    }
    fn num_outputs(&self) -> usize {
        Self::O_TOTAL
    }
}
