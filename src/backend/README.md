Nodes have input(s), logic, and output(s)
- choose to update state based on if inputs are sat
- maintains state of output throughout step
- reads input(s) by reading the output state of other nodes
- connected wires must have their own node in order to convey state to frontend

Each Node has:

ReadValues: fn() -> bool, Value[]
Logic: fn(Value[])
    Computes Out
    Enqueues next nodes to be visited
Out: Value[]

Master datastruct maps outputs to reachable input nodes

Queue (FIFO) of signals to propagate

Considerations:
- State
    - Out state of components are maintained across clock cycles (allows latches to work)
    - RAM and similar components maintain internal state seperatly through metadata
- Oscillation
    - Detect oscillation when same node is visited > N times

