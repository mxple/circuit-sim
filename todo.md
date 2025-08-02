# Frontend

- [ ] Canvas
    - [ ] Canvas

- [ ] GUI
    - [ ] Menu bar
        - [ ] File
        - [ ] Edit
        - [ ] View
        - [ ] Window
        - [ ] Help
        - [ ] About
    - [ ] Tool bar
        - [ ] Hotbar
    - [ ] Left panel
        - [ ] Component selector (file explorer-esque)
        - [ ] Settings
    - [ ] Right panel
    - [ ] Status bar


```rust
enum ComponentType {
    Custom(model),
    Builtin(supported_bit_widths, supported_inputs),
}

struct Component {
    name: String,
    svg: Resource,
    ports: Port[],
    builtin?
    bitsizes 0-32
    

}

```

Value 
- input width 
- value 
- z_high
- error

ANDGate
- Value inputs[]

intial = -1;
for (i in inputs):
    initial &= i;

return intial

- outputs[1]
