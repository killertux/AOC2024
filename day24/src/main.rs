use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{Error, Result},
    u64,
};

fn main() -> Result<()> {
    println!("Part 1 example 1: {}", part_1("example_1.txt")?);
    println!("Part 1 example 2: {}", part_1("example_2.txt")?);
    println!("Part 1 input: {}", part_1("input.txt")?);
    println!("Part 2 input: {}", part_2("input.txt")?);
    Ok(())
}

fn part_1(file: &str) -> Result<usize> {
    let (gates, wires) = read_input(file)?;
    let wires = execute(gates, wires);
    let mut z_wires: Vec<(Wire, WireState)> = wires
        .into_iter()
        .filter(|(wire, _)| wire.0.starts_with('z'))
        .collect();
    z_wires.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result = 0;
    for (i, wire) in z_wires.iter().enumerate() {
        if wire.1 == WireState::High {
            result |= 0x1 << i;
        }
    }
    Ok(result)
}

fn part_2(file: &str) -> Result<String> {
    let (gates, wires) = read_input(file)?;

    // In a full adder, we should not have any gate different than a XOR connected to the output.
    // With the exception of the last output which is the carry
    let incorrect_last_gates = gates
        .iter()
        .filter(|gate| match gate {
            LogicGate::And(_, _, output) | LogicGate::Or(_, _, output) => {
                output.0.starts_with('z') && output.0 != "z45"
            }
            _ => false,
        })
        .map(|gate| gate.output().clone())
        .collect_vec();
    // In a full adder, XORs gates should be either connected to the output or the input
    let incorrect_intermediary_gates = gates
        .iter()
        .filter(|gate| match gate {
            LogicGate::Xor(input_a, input_b, output) => {
                !output.0.starts_with('z')
                    && (!(input_a.0.starts_with('x') || input_a.0.starts_with('y'))
                        || !(input_b.0.starts_with('x') || input_b.0.starts_with('y')))
            }
            _ => false,
        })
        .map(|gate| gate.output().clone())
        .collect_vec();
    let mut correct_perm = Vec::new();
    let mut best_diff = u64::MAX;
    for perm in incorrect_intermediary_gates
        .iter()
        .chain(incorrect_last_gates.iter())
        .permutations(6)
        .filter(|perms| {
            incorrect_intermediary_gates.contains(perms[0])
                && incorrect_last_gates.contains(perms[1])
                && incorrect_intermediary_gates.contains(perms[2])
                && incorrect_last_gates.contains(perms[3])
                && incorrect_intermediary_gates.contains(perms[4])
                && incorrect_last_gates.contains(perms[5])
        })
    {
        let gates = swap_outputs(perm[0], perm[1], gates.clone());
        let gates = swap_outputs(perm[2], perm[3], gates);
        let gates = swap_outputs(perm[4], perm[5], gates);
        let wires = set_input('x', 0x1FFFFFFFFFFF, wires.clone());
        let wires = set_input('y', 0x1FFFFFFFFFFF, wires);
        let result = calculate_result(&execute(gates, wires));
        let diff = result ^ (0x1FFFFFFFFFFF + 0x1FFFFFFFFFFF);
        // Loking the results by Eye. In this one we can clearly see that we have one carries with issue
        if best_diff.count_ones() > diff.count_ones() {
            best_diff = diff;
            correct_perm = perm;
        }
    }
    let gates = swap_outputs(correct_perm[0], correct_perm[1], gates.clone());
    let gates = swap_outputs(correct_perm[2], correct_perm[3], gates);
    let gates = swap_outputs(correct_perm[4], correct_perm[5], gates);
    let mut nodes_to_check = Vec::new();
    for i in 0..=46 {
        if best_diff & (0x1 << i) > 0 {
            nodes_to_check.extend(list_of_nodes_from_output(
                Wire(format!("z{:02}", i + 1)),
                &gates,
            ))
        }
    }
    for perm in nodes_to_check.iter().permutations(2) {
        let gates = swap_outputs(perm[0], perm[1], gates.clone());
        let wires = set_input('x', 0x1FFFFFFFFFFF, wires.clone());
        let wires = set_input('y', 0x1FFFFFFFFFFF, wires.clone());
        let result = calculate_result(&execute(gates.clone(), wires.clone()));
        if result == 0x1FFFFFFFFFFF + 0x1FFFFFFFFFFF {
            let mut correct = true;
            for i in 0..45 {
                let value = 0x1 << i;
                let wires = set_input('x', value, wires.clone());
                let wires = set_input('y', 0, wires.clone());
                let result = calculate_result(&execute(gates.clone(), wires.clone()));
                if result != value {
                    correct = false;
                    break;
                }
            }
            if correct {
                correct_perm.extend(perm);
                break;
            }
        }
    }
    correct_perm.sort();
    Ok(correct_perm
        .into_iter()
        .map(|wire| wire.0.clone())
        .join(","))
}

fn list_of_nodes_from_output(output: Wire, gates: &[LogicGate]) -> Vec<Wire> {
    let mut result = Vec::new();
    let mut outputs_to_check = vec![output];
    while let Some(output) = outputs_to_check.pop() {
        for gate in gates.iter() {
            if *gate.output() == output {
                outputs_to_check.extend(
                    gate.inputs()
                        .into_iter()
                        .filter(|input| !(input.0.starts_with('x') || input.0.starts_with('y'))),
                );
                result.extend(
                    gate.inputs()
                        .into_iter()
                        .filter(|input| !(input.0.starts_with('x') || input.0.starts_with('y'))),
                );
            }
        }
    }
    result
}

fn swap_outputs(output_1: &Wire, output_2: &Wire, gates: Vec<LogicGate>) -> Vec<LogicGate> {
    gates
        .into_iter()
        .map(|mut gate| {
            match &mut gate {
                LogicGate::Or(_, _, output)
                | LogicGate::And(_, _, output)
                | LogicGate::Xor(_, _, output) => {
                    if output == output_1 {
                        *output = output_2.clone();
                    } else if output == output_2 {
                        *output = output_1.clone();
                    }
                }
            }
            gate
        })
        .collect()
}

fn calculate_result(wires: &HashMap<Wire, WireState>) -> u64 {
    let mut z_wires: Vec<(&Wire, &WireState)> = wires
        .iter()
        .filter(|(wire, _)| wire.0.starts_with('z'))
        .collect();
    z_wires.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result = 0;
    for (i, wire) in z_wires.iter().enumerate() {
        if *wire.1 == WireState::High {
            result |= 0x1 << i;
        }
    }
    result
}

fn set_input(
    input: char,
    value: u64,
    mut wires: HashMap<Wire, WireState>,
) -> HashMap<Wire, WireState> {
    wires.iter_mut().for_each(|(wire, wire_state)| {
        if let Some(n) = wire.0.strip_prefix(input) {
            let n: usize = n.parse().expect("Should be a number");
            if value & (0x1 << n) > 0 {
                *wire_state = WireState::High;
            } else {
                *wire_state = WireState::Low;
            }
        }
    });
    wires
}

fn execute(gates: Vec<LogicGate>, mut wires: HashMap<Wire, WireState>) -> HashMap<Wire, WireState> {
    let (mut gates, mut ready_gates) = remove_available_gates(gates, &wires);
    while let Some(gate) = ready_gates.pop() {
        match gate {
            LogicGate::And(wire_a, wire_b, output_wire) => {
                let a_state = &wires[&wire_a];
                let b_state = &wires[&wire_b];
                if *a_state == WireState::High && *b_state == WireState::High {
                    *wires.get_mut(&output_wire).unwrap() = WireState::High;
                } else {
                    *wires.get_mut(&output_wire).unwrap() = WireState::Low;
                }
            }
            LogicGate::Or(wire_a, wire_b, output_wire) => {
                let a_state = &wires[&wire_a];
                let b_state = &wires[&wire_b];
                if *a_state == WireState::High || *b_state == WireState::High {
                    *wires.get_mut(&output_wire).unwrap() = WireState::High;
                } else {
                    *wires.get_mut(&output_wire).unwrap() = WireState::Low;
                }
            }
            LogicGate::Xor(wire_a, wire_b, output_wire) => {
                let a_state = &wires[&wire_a];
                let b_state = &wires[&wire_b];
                if *a_state != *b_state {
                    *wires.get_mut(&output_wire).unwrap() = WireState::High;
                } else {
                    *wires.get_mut(&output_wire).unwrap() = WireState::Low;
                }
            }
        };
        let (remaining_gates, new_ready_gates) = remove_available_gates(gates, &wires);
        gates = remaining_gates;
        ready_gates.extend(new_ready_gates);
    }
    wires
}

fn remove_available_gates(
    gates: Vec<LogicGate>,
    wires: &HashMap<Wire, WireState>,
) -> (Vec<LogicGate>, Vec<LogicGate>) {
    gates
        .into_iter()
        .fold((Vec::new(), Vec::new()), |mut acc, logic_gate| {
            match &logic_gate {
                LogicGate::Or(gate_a, gate_b, _) => {
                    if wires[gate_a] != WireState::Impedance
                        && wires[gate_b] != WireState::Impedance
                    {
                        acc.1.push(logic_gate);
                    } else {
                        acc.0.push(logic_gate);
                    }
                }
                LogicGate::And(gate_a, gate_b, _) => {
                    if wires[gate_a] != WireState::Impedance
                        && wires[gate_b] != WireState::Impedance
                    {
                        acc.1.push(logic_gate);
                    } else {
                        acc.0.push(logic_gate);
                    }
                }
                LogicGate::Xor(gate_a, gate_b, _) => {
                    if wires[gate_a] != WireState::Impedance
                        && wires[gate_b] != WireState::Impedance
                    {
                        acc.1.push(logic_gate);
                    } else {
                        acc.0.push(logic_gate);
                    }
                }
            }
            acc
        })
}

fn read_input(file: &str) -> Result<(Vec<LogicGate>, HashMap<Wire, WireState>)> {
    let data = read_to_string(file)?;
    let (wires, gates) = data.split_once("\n\n").ok_or_invalid_data()?;
    let mut wires = wires
        .split('\n')
        .map(|line| -> Result<(Wire, WireState)> {
            let (wire, state) = line.split_once(':').ok_or_invalid_data()?;
            Ok((
                Wire(wire.to_string()),
                if state.trim() == "1" {
                    WireState::High
                } else {
                    WireState::Low
                },
            ))
        })
        .collect::<Result<HashMap<Wire, WireState>>>()?;
    let gates = gates
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (gate, output) = line.split_once("->").ok_or_invalid_data()?;
            let output = output.trim().to_string();
            wires
                .entry(Wire(output.clone()))
                .or_insert(WireState::Impedance);
            let (wire_a, rest) = gate.split_once(" ").ok_or_invalid_data()?;
            let (gate, wire_b) = rest.split_once(" ").ok_or_invalid_data()?;
            Ok(match gate.trim() {
                "AND" => LogicGate::And(
                    Wire(wire_a.trim().to_string()),
                    Wire(wire_b.trim().to_string()),
                    Wire(output),
                ),
                "OR" => LogicGate::Or(
                    Wire(wire_a.trim().to_string()),
                    Wire(wire_b.trim().to_string()),
                    Wire(output),
                ),
                "XOR" => LogicGate::Xor(
                    Wire(wire_a.trim().to_string()),
                    Wire(wire_b.trim().to_string()),
                    Wire(output),
                ),
                other => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("`{other}` is not a valid gate"),
                    ))
                }
            })
        })
        .collect::<Result<Vec<LogicGate>>>()?;
    Ok((gates, wires))
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum WireState {
    High,
    Low,
    Impedance,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Wire(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LogicGate {
    Or(Wire, Wire, Wire),
    And(Wire, Wire, Wire),
    Xor(Wire, Wire, Wire),
}

impl LogicGate {
    pub fn output(&self) -> &Wire {
        match self {
            Self::Or(_, _, output) | Self::And(_, _, output) | Self::Xor(_, _, output) => output,
        }
    }

    pub fn inputs(&self) -> Vec<Wire> {
        match self {
            Self::Or(input_a, input_b, _)
            | Self::And(input_a, input_b, _)
            | Self::Xor(input_a, input_b, _) => {
                vec![input_a.clone(), input_b.clone()]
            }
        }
    }
}

trait OkOrInvalidData<T> {
    fn ok_or_invalid_data(self) -> Result<T>;
}

impl<T> OkOrInvalidData<T> for Option<T> {
    fn ok_or_invalid_data(self) -> Result<T> {
        self.ok_or(Error::new(std::io::ErrorKind::InvalidData, "Invalid data"))
    }
}
