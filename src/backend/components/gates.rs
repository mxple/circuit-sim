use crate::{component::Component, value::Value};

macro_rules! gates {
    ($($Id:ident: $f:expr, $invert:expr),* $(,)?) => {
        $(
            pub struct $Id {
                bitsize: u8,
                num_inputs: u8,
            }

            impl $Id {
                pub fn new(bitsize: u8, num_inputs: u8) -> Self {
                    Self { bitsize, num_inputs }
                }
            }

            impl Component for $Id {
                fn init(&mut self) -> Vec<Value> {
                    vec![Value::floating(self.bitsize)]
                }

                fn update(&mut self, input: &[Value]) -> Vec<Value> {
                    let out = input[..usize::from(self.num_inputs)]
                        .iter()
                        .cloned()
                        .reduce($f)
                        .unwrap_or_else(|| Value::floating(self.bitsize));

                    let out = if $invert {
                        !out
                    } else {
                        out
                    };

                    vec![out]
                }

                fn num_inputs(&self) -> usize {
                    self.num_inputs as usize
                }
                fn num_outputs(&self) -> usize {
                    1
                }
            }
        )*
    }
}

gates! {
    AndGate:  |a, b| a & b, false,
    OrGate:   |a, b| a | b, false,
    XorGate:  |a, b| a ^ b, false,
    NandGate: |a, b| a & b, true,
    NorGate:  |a, b| a | b, true,
    XNorGate: |a, b| a ^ b, true,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_gate() {
        let mut gate = AndGate::new(32, 100);
        let init = gate.init();
        assert_eq!(init.len(), 1);
        assert!(init[0].is_floating());

        let inputs: Vec<Value> = (0..100).map(|_| Value::from(rand::random::<u32>())).collect();
        let output = gate.update(&inputs);
        assert_eq!(output[0].bits, inputs.iter().fold(u32::MAX, |a, b| {a & b.bits}));
    }

    #[test]
    fn test_nand_gate() {
        let mut gate = NandGate::new(32, 100);
        let init = gate.init();
        assert_eq!(init.len(), 1);
        assert!(init[0].is_floating());

        let inputs: Vec<Value> = (0..100).map(|_| Value::from(rand::random::<u32>())).collect();
        let output = gate.update(&inputs);
        assert_eq!(output[0].bits, !inputs.iter().fold(u32::MAX, |a, b| {a & b.bits}));
    }

    #[test]
    fn test_or_gate() {
        let mut gate = OrGate::new(32, 100);
        let init = gate.init();
        assert_eq!(init.len(), 1);
        assert!(init[0].is_floating());

        let inputs: Vec<Value> = (0..100).map(|_| Value::from(rand::random::<u32>())).collect();
        let output = gate.update(&inputs);
        assert_eq!(output[0].bits, inputs.iter().fold(0, |a, b| {a | b.bits}));
    }

    #[test]
    fn test_nor_gate() {
        let mut gate = NorGate::new(32, 100);
        let init = gate.init();
        assert_eq!(init.len(), 1);
        assert!(init[0].is_floating());

        let inputs: Vec<Value> = (0..100).map(|_| Value::from(rand::random::<u32>())).collect();
        let output = gate.update(&inputs);
        assert_eq!(output[0].bits, !inputs.iter().fold(0, |a, b| {a | b.bits}));
    }

    #[test]
    fn test_xor_gate() {
        let mut gate = XorGate::new(32, 100);
        let init = gate.init();
        assert_eq!(init.len(), 1);
        assert!(init[0].is_floating());

        let inputs: Vec<Value> = (0..100).map(|_| Value::from(rand::random::<u32>())).collect();
        let output = gate.update(&inputs);
        assert_eq!(output[0].bits, inputs.iter().fold(0, |a, b| {a ^ b.bits}));
    }

    #[test]
    fn test_xnor_gate() {
        let mut gate = XNorGate::new(32, 100);
        let init = gate.init();
        assert_eq!(init.len(), 1);
        assert!(init[0].is_floating());

        let inputs: Vec<Value> = (0..100).map(|_| Value::from(rand::random::<u32>())).collect();
        let output = gate.update(&inputs);
        assert_eq!(output[0].bits, !inputs.iter().fold(0, |a, b| {a ^ b.bits}));
    }
}
