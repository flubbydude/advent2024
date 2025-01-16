const NODE_NAME_LEN: usize = 3;
pub type NodeName = [u8; NODE_NAME_LEN];

const NUM_Z_BITS: u8 = 64;

pub fn create_node_name(first_byte: u8, n: u8) -> NodeName {
    assert!(first_byte.is_ascii_alphabetic());
    if n >= NUM_Z_BITS {
        panic!("Input integer out of range (0-63).");
    }

    let tens_digit = n / 10;
    let ones_digit = n % 10;

    [first_byte, tens_digit + b'0', ones_digit + b'0']
}
