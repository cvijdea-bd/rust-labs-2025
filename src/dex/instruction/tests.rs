use super::*;

fn assert_helper(buffer: &[u8], expected_inst: Instruction, expected_size: usize) {
    let inst = Instruction::try_decode(buffer).unwrap();
    assert_eq!(inst, expected_inst);
    assert_eq!(inst.size_bytes(), expected_size);
}

#[test]
fn test_nop() {
    let buffer = [0x00, 0x00];
    assert_helper(&buffer, Instruction::Nop, 2);
}

#[test]
fn test_move() {
    let buffer = [0x01, 0x10];
    assert_helper(&buffer, Instruction::Move { dst: 0, src: 1 }, 2);
}

#[test]
fn test_move_from16() {
    let buffer = [0x02, 0x00, 0x19, 0x00];
    assert_helper(&buffer, Instruction::MoveFrom16 { dst: 0, src: 25 }, 4);
}

#[test]
fn test_move_wide_from16() {
    let buffer = [0x05, 0x16, 0x00, 0x00];
    assert_helper(&buffer, Instruction::MoveWideFrom16 { dst: 22, src: 0 }, 4);
}

#[test]
fn test_move_object() {
    let buffer = [0x07, 0x81];
    assert_helper(&buffer, Instruction::MoveObject { dst: 1, src: 8 }, 2);
}

#[test]
fn test_move_object_from16() {
    let buffer = [0x08, 0x01, 0x15, 0x00];
    assert_helper(
        &buffer,
        Instruction::MoveObjectFrom16 { dst: 1, src: 21 },
        4,
    );
}

#[test]
fn test_move_result() {
    let buffer = [0x0A, 0x00];
    assert_helper(&buffer, Instruction::MoveResult { dst: 0 }, 2);
}

#[test]
fn test_move_result_wide() {
    let buffer = [0x0B, 0x02];
    assert_helper(&buffer, Instruction::MoveResultWide { dst: 2 }, 2);
}

#[test]
fn test_move_result_object() {
    let buffer = [0x0C, 0x00];
    assert_helper(&buffer, Instruction::MoveResultObject { dst: 0 }, 2);
}

#[test]
fn test_move_exception() {
    let buffer = [0x0D, 0x19];
    assert_helper(&buffer, Instruction::MoveException { dst: 25 }, 2);
}

#[test]
fn test_return_void() {
    let buffer = [0x0E, 0x00];
    assert_helper(&buffer, Instruction::ReturnVoid, 2);
}

#[test]
fn test_return() {
    let buffer = [0x0F, 0x00];
    assert_helper(&buffer, Instruction::Return { value: 0 }, 2);
}

#[test]
fn test_return_wide() {
    let buffer = [0x10, 0x00];
    assert_helper(&buffer, Instruction::ReturnWide { value: 0 }, 2);
}

#[test]
fn test_return_object() {
    let buffer = [0x11, 0x00];
    assert_helper(&buffer, Instruction::ReturnObject { value: 0 }, 2);
}

#[test]
fn test_const4() {
    let buffer = [0x12, 0x21];
    assert_helper(&buffer, Instruction::Const4 { dst: 1, value: 2 }, 2);
}

#[test]
fn test_const16() {
    let buffer = [0x13, 0x00, 0x0A, 0x00];
    assert_helper(&buffer, Instruction::Const16 { dst: 0, value: 10 }, 4);
}

#[test]
fn test_const() {
    let buffer = [0x14, 0x00, 0x4E, 0x61, 0xBC, 0x00];
    assert_helper(
        &buffer,
        Instruction::Const {
            dst: 0,
            value: 12345678,
        },
        6,
    );
}

#[test]
fn test_const_high16() {
    let buffer = [0x15, 0x00, 0x20, 0x41];
    assert_helper(
        &buffer,
        Instruction::ConstHigh16 {
            dst: 0,
            value: 0x4120,
        },
        4,
    );
}

#[test]
fn test_const_wide16() {
    let buffer = [0x16, 0x00, 0x0A, 0x00];
    assert_helper(&buffer, Instruction::ConstWide16 { dst: 0, value: 10 }, 4);
}

#[test]
fn test_const_wide32() {
    let buffer = [0x17, 0x02, 0x4E, 0x61, 0xBC, 0x00];
    assert_helper(
        &buffer,
        Instruction::ConstWide32 {
            dst: 2,
            value: 12345678,
        },
        6,
    );
}

#[test]
fn test_const_wide() {
    let buffer = [0x18, 0x02, 0x87, 0x4B, 0x6B, 0x5D, 0x54, 0xDC, 0x2B, 0x00];
    assert_helper(
        &buffer,
        Instruction::ConstWide {
            dst: 2,
            value: 12345678901234567,
        },
        10,
    );
}

#[test]
fn test_const_wide_high16() {
    let buffer = [0x19, 0x00, 0x24, 0x40];
    assert_helper(
        &buffer,
        Instruction::ConstWideHigh16 {
            dst: 0,
            value: 0x4024,
        },
        4,
    );
}

#[test]
fn test_const_string() {
    let buffer = [0x1A, 0x08, 0x00, 0x00];
    assert_helper(
        &buffer,
        Instruction::ConstString {
            dst: 8,
            string_idx: 0,
        },
        4,
    );
}

#[test]
fn test_const_class() {
    let buffer = [0x1C, 0x00, 0x01, 0x00];
    assert_helper(
        &buffer,
        Instruction::ConstClass {
            dst: 0,
            type_idx: 1,
        },
        4,
    );
}

#[test]
fn test_monitor_enter() {
    let buffer = [0x1D, 0x03];
    assert_helper(&buffer, Instruction::MonitorEnter { reference: 3 }, 2);
}

#[test]
fn test_monitor_exit() {
    let buffer = [0x1E, 0x03];
    assert_helper(&buffer, Instruction::MonitorExit { reference: 3 }, 2);
}

#[test]
fn test_check_cast() {
    let buffer = [0x1F, 0x04, 0x01, 0x00];
    assert_helper(
        &buffer,
        Instruction::CheckCast {
            reference: 4,
            type_idx: 1,
        },
        4,
    );
}

#[test]
fn test_instance_of() {
    let buffer = [0x20, 0x40, 0x01, 0x00];
    assert_helper(
        &buffer,
        Instruction::InstanceOf {
            dst: 0,
            reference: 4,
            type_idx: 1,
        },
        4,
    );
}

#[test]
fn test_array_length() {
    let buffer = [0x21, 0x11];
    assert_helper(&buffer, Instruction::ArrayLength { dst: 1, array: 1 }, 2);
}

#[test]
fn test_new_instance() {
    let buffer = [0x22, 0x00, 0x15, 0x00];
    assert_helper(
        &buffer,
        Instruction::NewInstance {
            dst: 0,
            type_idx: 21,
        },
        4,
    );
}

#[test]
fn test_new_array() {
    let buffer = [0x23, 0x12, 0x25, 0x00];
    assert_helper(
        &buffer,
        Instruction::NewArray {
            dst: 2,
            size: 1,
            type_idx: 37,
        },
        4,
    );
}

#[test]
fn test_filled_new_array() {
    let buffer = [0x24, 0x20, 0x53, 0x0D, 0x00, 0x00];
    assert_helper(
        &buffer,
        Instruction::FilledNewArray {
            type_idx: 3411,
            args: [0, 0, 0, 0, 0],
            arg_cnt: 2,
        },
        6,
    );
}

#[test]
fn test_filled_new_array_range() {
    let buffer = [0x25, 0x03, 0x06, 0x00, 0x13, 0x00];
    assert_helper(
        &buffer,
        Instruction::FilledNewArrayRange {
            type_idx: 6,
            first_arg: 19,
            arg_cnt: 3,
        },
        6,
    );
}

#[test]
fn test_fill_array_data() {
    let buffer = [0x26, 0x06, 0x25, 0x00, 0x00, 0x00];
    assert_helper(
        &buffer,
        Instruction::FillArrayData {
            array: 6,
            offset: 37,
        },
        6,
    );
}

#[test]
fn test_throw() {
    let buffer = [0x27, 0x00];
    assert_helper(&buffer, Instruction::Throw { exception: 0 }, 2);
}

#[test]
fn test_goto() {
    let buffer = [0x28, 0xF0];
    assert_helper(&buffer, Instruction::Goto { offset: -16 }, 2);
}

#[test]
fn test_goto16() {
    let buffer = [0x29, 0x00, 0x0F, 0xFE];
    assert_helper(&buffer, Instruction::Goto16 { offset: -497 }, 4);
}

#[test]
fn test_packed_switch() {
    let buffer = [0x2B, 0x02, 0x0C, 0x00, 0x00, 0x00];
    assert_helper(
        &buffer,
        Instruction::PackedSwitch {
            value: 2,
            offset: 12,
        },
        6,
    );
}

#[test]
fn test_sparse_switch() {
    let buffer = [0x2C, 0x02, 0x0C, 0x00, 0x00, 0x00];
    assert_helper(
        &buffer,
        Instruction::SparseSwitch {
            value: 2,
            offset: 12,
        },
        6,
    );
}

#[test]
fn test_cmpl_float() {
    let buffer = [0x2D, 0x00, 0x06, 0x07];
    assert_helper(
        &buffer,
        Instruction::CmplFloat {
            dst: 0,
            src_a: 6,
            src_b: 7,
        },
        4,
    );
}

#[test]
fn test_cmpg_float() {
    let buffer = [0x2E, 0x00, 0x06, 0x07];
    assert_helper(
        &buffer,
        Instruction::CmpgFloat {
            dst: 0,
            src_a: 6,
            src_b: 7,
        },
        4,
    );
}

#[test]
fn test_cmpl_double() {
    let buffer = [0x2F, 0x19, 0x06, 0x08];
    assert_helper(
        &buffer,
        Instruction::CmplDouble {
            dst: 25,
            src_a: 6,
            src_b: 8,
        },
        4,
    );
}

#[test]
fn test_cmpg_double() {
    let buffer = [0x30, 0x00, 0x08, 0x0A];
    assert_helper(
        &buffer,
        Instruction::CmpgDouble {
            dst: 0,
            src_a: 8,
            src_b: 10,
        },
        4,
    );
}

#[test]
fn test_cmp_long() {
    let buffer = [0x31, 0x00, 0x02, 0x04];
    assert_helper(
        &buffer,
        Instruction::CmpLong {
            dst: 0,
            src_a: 2,
            src_b: 4,
        },
        4,
    );
}

#[test]
fn test_if_eq() {
    let buffer = [0x32, 0xB3, 0x66, 0x00];
    assert_helper(
        &buffer,
        Instruction::IfEq {
            a: 3,
            b: 11,
            offset: 102,
        },
        4,
    );
}

#[test]
fn test_if_ne() {
    let buffer = [0x33, 0xA3, 0x10, 0x00];
    assert_helper(
        &buffer,
        Instruction::IfNe {
            a: 3,
            b: 10,
            offset: 16,
        },
        4,
    );
}

#[test]
fn test_if_lt() {
    let buffer = [0x34, 0x32, 0xCB, 0xFF];
    assert_helper(
        &buffer,
        Instruction::IfLt {
            a: 2,
            b: 3,
            offset: -53,
        },
        4,
    );
}

#[test]
fn test_if_ge() {
    let buffer = [0x35, 0x10, 0x1B, 0x00];
    assert_helper(
        &buffer,
        Instruction::IfGe {
            a: 0,
            b: 1,
            offset: 27,
        },
        4,
    );
}

#[test]
fn test_if_gt() {
    let buffer = [0x36, 0x10, 0x1B, 0x00];
    assert_helper(
        &buffer,
        Instruction::IfGt {
            a: 0,
            b: 1,
            offset: 27,
        },
        4,
    );
}

#[test]
fn test_if_le() {
    let buffer = [0x37, 0x56, 0x0B, 0x00];
    assert_helper(
        &buffer,
        Instruction::IfLe {
            a: 6,
            b: 5,
            offset: 11,
        },
        4,
    );
}

#[test]
fn test_if_eqz() {
    let buffer = [0x38, 0x02, 0x19, 0x00];
    assert_helper(&buffer, Instruction::IfEqz { a: 2, offset: 25 }, 4);
}

#[test]
fn test_if_nez() {
    let buffer = [0x39, 0x02, 0x12, 0x00];
    assert_helper(&buffer, Instruction::IfNez { a: 2, offset: 18 }, 4);
}

#[test]
fn test_if_ltz() {
    let buffer = [0x3A, 0x00, 0x16, 0x00];
    assert_helper(&buffer, Instruction::IfLtz { a: 0, offset: 22 }, 4);
}

#[test]
fn test_if_gez() {
    let buffer = [0x3B, 0x00, 0x16, 0x00];
    assert_helper(&buffer, Instruction::IfGez { a: 0, offset: 22 }, 4);
}

#[test]
fn test_if_gtz() {
    let buffer = [0x3C, 0x00, 0x1D, 0x00];
    assert_helper(&buffer, Instruction::IfGtz { a: 0, offset: 29 }, 4);
}

#[test]
fn test_if_lez() {
    let buffer = [0x3D, 0x00, 0x1D, 0x00];
    assert_helper(&buffer, Instruction::IfLez { a: 0, offset: 29 }, 4);
}

#[test]
fn test_aget() {
    let buffer = [0x44, 0x07, 0x03, 0x06];
    assert_helper(
        &buffer,
        Instruction::Aget {
            src: 7,
            array: 3,
            index: 6,
        },
        4,
    );
}

#[test]
fn test_aget_wide() {
    let buffer = [0x45, 0x05, 0x01, 0x04];
    assert_helper(
        &buffer,
        Instruction::AgetWide {
            src: 5,
            array: 1,
            index: 4,
        },
        4,
    );
}

#[test]
fn test_aget_object() {
    let buffer = [0x46, 0x02, 0x02, 0x00];
    assert_helper(
        &buffer,
        Instruction::AgetObject {
            src: 2,
            array: 2,
            index: 0,
        },
        4,
    );
}

#[test]
fn test_aget_boolean() {
    let buffer = [0x47, 0x00, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::AgetBoolean {
            src: 0,
            array: 0,
            index: 1,
        },
        4,
    );
}

#[test]
fn test_aget_byte() {
    let buffer = [0x48, 0x00, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::AgetByte {
            src: 0,
            array: 0,
            index: 1,
        },
        4,
    );
}

#[test]
fn test_aget_char() {
    let buffer = [0x49, 0x05, 0x00, 0x03];
    assert_helper(
        &buffer,
        Instruction::AgetChar {
            src: 5,
            array: 0,
            index: 3,
        },
        4,
    );
}

#[test]
fn test_aget_short() {
    let buffer = [0x4A, 0x00, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::AgetShort {
            src: 0,
            array: 0,
            index: 1,
        },
        4,
    );
}

#[test]
fn test_aput() {
    let buffer = [0x4B, 0x00, 0x03, 0x05];
    assert_helper(
        &buffer,
        Instruction::Aput {
            dst: 0,
            array: 3,
            index: 5,
        },
        4,
    );
}

#[test]
fn test_aput_wide() {
    let buffer = [0x4C, 0x05, 0x01, 0x04];
    assert_helper(
        &buffer,
        Instruction::AputWide {
            dst: 5,
            array: 1,
            index: 4,
        },
        4,
    );
}

#[test]
fn test_aput_object() {
    let buffer = [0x4D, 0x02, 0x01, 0x00];
    assert_helper(
        &buffer,
        Instruction::AputObject {
            dst: 2,
            array: 1,
            index: 0,
        },
        4,
    );
}

#[test]
fn test_aput_boolean() {
    let buffer = [0x4E, 0x01, 0x00, 0x02];
    assert_helper(
        &buffer,
        Instruction::AputBoolean {
            dst: 1,
            array: 0,
            index: 2,
        },
        4,
    );
}

#[test]
fn test_aput_byte() {
    let buffer = [0x4F, 0x02, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::AputByte {
            dst: 2,
            array: 0,
            index: 1,
        },
        4,
    );
}

#[test]
fn test_aput_char() {
    let buffer = [0x50, 0x03, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::AputChar {
            dst: 3,
            array: 0,
            index: 1,
        },
        4,
    );
}

#[test]
fn test_aput_short() {
    let buffer = [0x51, 0x02, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::AputShort {
            dst: 2,
            array: 0,
            index: 1,
        },
        4,
    );
}

#[test]
fn test_iget() {
    let buffer = [0x52, 0x10, 0x03, 0x00];
    assert_helper(
        &buffer,
        Instruction::Iget {
            src: 0,
            object: 1,
            field_idx: 3,
        },
        4,
    );
}

#[test]
fn test_iget_wide() {
    let buffer = [0x53, 0x20, 0x04, 0x00];
    assert_helper(
        &buffer,
        Instruction::IgetWide {
            src: 0,
            object: 2,
            field_idx: 4,
        },
        4,
    );
}

#[test]
fn test_iget_object() {
    let buffer = [0x54, 0x10, 0x02, 0x00];
    assert_helper(
        &buffer,
        Instruction::IgetObject {
            src: 0,
            object: 1,
            field_idx: 2,
        },
        4,
    );
}

#[test]
fn test_iget_boolean() {
    let buffer = [0x55, 0xFC, 0x00, 0x00];
    assert_helper(
        &buffer,
        Instruction::IgetBoolean {
            src: 12,
            object: 15,
            field_idx: 0,
        },
        4,
    );
}

#[test]
fn test_iget_byte() {
    let buffer = [0x56, 0x32, 0x01, 0x00];
    assert_helper(
        &buffer,
        Instruction::IgetByte {
            src: 2,
            object: 3,
            field_idx: 1,
        },
        4,
    );
}

#[test]
fn test_iget_char() {
    let buffer = [0x57, 0x20, 0x03, 0x00];
    assert_helper(
        &buffer,
        Instruction::IgetChar {
            src: 0,
            object: 2,
            field_idx: 3,
        },
        4,
    );
}

#[test]
fn test_iget_short() {
    let buffer = [0x58, 0x30, 0x08, 0x00];
    assert_helper(
        &buffer,
        Instruction::IgetShort {
            src: 0,
            object: 3,
            field_idx: 8,
        },
        4,
    );
}

#[test]
fn test_iput() {
    let buffer = [0x59, 0x20, 0x02, 0x00];
    assert_helper(
        &buffer,
        Instruction::Iput {
            dst: 0,
            object: 2,
            field_idx: 2,
        },
        4,
    );
}

#[test]
fn test_iput_wide() {
    let buffer = [0x5A, 0x20, 0x00, 0x00];
    assert_helper(
        &buffer,
        Instruction::IputWide {
            dst: 0,
            object: 2,
            field_idx: 0,
        },
        4,
    );
}

#[test]
fn test_iput_object() {
    let buffer = [0x5B, 0x20, 0x00, 0x00];
    assert_helper(
        &buffer,
        Instruction::IputObject {
            dst: 0,
            object: 2,
            field_idx: 0,
        },
        4,
    );
}

#[test]
fn test_iput_boolean() {
    let buffer = [0x5C, 0x30, 0x00, 0x00];
    assert_helper(
        &buffer,
        Instruction::IputBoolean {
            dst: 0,
            object: 3,
            field_idx: 0,
        },
        4,
    );
}

#[test]
fn test_iput_byte() {
    let buffer = [0x5D, 0x20, 0x01, 0x00];
    assert_helper(
        &buffer,
        Instruction::IputByte {
            dst: 0,
            object: 2,
            field_idx: 1,
        },
        4,
    );
}

#[test]
fn test_iput_char() {
    let buffer = [0x5E, 0x20, 0x03, 0x00];
    assert_helper(
        &buffer,
        Instruction::IputChar {
            dst: 0,
            object: 2,
            field_idx: 3,
        },
        4,
    );
}

#[test]
fn test_iput_short() {
    let buffer = [0x5F, 0x21, 0x08, 0x00];
    assert_helper(
        &buffer,
        Instruction::IputShort {
            dst: 1,
            object: 2,
            field_idx: 8,
        },
        4,
    );
}

#[test]
fn test_sget() {
    let buffer = [0x60, 0x00, 0x07, 0x00];
    assert_helper(
        &buffer,
        Instruction::Sget {
            src: 0,
            field_idx: 7,
        },
        4,
    );
}

#[test]
fn test_sget_wide() {
    let buffer = [0x61, 0x00, 0x05, 0x00];
    assert_helper(
        &buffer,
        Instruction::SgetWide {
            src: 0,
            field_idx: 5,
        },
        4,
    );
}

#[test]
fn test_sget_object() {
    let buffer = [0x62, 0x01, 0x0C, 0x00];
    assert_helper(
        &buffer,
        Instruction::SgetObject {
            src: 1,
            field_idx: 12,
        },
        4,
    );
}

#[test]
fn test_sget_boolean() {
    let buffer = [0x63, 0x00, 0x0C, 0x00];
    assert_helper(
        &buffer,
        Instruction::SgetBoolean {
            src: 0,
            field_idx: 12,
        },
        4,
    );
}

#[test]
fn test_sget_byte() {
    let buffer = [0x64, 0x00, 0x02, 0x00];
    assert_helper(
        &buffer,
        Instruction::SgetByte {
            src: 0,
            field_idx: 2,
        },
        4,
    );
}

#[test]
fn test_sget_char() {
    let buffer = [0x65, 0x00, 0x07, 0x00];
    assert_helper(
        &buffer,
        Instruction::SgetChar {
            src: 0,
            field_idx: 7,
        },
        4,
    );
}

#[test]
fn test_sget_short() {
    let buffer = [0x66, 0x00, 0x0B, 0x00];
    assert_helper(
        &buffer,
        Instruction::SgetShort {
            src: 0,
            field_idx: 11,
        },
        4,
    );
}

#[test]
fn test_sput() {
    let buffer = [0x67, 0x00, 0x01, 0x00];
    assert_helper(
        &buffer,
        Instruction::Sput {
            dst: 0,
            field_idx: 1,
        },
        4,
    );
}

#[test]
fn test_sput_wide() {
    let buffer = [0x68, 0x00, 0x05, 0x00];
    assert_helper(
        &buffer,
        Instruction::SputWide {
            dst: 0,
            field_idx: 5,
        },
        4,
    );
}

#[test]
fn test_sput_object() {
    let buffer = [0x69, 0x00, 0x0C, 0x00];
    assert_helper(
        &buffer,
        Instruction::SputObject {
            dst: 0,
            field_idx: 12,
        },
        4,
    );
}

#[test]
fn test_sput_boolean() {
    let buffer = [0x6A, 0x00, 0x03, 0x00];
    assert_helper(
        &buffer,
        Instruction::SputBoolean {
            dst: 0,
            field_idx: 3,
        },
        4,
    );
}

#[test]
fn test_sput_byte() {
    let buffer = [0x6B, 0x00, 0x02, 0x00];
    assert_helper(
        &buffer,
        Instruction::SputByte {
            dst: 0,
            field_idx: 2,
        },
        4,
    );
}

#[test]
fn test_sput_char() {
    let buffer = [0x6C, 0x01, 0x07, 0x00];
    assert_helper(
        &buffer,
        Instruction::SputChar {
            dst: 1,
            field_idx: 7,
        },
        4,
    );
}

#[test]
fn test_sput_short() {
    let buffer = [0x6D, 0x00, 0x0B, 0x00];
    assert_helper(
        &buffer,
        Instruction::SputShort {
            dst: 0,
            field_idx: 11,
        },
        4,
    );
}

#[test]
fn test_invoke_virtual() {
    let buffer = [0x6E, 0x54, 0x06, 0x00, 0x32, 0x10];
    assert_helper(
        &buffer,
        Instruction::InvokeVirtual {
            method_idx: 6,
            args: [2, 3, 0, 1, 4],
            arg_cnt: 5,
        },
        6,
    );
}

#[test]
fn test_invoke_super() {
    let buffer = [0x6F, 0x45, 0xA6, 0x01, 0x32, 0x10];
    assert_helper(
        &buffer,
        Instruction::InvokeSuper {
            method_idx: 422,
            args: [2, 3, 0, 1, 5],
            arg_cnt: 4,
        },
        6,
    );
}

#[test]
fn test_invoke_direct() {
    let buffer = [0x70, 0x45, 0x08, 0x00, 0x32, 0x10];
    assert_helper(
        &buffer,
        Instruction::InvokeDirect {
            method_idx: 8,
            args: [2, 3, 0, 1, 5],
            arg_cnt: 4,
        },
        6,
    );
}

#[test]
fn test_invoke_static() {
    let buffer = [0x71, 0x45, 0x34, 0x00, 0x32, 0x10];
    assert_helper(
        &buffer,
        Instruction::InvokeStatic {
            method_idx: 52,
            args: [2, 3, 0, 1, 5],
            arg_cnt: 4,
        },
        6,
    );
}

#[test]
fn test_invoke_interface() {
    let buffer = [0x72, 0x45, 0x21, 0x02, 0x32, 0x10];
    assert_helper(
        &buffer,
        Instruction::InvokeInterface {
            method_idx: 545,
            args: [2, 3, 0, 1, 5],
            arg_cnt: 4,
        },
        6,
    );
}

#[test]
fn test_invoke_virtual_range() {
    let buffer = [0x74, 0x03, 0x06, 0x00, 0x13, 0x00];
    assert_helper(
        &buffer,
        Instruction::InvokeVirtualRange {
            method_idx: 6,
            first_arg: 19,
            arg_cnt: 3,
        },
        6,
    );
}

#[test]
fn test_invoke_super_range() {
    let buffer = [0x75, 0x03, 0xA6, 0x01, 0x01, 0x00];
    assert_helper(
        &buffer,
        Instruction::InvokeSuperRange {
            method_idx: 422,
            first_arg: 1,
            arg_cnt: 3,
        },
        6,
    );
}

#[test]
fn test_invoke_direct_range() {
    let buffer = [0x76, 0x03, 0x3A, 0x00, 0x13, 0x00];
    assert_helper(
        &buffer,
        Instruction::InvokeDirectRange {
            method_idx: 58,
            first_arg: 19,
            arg_cnt: 3,
        },
        6,
    );
}

#[test]
fn test_invoke_static_range() {
    let buffer = [0x77, 0x03, 0x3A, 0x00, 0x13, 0x00];
    assert_helper(
        &buffer,
        Instruction::InvokeStaticRange {
            method_idx: 58,
            first_arg: 19,
            arg_cnt: 3,
        },
        6,
    );
}

#[test]
fn test_invoke_interface_range() {
    let buffer = [0x78, 0x04, 0x21, 0x02, 0x01, 0x00];
    assert_helper(
        &buffer,
        Instruction::InvokeInterfaceRange {
            method_idx: 545,
            first_arg: 1,
            arg_cnt: 4,
        },
        6,
    );
}

#[test]
fn test_neg_int() {
    let buffer = [0x7B, 0x01];
    assert_helper(&buffer, Instruction::NegInt { dst: 1, src: 0 }, 2);
}

#[test]
fn test_not_int() {
    let buffer = [0x7C, 0x01];
    assert_helper(&buffer, Instruction::NotInt { dst: 1, src: 0 }, 2);
}

#[test]
fn test_neg_long() {
    let buffer = [0x7D, 0x02];
    assert_helper(&buffer, Instruction::NegLong { dst: 2, src: 0 }, 2);
}

#[test]
fn test_not_long() {
    let buffer = [0x7E, 0x02];
    assert_helper(&buffer, Instruction::NotLong { dst: 2, src: 0 }, 2);
}

#[test]
fn test_neg_float() {
    let buffer = [0x7F, 0x01];
    assert_helper(&buffer, Instruction::NegFloat { dst: 1, src: 0 }, 2);
}

#[test]
fn test_neg_double() {
    let buffer = [0x80, 0x02];
    assert_helper(&buffer, Instruction::NegDouble { dst: 2, src: 0 }, 2);
}

#[test]
fn test_int_to_long() {
    let buffer = [0x81, 0x06];
    assert_helper(&buffer, Instruction::IntToLong { dst: 6, src: 0 }, 2);
}

#[test]
fn test_int_to_float() {
    let buffer = [0x82, 0x06];
    assert_helper(&buffer, Instruction::IntToFloat { dst: 6, src: 0 }, 2);
}

#[test]
fn test_int_to_double() {
    let buffer = [0x83, 0x06];
    assert_helper(&buffer, Instruction::IntToDouble { dst: 6, src: 0 }, 2);
}

#[test]
fn test_long_to_int() {
    let buffer = [0x84, 0x24];
    assert_helper(&buffer, Instruction::LongToInt { dst: 4, src: 2 }, 2);
}

#[test]
fn test_long_to_float() {
    let buffer = [0x85, 0x10];
    assert_helper(&buffer, Instruction::LongToFloat { dst: 0, src: 1 }, 2);
}

#[test]
fn test_long_to_double() {
    let buffer = [0x86, 0x10];
    assert_helper(&buffer, Instruction::LongToDouble { dst: 0, src: 1 }, 2);
}

#[test]
fn test_float_to_int() {
    let buffer = [0x87, 0x30];
    assert_helper(&buffer, Instruction::FloatToInt { dst: 0, src: 3 }, 2);
}

#[test]
fn test_float_to_long() {
    let buffer = [0x88, 0x30];
    assert_helper(&buffer, Instruction::FloatToLong { dst: 0, src: 3 }, 2);
}

#[test]
fn test_float_to_double() {
    let buffer = [0x89, 0x30];
    assert_helper(&buffer, Instruction::FloatToDouble { dst: 0, src: 3 }, 2);
}

#[test]
fn test_double_to_int() {
    let buffer = [0x8A, 0x40];
    assert_helper(&buffer, Instruction::DoubleToInt { dst: 0, src: 4 }, 2);
}

#[test]
fn test_double_to_long() {
    let buffer = [0x8B, 0x40];
    assert_helper(&buffer, Instruction::DoubleToLong { dst: 0, src: 4 }, 2);
}

#[test]
fn test_double_to_float() {
    let buffer = [0x8C, 0x40];
    assert_helper(&buffer, Instruction::DoubleToFloat { dst: 0, src: 4 }, 2);
}

#[test]
fn test_int_to_byte() {
    let buffer = [0x8D, 0x00];
    assert_helper(&buffer, Instruction::IntToByte { dst: 0, src: 0 }, 2);
}

#[test]
fn test_int_to_char() {
    let buffer = [0x8E, 0x33];
    assert_helper(&buffer, Instruction::IntToChar { dst: 3, src: 3 }, 2);
}

#[test]
fn test_int_to_short() {
    let buffer = [0x8F, 0x00];
    assert_helper(&buffer, Instruction::IntToShort { dst: 0, src: 0 }, 2);
}

#[test]
fn test_add_int() {
    let buffer = [0x90, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::AddInt {
            dst: 0,
            src_a: 2,
            src_b: 3,
        },
        4,
    );
}

#[test]
fn test_sub_int() {
    let buffer = [0x91, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::SubInt {
            dst: 0,
            src_a: 2,
            src_b: 3,
        },
        4,
    );
}

#[test]
fn test_mul_int() {
    let buffer = [0x92, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::MulInt {
            dst: 0,
            src_a: 2,
            src_b: 3,
        },
        4,
    );
}

#[test]
fn test_div_int() {
    let buffer = [0x93, 0x03, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::DivInt {
            dst: 3,
            src_a: 0,
            src_b: 1,
        },
        4,
    );
}

#[test]
fn test_rem_int() {
    let buffer = [0x94, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::RemInt {
            dst: 0,
            src_a: 2,
            src_b: 3,
        },
        4,
    );
}

#[test]
fn test_and_int() {
    let buffer = [0x95, 0x03, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::AndInt {
            dst: 3,
            src_a: 0,
            src_b: 1,
        },
        4,
    );
}

#[test]
fn test_or_int() {
    let buffer = [0x96, 0x03, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::OrInt {
            dst: 3,
            src_a: 0,
            src_b: 1,
        },
        4,
    );
}

#[test]
fn test_xor_int() {
    let buffer = [0x97, 0x03, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::XorInt {
            dst: 3,
            src_a: 0,
            src_b: 1,
        },
        4,
    );
}

#[test]
fn test_shl_int() {
    let buffer = [0x98, 0x02, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::ShlInt {
            dst: 2,
            src_a: 0,
            src_b: 1,
        },
        4,
    );
}

#[test]
fn test_shr_int() {
    let buffer = [0x99, 0x02, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::ShrInt {
            dst: 2,
            src_a: 0,
            src_b: 1,
        },
        4,
    );
}

#[test]
fn test_ushr_int() {
    let buffer = [0x9A, 0x02, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::UShrInt {
            dst: 2,
            src_a: 0,
            src_b: 1,
        },
        4,
    );
}

#[test]
fn test_add_long() {
    let buffer = [0x9B, 0x00, 0x03, 0x05];
    assert_helper(
        &buffer,
        Instruction::AddLong {
            dst: 0,
            src_a: 3,
            src_b: 5,
        },
        4,
    );
}

#[test]
fn test_sub_long() {
    let buffer = [0x9C, 0x00, 0x03, 0x05];
    assert_helper(
        &buffer,
        Instruction::SubLong {
            dst: 0,
            src_a: 3,
            src_b: 5,
        },
        4,
    );
}

#[test]
fn test_mul_long() {
    let buffer = [0x9D, 0x00, 0x03, 0x05];
    assert_helper(
        &buffer,
        Instruction::MulLong {
            dst: 0,
            src_a: 3,
            src_b: 5,
        },
        4,
    );
}

#[test]
fn test_div_long() {
    let buffer = [0x9E, 0x06, 0x00, 0x02];
    assert_helper(
        &buffer,
        Instruction::DivLong {
            dst: 6,
            src_a: 0,
            src_b: 2,
        },
        4,
    );
}

#[test]
fn test_rem_long() {
    let buffer = [0x9F, 0x06, 0x00, 0x02];
    assert_helper(
        &buffer,
        Instruction::RemLong {
            dst: 6,
            src_a: 0,
            src_b: 2,
        },
        4,
    );
}

#[test]
fn test_and_long() {
    let buffer = [0xA0, 0x06, 0x00, 0x02];
    assert_helper(
        &buffer,
        Instruction::AndLong {
            dst: 6,
            src_a: 0,
            src_b: 2,
        },
        4,
    );
}

#[test]
fn test_or_long() {
    let buffer = [0xA1, 0x06, 0x00, 0x02];
    assert_helper(
        &buffer,
        Instruction::OrLong {
            dst: 6,
            src_a: 0,
            src_b: 2,
        },
        4,
    );
}

#[test]
fn test_xor_long() {
    let buffer = [0xA2, 0x06, 0x00, 0x02];
    assert_helper(
        &buffer,
        Instruction::XorLong {
            dst: 6,
            src_a: 0,
            src_b: 2,
        },
        4,
    );
}

#[test]
fn test_shl_long() {
    let buffer = [0xA3, 0x02, 0x00, 0x04];
    assert_helper(
        &buffer,
        Instruction::ShlLong {
            dst: 2,
            src_a: 0,
            src_b: 4,
        },
        4,
    );
}

#[test]
fn test_shr_long() {
    let buffer = [0xA4, 0x02, 0x00, 0x04];
    assert_helper(
        &buffer,
        Instruction::ShrLong {
            dst: 2,
            src_a: 0,
            src_b: 4,
        },
        4,
    );
}

#[test]
fn test_ushr_long() {
    let buffer = [0xA5, 0x02, 0x00, 0x04];
    assert_helper(
        &buffer,
        Instruction::UShrLong {
            dst: 2,
            src_a: 0,
            src_b: 4,
        },
        4,
    );
}

#[test]
fn test_add_float() {
    let buffer = [0xA6, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::AddFloat {
            dst: 0,
            src_a: 2,
            src_b: 3,
        },
        4,
    );
}

#[test]
fn test_sub_float() {
    let buffer = [0xA7, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::SubFloat {
            dst: 0,
            src_a: 2,
            src_b: 3,
        },
        4,
    );
}

#[test]
fn test_mul_float() {
    let buffer = [0xA8, 0x03, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::MulFloat {
            dst: 3,
            src_a: 0,
            src_b: 1,
        },
        4,
    );
}

#[test]
fn test_div_float() {
    let buffer = [0xA9, 0x03, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::DivFloat {
            dst: 3,
            src_a: 0,
            src_b: 1,
        },
        4,
    );
}

#[test]
fn test_rem_float() {
    let buffer = [0xAA, 0x03, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::RemFloat {
            dst: 3,
            src_a: 0,
            src_b: 1,
        },
        4,
    );
}

#[test]
fn test_add_double() {
    let buffer = [0xAB, 0x00, 0x03, 0x05];
    assert_helper(
        &buffer,
        Instruction::AddDouble {
            dst: 0,
            src_a: 3,
            src_b: 5,
        },
        4,
    );
}

#[test]
fn test_sub_double() {
    let buffer = [0xAC, 0x00, 0x03, 0x05];
    assert_helper(
        &buffer,
        Instruction::SubDouble {
            dst: 0,
            src_a: 3,
            src_b: 5,
        },
        4,
    );
}

#[test]
fn test_mul_double() {
    let buffer = [0xAD, 0x06, 0x00, 0x02];
    assert_helper(
        &buffer,
        Instruction::MulDouble {
            dst: 6,
            src_a: 0,
            src_b: 2,
        },
        4,
    );
}

#[test]
fn test_div_double() {
    let buffer = [0xAE, 0x06, 0x00, 0x02];
    assert_helper(
        &buffer,
        Instruction::DivDouble {
            dst: 6,
            src_a: 0,
            src_b: 2,
        },
        4,
    );
}

#[test]
fn test_rem_double() {
    let buffer = [0xAF, 0x06, 0x00, 0x02];
    assert_helper(
        &buffer,
        Instruction::RemDouble {
            dst: 6,
            src_a: 0,
            src_b: 2,
        },
        4,
    );
}

#[test]
fn test_add_int_2addr() {
    let buffer = [0xB0, 0x10];
    assert_helper(&buffer, Instruction::AddInt2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_sub_int_2addr() {
    let buffer = [0xB1, 0x40];
    assert_helper(&buffer, Instruction::SubInt2Addr { dst: 0, src: 4 }, 2);
}

#[test]
fn test_mul_int_2addr() {
    let buffer = [0xB2, 0x10];
    assert_helper(&buffer, Instruction::MulInt2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_div_int_2addr() {
    let buffer = [0xB3, 0x10];
    assert_helper(&buffer, Instruction::DivInt2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_rem_int_2addr() {
    let buffer = [0xB4, 0x10];
    assert_helper(&buffer, Instruction::RemInt2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_and_int_2addr() {
    let buffer = [0xB5, 0x10];
    assert_helper(&buffer, Instruction::AndInt2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_or_int_2addr() {
    let buffer = [0xB6, 0x10];
    assert_helper(&buffer, Instruction::OrInt2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_xor_int_2addr() {
    let buffer = [0xB7, 0x10];
    assert_helper(&buffer, Instruction::XorInt2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_shl_int_2addr() {
    let buffer = [0xB8, 0x10];
    assert_helper(&buffer, Instruction::ShlInt2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_shr_int_2addr() {
    let buffer = [0xB9, 0x10];
    assert_helper(&buffer, Instruction::ShrInt2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_ushr_int_2addr() {
    let buffer = [0xBA, 0x10];
    assert_helper(&buffer, Instruction::UShrInt2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_add_long_2addr() {
    let buffer = [0xBB, 0x20];
    assert_helper(&buffer, Instruction::AddLong2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_sub_long_2addr() {
    let buffer = [0xBC, 0x70];
    assert_helper(&buffer, Instruction::SubLong2Addr { dst: 0, src: 7 }, 2);
}

#[test]
fn test_mul_long_2addr() {
    let buffer = [0xBD, 0x70];
    assert_helper(&buffer, Instruction::MulLong2Addr { dst: 0, src: 7 }, 2);
}

#[test]
fn test_div_long_2addr() {
    let buffer = [0xBE, 0x20];
    assert_helper(&buffer, Instruction::DivLong2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_rem_long_2addr() {
    let buffer = [0xBF, 0x20];
    assert_helper(&buffer, Instruction::RemLong2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_and_long_2addr() {
    let buffer = [0xC0, 0x20];
    assert_helper(&buffer, Instruction::AndLong2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_or_long_2addr() {
    let buffer = [0xC1, 0x20];
    assert_helper(&buffer, Instruction::OrLong2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_xor_long_2addr() {
    let buffer = [0xC2, 0x20];
    assert_helper(&buffer, Instruction::XorLong2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_shl_long_2addr() {
    let buffer = [0xC3, 0x20];
    assert_helper(&buffer, Instruction::ShlLong2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_shr_long_2addr() {
    let buffer = [0xC4, 0x20];
    assert_helper(&buffer, Instruction::ShrLong2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_ushr_long_2addr() {
    let buffer = [0xC5, 0x20];
    assert_helper(&buffer, Instruction::UShrLong2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_add_float_2addr() {
    let buffer = [0xC6, 0x40];
    assert_helper(&buffer, Instruction::AddFloat2Addr { dst: 0, src: 4 }, 2);
}

#[test]
fn test_sub_float_2addr() {
    let buffer = [0xC7, 0x40];
    assert_helper(&buffer, Instruction::SubFloat2Addr { dst: 0, src: 4 }, 2);
}

#[test]
fn test_mul_float_2addr() {
    let buffer = [0xC8, 0x10];
    assert_helper(&buffer, Instruction::MulFloat2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_div_float_2addr() {
    let buffer = [0xC9, 0x10];
    assert_helper(&buffer, Instruction::DivFloat2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_rem_float_2addr() {
    let buffer = [0xCA, 0x10];
    assert_helper(&buffer, Instruction::RemFloat2Addr { dst: 0, src: 1 }, 2);
}

#[test]
fn test_add_double_2addr() {
    let buffer = [0xCB, 0x70];
    assert_helper(&buffer, Instruction::AddDouble2Addr { dst: 0, src: 7 }, 2);
}

#[test]
fn test_sub_double_2addr() {
    let buffer = [0xCC, 0x70];
    assert_helper(&buffer, Instruction::SubDouble2Addr { dst: 0, src: 7 }, 2);
}

#[test]
fn test_mul_double_2addr() {
    let buffer = [0xCD, 0x20];
    assert_helper(&buffer, Instruction::MulDouble2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_div_double_2addr() {
    let buffer = [0xCE, 0x20];
    assert_helper(&buffer, Instruction::DivDouble2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_rem_double_2addr() {
    let buffer = [0xCF, 0x20];
    assert_helper(&buffer, Instruction::RemDouble2Addr { dst: 0, src: 2 }, 2);
}

#[test]
fn test_add_int_lit16() {
    let buffer = [0xD0, 0x01, 0xD2, 0x04];
    assert_helper(
        &buffer,
        Instruction::AddIntLit16 {
            dst: 1,
            src: 0,
            value: 1234,
        },
        4,
    );
}

#[test]
fn test_sub_int_lit16() {
    let buffer = [0xD1, 0x01, 0xD2, 0x04];
    assert_helper(
        &buffer,
        Instruction::RsubInt {
            dst: 1,
            src: 0,
            value: 1234,
        },
        4,
    );
}

#[test]
fn test_mul_int_lit16() {
    let buffer = [0xD2, 0x01, 0xD2, 0x04];
    assert_helper(
        &buffer,
        Instruction::MulIntLit16 {
            dst: 1,
            src: 0,
            value: 1234,
        },
        4,
    );
}

#[test]
fn test_div_int_lit16() {
    let buffer = [0xD3, 0x01, 0xD2, 0x04];
    assert_helper(
        &buffer,
        Instruction::DivIntLit16 {
            dst: 1,
            src: 0,
            value: 1234,
        },
        4,
    );
}

#[test]
fn test_rem_int_lit16() {
    let buffer = [0xD4, 0x01, 0xD2, 0x04];
    assert_helper(
        &buffer,
        Instruction::RemIntLit16 {
            dst: 1,
            src: 0,
            value: 1234,
        },
        4,
    );
}

#[test]
fn test_and_int_lit16() {
    let buffer = [0xD5, 0x01, 0xD2, 0x04];
    assert_helper(
        &buffer,
        Instruction::AndIntLit16 {
            dst: 1,
            src: 0,
            value: 1234,
        },
        4,
    );
}

#[test]
fn test_or_int_lit16() {
    let buffer = [0xD6, 0x01, 0xD2, 0x04];
    assert_helper(
        &buffer,
        Instruction::OrIntLit16 {
            dst: 1,
            src: 0,
            value: 1234,
        },
        4,
    );
}

#[test]
fn test_xor_int_lit16() {
    let buffer = [0xD7, 0x01, 0xD2, 0x04];
    assert_helper(
        &buffer,
        Instruction::XorIntLit16 {
            dst: 1,
            src: 0,
            value: 1234,
        },
        4,
    );
}

#[test]
fn test_add_int_lit8() {
    let buffer = [0xD8, 0x00, 0x02, 0x01];
    assert_helper(
        &buffer,
        Instruction::AddIntLit8 {
            dst: 0,
            src: 2,
            value: 1,
        },
        4,
    );
}

#[test]
fn test_sub_int_lit8() {
    let buffer = [0xD9, 0x00, 0x02, 0x01];
    assert_helper(
        &buffer,
        Instruction::RsubIntLit8 {
            dst: 0,
            src: 2,
            value: 1,
        },
        4,
    );
}

#[test]
fn test_mul_int_lit8() {
    let buffer = [0xDA, 0x00, 0x00, 0x02];
    assert_helper(
        &buffer,
        Instruction::MulIntLit8 {
            dst: 0,
            src: 0,
            value: 2,
        },
        4,
    );
}

#[test]
fn test_div_int_lit8() {
    let buffer = [0xDB, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::DivIntLit8 {
            dst: 0,
            src: 2,
            value: 3,
        },
        4,
    );
}

#[test]
fn test_rem_int_lit8() {
    let buffer = [0xDC, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::RemIntLit8 {
            dst: 0,
            src: 2,
            value: 3,
        },
        4,
    );
}

#[test]
fn test_and_int_lit8() {
    let buffer = [0xDD, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::AndIntLit8 {
            dst: 0,
            src: 2,
            value: 3,
        },
        4,
    );
}

#[test]
fn test_or_int_lit8() {
    let buffer = [0xDE, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::OrIntLit8 {
            dst: 0,
            src: 2,
            value: 3,
        },
        4,
    );
}

#[test]
fn test_xor_int_lit8() {
    let buffer = [0xDF, 0x00, 0x02, 0x03];
    assert_helper(
        &buffer,
        Instruction::XorIntLit8 {
            dst: 0,
            src: 2,
            value: 3,
        },
        4,
    );
}

#[test]
fn test_shl_int_lit8() {
    let buffer = [0xE0, 0x01, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::ShlIntLit8 {
            dst: 1,
            src: 0,
            value: 1,
        },
        4,
    );
}

#[test]
fn test_shr_int_lit8() {
    let buffer = [0xE1, 0x01, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::ShrIntLit8 {
            dst: 1,
            src: 0,
            value: 1,
        },
        4,
    );
}

#[test]
fn test_ushr_int_lit8() {
    let buffer = [0xE2, 0x01, 0x00, 0x01];
    assert_helper(
        &buffer,
        Instruction::UShrIntLit8 {
            dst: 1,
            src: 0,
            value: 1,
        },
        4,
    );
}

// #[test]
// fn test_invoke_polymorphic() {
//     let buffer = [0xFA, 0x40, 0x21, 0x02, 0x31, 0x54];
//     assert_helper(
//         &buffer,
//         Instruction::InvokePolymorphic {
//             method_idx: 0x0221,
//             proto_idx: 0x5431,
//             args: [0, 1, 2, 3, 4],
//             arg_cnt: 4,
//         },
//         6,
//     );
// }

// #[test]
// fn test_invoke_polymorphic_range() {
//     let buffer = [0xFB, 0x40, 0x21, 0x02, 0x31, 0x54];
//     assert_helper(
//         &buffer,
//         Instruction::InvokePolymorphicRange {
//             method_idx: 0x0221,
//             proto_idx: 0x5431,
//             first_arg: 0x5431,
//             arg_cnt: 4,
//         },
//         6,
//     );
// }

#[test]
fn test_invoke_custom() {
    let buffer = [0xFC, 0x45, 0x21, 0x02, 0x32, 0x10];
    assert_helper(
        &buffer,
        Instruction::InvokeCustom {
            call_site_idx: 0x221,
            args: [2, 3, 0, 1, 5],
            arg_cnt: 4,
        },
        6,
    );
}

#[test]
fn test_invoke_custom_range() {
    let buffer = [0xFD, 0x04, 0x21, 0x02, 0x31, 0x54];
    assert_helper(
        &buffer,
        Instruction::InvokeCustomRange {
            call_site_idx: 0x221,
            first_arg: 0x5431,
            arg_cnt: 4,
        },
        6,
    );
}

#[test]
fn test_const_method_handle() {
    let buffer = [0xFE, 0x01, 0x23, 0x04];
    assert_helper(
        &buffer,
        Instruction::ConstMethodHandle {
            dst: 1,
            method_handle_idx: 0x0423,
        },
        4,
    );
}

#[test]
fn test_const_method_type() {
    let buffer = [0xFF, 0x01, 0x23, 0x04];
    assert_helper(
        &buffer,
        Instruction::ConstMethodType {
            dst: 1,
            proto_idx: 0x0423,
        },
        4,
    );
}
