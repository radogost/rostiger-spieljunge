/// Performs an 8bit "add" arithmetic of 2 operands and returns the result, carry and half carry
pub(crate) fn add2_8bit(a: u8, b: u8) -> (u8, bool, bool) {
    let res = a.wrapping_add(a);
    let carry = res < a;
    let half_carry = (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10;
    (res, carry, half_carry)
}

/// Performs an 8bit "add" arithmetic of 3 operands and returns the result, carry and half carry
pub(crate) fn add3_8bit(a: u8, b: u8, c: u8) -> (u8, bool, bool) {
    let (temp, c1, hc1) = add2_8bit(a, b);
    let (res, c2, hc2) = add2_8bit(temp, c);
    (res, c1 | c2, hc1 | hc2)
}

/// Performs an 16bit "add" arithmetic of 2 operands and returns the result, carry and half carry
pub(crate) fn add2_16bit(a: u16, b: u16) -> (u16, bool, bool) {
    let res = a.wrapping_add(b);
    let carry = res < a;
    let half_carry = (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10;
    (res, carry, half_carry)
}

/// Performs an 8bit "sub" arithmetic of 2 operands and returns the result, carry and half carry
pub(crate) fn sub2_8bit(a: u8, b: u8) -> (u8, bool, bool) {
    let res = a.wrapping_sub(b);
    let carry = a < b;
    let half_carry = (a & 0xf) < (b & 0xf);
    (res, carry, half_carry)
}

/// Performs an 8bit "sub" arithmetic of 2 operands and returns the result, carry and half carry
pub(crate) fn sub3_8bit(a: u8, b: u8, c: u8) -> (u8, bool, bool) {
    let (temp, c1, hc1) = sub2_8bit(a, b);
    let (res, c2, hc2) = sub2_8bit(temp, c);
    (res, c1 | c2, hc1 | hc2)
}

/// Converts an 8-bit signed 2's complement to u16
pub(crate) fn signed_byte_to_u16(val: u8) -> u16 {
    if (val & 0x80) != 0 {
        // highest bit is 1, so it's a negative number
        // build 2's complement of it
        0xff00 | (val as u16)
    } else {
        val as u16
    }
}
