use crate::states::State;

/// Represents an instruction for the Universal Machine
type Umi = u32;

///  Represent the components of a instruction within the instruction set.
///
/// <br>
///
/// # Fields
/// * `width`: Refers to size of the value within the instruction.
/// * `lsb`: Identifies where the value starts within the instruction in Bigendian format.
pub struct Field {
    width: u32,
    lsb: u32,
}

/// Represent the identifiers for every Opcode within the instruction set.
pub enum Opcode {
    CMov,
    SegLoad,
    SegStore,
    Add,
    Mul,
    Div,
    BNand,
    Halt,
    MapSeg,
    UnmapSeg,
    Output,
    Input,
    LoadProg,
    LoadVal,
}

impl From<u32> for Opcode {
    fn from(val: u32) -> Self {
        match val {
            0 => Opcode::CMov,
            1 => Opcode::SegLoad,
            2 => Opcode::SegStore,
            3 => Opcode::Add,
            4 => Opcode::Mul,
            5 => Opcode::Div,
            6 => Opcode::BNand,
            7 => Opcode::Halt,
            8 => Opcode::MapSeg,
            9 => Opcode::UnmapSeg,
            10 => Opcode::Output,
            11 => Opcode::Input,
            12 => Opcode::LoadProg,
            13 => Opcode::LoadVal,
            _ => panic!("Not valid Opcode"),
        }
    }
}

// Register at position A
static RA: Field = Field { width: 3, lsb: 6 };
// Register at position B
static RB: Field = Field { width: 3, lsb: 3 };
// Register at position C
static RC: Field = Field { width: 3, lsb: 0 };
// Register used only on the load value instruction
static RL: Field = Field { width: 3, lsb: 25 };
// Load value field
static VL: Field = Field { width: 25, lsb: 0 };
// Representation of the Opcode
static OP: Field = Field { width: 4, lsb: 28 };

pub fn run(state: &mut State) {
    loop {
        decode_inst(state)
    }
}

/// Creates a binary string that is used in conjunction with a
/// logical bitwise operator to identify or manipulate a single
/// bit (or set of bits) within another binary string.
///
/// <br>
///
/// # Argument:
/// `bits`: the size of the sequence of bits.
fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}

/// Gets the value of an specify field within the instruction
///
/// <br>
///
/// # Arguments:
/// * `field`: Representation of the value at some position within the instruction.
/// * `instruction`: 32-bit code word.
fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

/// Decodes the instruction set and calls the appropriate state method to handle the current
/// executing instruction.
///
/// <br>
///
/// # Argument:
/// * `state`: The struct representing the state of the UM.
fn decode_inst(state: &mut State) {
    let inst = state.get_instruction();
    match get(&OP, inst).into() {
        Opcode::CMov => state.cmov(get(&RA, inst), get(&RB, inst), get(&RC, inst)),
        Opcode::SegLoad => state.seg_load(get(&RA, inst), get(&RB, inst), get(&RC, inst)),
        Opcode::SegStore => state.seg_store(get(&RA, inst), get(&RB, inst), get(&RC, inst)),
        Opcode::Add => state.add(get(&RA, inst), get(&RB, inst), get(&RC, inst)),
        Opcode::Mul => state.mul(get(&RA, inst), get(&RB, inst), get(&RC, inst)),
        Opcode::Div => state.div(get(&RA, inst), get(&RB, inst), get(&RC, inst)),
        Opcode::BNand => state.b_nand(get(&RA, inst), get(&RB, inst), get(&RC, inst)),
        Opcode::Halt => state.halt(),
        Opcode::MapSeg => state.map_seg(get(&RB, inst), get(&RC, inst)),
        Opcode::UnmapSeg => state.unmap_seg(get(&RC, inst)),
        Opcode::Output => state.output(get(&RC, inst)),
        Opcode::Input => state.input(get(&RC, inst)),
        Opcode::LoadProg => state.load_prog(get(&RB, inst), get(&RC, inst)),
        Opcode::LoadVal => state.load_val(get(&RL, inst), get(&VL, inst)),
    }
}
