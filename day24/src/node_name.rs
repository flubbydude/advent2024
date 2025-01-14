const NODE_NAME_LEN: usize = 3;
pub type NodeName = [u8; NODE_NAME_LEN];

const NUM_Z_BITS: u8 = 64;

pub fn int_to_z_node_name(n: u8) -> NodeName {
    if n >= NUM_Z_BITS {
        panic!("Input integer out of range (0-63).");
    }

    let tens_digit = n / 10;
    let ones_digit = n % 10;

    [b'z', tens_digit + b'0', ones_digit + b'0']
}
