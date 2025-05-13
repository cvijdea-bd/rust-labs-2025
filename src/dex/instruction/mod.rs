mod decode;
mod size;
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    // 00-0D: Basic operations
    Nop,
    Move {
        dst: u8,
        src: u8,
    },
    MoveFrom16 {
        dst: u8,
        src: u16,
    },
    Move16 {
        dst: u16,
        src: u16,
    },
    MoveWide {
        dst: u8,
        src: u8,
    },
    MoveWideFrom16 {
        dst: u8,
        src: u16,
    },
    MoveWide16 {
        dst: u16,
        src: u16,
    },
    MoveObject {
        dst: u8,
        src: u8,
    },
    MoveObjectFrom16 {
        dst: u8,
        src: u16,
    },
    MoveObject16 {
        dst: u16,
        src: u16,
    },
    MoveResult {
        dst: u8,
    },
    MoveResultWide {
        dst: u8,
    },
    MoveResultObject {
        dst: u8,
    },
    MoveException {
        dst: u8,
    },

    // 0E-11: Returns
    ReturnVoid,
    Return {
        value: u8,
    },
    ReturnWide {
        value: u8,
    },
    ReturnObject {
        value: u8,
    },

    // 12-1F: Constants and checks
    Const4 {
        dst: u8,
        value: i8,
    },
    Const16 {
        dst: u8,
        value: i16,
    },
    Const {
        dst: u8,
        value: i32,
    },
    ConstHigh16 {
        dst: u8,
        value: i16,
    },
    ConstWide16 {
        dst: u8,
        value: i16,
    },
    ConstWide32 {
        dst: u8,
        value: i32,
    },
    ConstWide {
        dst: u8,
        value: i64,
    },
    ConstWideHigh16 {
        dst: u8,
        value: i16,
    },
    ConstString {
        dst: u8,
        string_idx: u16,
    },
    ConstStringJumbo {
        dst: u8,
        string_idx: u32,
    },
    ConstClass {
        dst: u8,
        type_idx: u16,
    },
    MonitorEnter {
        reference: u8,
    },
    MonitorExit {
        reference: u8,
    },
    CheckCast {
        reference: u8,
        type_idx: u16,
    },
    InstanceOf {
        dst: u8,
        reference: u8,
        type_idx: u16,
    },

    // 21-2A: Arrays and jumps
    ArrayLength {
        dst: u8,
        array: u8,
    },
    NewInstance {
        dst: u8,
        type_idx: u16,
    },
    NewArray {
        dst: u8,
        size: u8,
        type_idx: u16,
    },
    FilledNewArray {
        type_idx: u16,
        args: [u8; 5],
        arg_cnt: u8,
    },
    FilledNewArrayRange {
        type_idx: u16,
        first_arg: u16,
        arg_cnt: u8,
    },
    FillArrayData {
        array: u8,
        offset: i32,
    },
    Throw {
        exception: u8,
    },
    Goto {
        offset: i8,
    },
    Goto16 {
        offset: i16,
    },
    Goto32 {
        offset: i32,
    },
    PackedSwitch {
        value: u8,
        offset: i32,
    },
    SparseSwitch {
        value: u8,
        offset: i32,
    },

    // 2D-37: Comparisons and branches
    CmplFloat {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    CmpgFloat {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    CmplDouble {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    CmpgDouble {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    CmpLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    IfEq {
        a: u8,
        b: u8,
        offset: i16,
    },
    IfNe {
        a: u8,
        b: u8,
        offset: i16,
    },
    IfLt {
        a: u8,
        b: u8,
        offset: i16,
    },
    IfGe {
        a: u8,
        b: u8,
        offset: i16,
    },
    IfGt {
        a: u8,
        b: u8,
        offset: i16,
    },
    IfLe {
        a: u8,
        b: u8,
        offset: i16,
    },
    IfEqz {
        value: u8,
        offset: i16,
    },
    IfNez {
        value: u8,
        offset: i16,
    },
    IfLtz {
        value: u8,
        offset: i16,
    },
    IfGez {
        value: u8,
        offset: i16,
    },
    IfGtz {
        value: u8,
        offset: i16,
    },
    IfLez {
        value: u8,
        offset: i16,
    },

    // 44-51: Array operations
    Aget {
        src: u8,
        array: u8,
        index: u8,
    },
    AgetWide {
        src: u8,
        array: u8,
        index: u8,
    },
    AgetObject {
        src: u8,
        array: u8,
        index: u8,
    },
    AgetBoolean {
        src: u8,
        array: u8,
        index: u8,
    },
    AgetByte {
        src: u8,
        array: u8,
        index: u8,
    },
    AgetChar {
        src: u8,
        array: u8,
        index: u8,
    },
    AgetShort {
        src: u8,
        array: u8,
        index: u8,
    },
    Aput {
        dst: u8,
        array: u8,
        index: u8,
    },
    AputWide {
        dst: u8,
        array: u8,
        index: u8,
    },
    AputObject {
        dst: u8,
        array: u8,
        index: u8,
    },
    AputBoolean {
        dst: u8,
        array: u8,
        index: u8,
    },
    AputByte {
        dst: u8,
        array: u8,
        index: u8,
    },
    AputChar {
        dst: u8,
        array: u8,
        index: u8,
    },
    AputShort {
        dst: u8,
        array: u8,
        index: u8,
    },

    // 52-6D: Field operations
    Iget {
        src: u8,
        object: u8,
        field_idx: u16,
    },
    IgetWide {
        src: u8,
        object: u8,
        field_idx: u16,
    },
    IgetObject {
        src: u8,
        object: u8,
        field_idx: u16,
    },
    IgetBoolean {
        src: u8,
        object: u8,
        field_idx: u16,
    },
    IgetByte {
        src: u8,
        object: u8,
        field_idx: u16,
    },
    IgetChar {
        src: u8,
        object: u8,
        field_idx: u16,
    },
    IgetShort {
        src: u8,
        object: u8,
        field_idx: u16,
    },
    Iput {
        dst: u8,
        object: u8,
        field_idx: u16,
    },
    IputWide {
        dst: u8,
        object: u8,
        field_idx: u16,
    },
    IputObject {
        dst: u8,
        object: u8,
        field_idx: u16,
    },
    IputBoolean {
        dst: u8,
        object: u8,
        field_idx: u16,
    },
    IputByte {
        dst: u8,
        object: u8,
        field_idx: u16,
    },
    IputChar {
        dst: u8,
        object: u8,
        field_idx: u16,
    },
    IputShort {
        dst: u8,
        object: u8,
        field_idx: u16,
    },
    Sget {
        src: u8,
        field_idx: u16,
    },
    SgetWide {
        src: u8,
        field_idx: u16,
    },
    SgetObject {
        src: u8,
        field_idx: u16,
    },
    SgetBoolean {
        src: u8,
        field_idx: u16,
    },
    SgetByte {
        src: u8,
        field_idx: u16,
    },
    SgetChar {
        src: u8,
        field_idx: u16,
    },
    SgetShort {
        src: u8,
        field_idx: u16,
    },
    Sput {
        dst: u8,
        field_idx: u16,
    },
    SputWide {
        dst: u8,
        field_idx: u16,
    },
    SputObject {
        dst: u8,
        field_idx: u16,
    },
    SputBoolean {
        dst: u8,
        field_idx: u16,
    },
    SputByte {
        dst: u8,
        field_idx: u16,
    },
    SputChar {
        dst: u8,
        field_idx: u16,
    },
    SputShort {
        dst: u8,
        field_idx: u16,
    },

    // 6E-72: Invokes
    InvokeVirtual {
        method_idx: u16,
        args: [u8; 5],
        arg_cnt: u8,
    },
    InvokeSuper {
        method_idx: u16,
        args: [u8; 5],
        arg_cnt: u8,
    },
    InvokeDirect {
        method_idx: u16,
        args: [u8; 5],
        arg_cnt: u8,
    },
    InvokeStatic {
        method_idx: u16,
        args: [u8; 5],
        arg_cnt: u8,
    },
    InvokeInterface {
        method_idx: u16,
        args: [u8; 5],
        arg_cnt: u8,
    },

    // 74-78: Range invokes
    InvokeVirtualRange {
        method_idx: u16,
        first_arg: u16,
        arg_cnt: u8,
    },
    InvokeSuperRange {
        method_idx: u16,
        first_arg: u16,
        arg_cnt: u8,
    },
    InvokeDirectRange {
        method_idx: u16,
        first_arg: u16,
        arg_cnt: u8,
    },
    InvokeStaticRange {
        method_idx: u16,
        first_arg: u16,
        arg_cnt: u8,
    },
    InvokeInterfaceRange {
        method_idx: u16,
        first_arg: u16,
        arg_cnt: u8,
    },

    // 7B-8F: Unary operations
    NegInt {
        dst: u8,
        src: u8,
    },
    NotInt {
        dst: u8,
        src: u8,
    },
    NegLong {
        dst: u8,
        src: u8,
    },
    NotLong {
        dst: u8,
        src: u8,
    },
    NegFloat {
        dst: u8,
        src: u8,
    },
    NegDouble {
        dst: u8,
        src: u8,
    },
    IntToLong {
        dst: u8,
        src: u8,
    },
    IntToFloat {
        dst: u8,
        src: u8,
    },
    IntToDouble {
        dst: u8,
        src: u8,
    },
    LongToInt {
        dst: u8,
        src: u8,
    },
    LongToFloat {
        dst: u8,
        src: u8,
    },
    LongToDouble {
        dst: u8,
        src: u8,
    },
    FloatToInt {
        dst: u8,
        src: u8,
    },
    FloatToLong {
        dst: u8,
        src: u8,
    },
    FloatToDouble {
        dst: u8,
        src: u8,
    },
    DoubleToInt {
        dst: u8,
        src: u8,
    },
    DoubleToLong {
        dst: u8,
        src: u8,
    },
    DoubleToFloat {
        dst: u8,
        src: u8,
    },
    IntToByte {
        dst: u8,
        src: u8,
    },
    IntToChar {
        dst: u8,
        src: u8,
    },
    IntToShort {
        dst: u8,
        src: u8,
    },

    // 90-AF: Binary operations
    AddInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    SubInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    MulInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    DivInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    RemInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    AndInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    OrInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    XorInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    ShlInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    ShrInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    UshrInt {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    AddLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    SubLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    MulLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    DivLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    RemLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    AndLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    OrLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    XorLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    ShlLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    ShrLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    UshrLong {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    AddFloat {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    SubFloat {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    MulFloat {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    DivFloat {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    RemFloat {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    AddDouble {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    SubDouble {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    MulDouble {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    DivDouble {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },
    RemDouble {
        dst: u8,
        src_a: u8,
        src_b: u8,
    },

    // B0-CF: 2addr binary operations
    AddInt2Addr {
        dst: u8,
        src: u8,
    },
    SubInt2Addr {
        dst: u8,
        src: u8,
    },
    MulInt2Addr {
        dst: u8,
        src: u8,
    },
    DivInt2Addr {
        dst: u8,
        src: u8,
    },
    RemInt2Addr {
        dst: u8,
        src: u8,
    },
    AndInt2Addr {
        dst: u8,
        src: u8,
    },
    OrInt2Addr {
        dst: u8,
        src: u8,
    },
    XorInt2Addr {
        dst: u8,
        src: u8,
    },
    ShlInt2Addr {
        dst: u8,
        src: u8,
    },
    ShrInt2Addr {
        dst: u8,
        src: u8,
    },
    UshrInt2Addr {
        dst: u8,
        src: u8,
    },
    AddLong2Addr {
        dst: u8,
        src: u8,
    },
    SubLong2Addr {
        dst: u8,
        src: u8,
    },
    MulLong2Addr {
        dst: u8,
        src: u8,
    },
    DivLong2Addr {
        dst: u8,
        src: u8,
    },
    RemLong2Addr {
        dst: u8,
        src: u8,
    },
    AndLong2Addr {
        dst: u8,
        src: u8,
    },
    OrLong2Addr {
        dst: u8,
        src: u8,
    },
    XorLong2Addr {
        dst: u8,
        src: u8,
    },
    ShlLong2Addr {
        dst: u8,
        src: u8,
    },
    ShrLong2Addr {
        dst: u8,
        src: u8,
    },
    UshrLong2Addr {
        dst: u8,
        src: u8,
    },
    AddFloat2Addr {
        dst: u8,
        src: u8,
    },
    SubFloat2Addr {
        dst: u8,
        src: u8,
    },
    MulFloat2Addr {
        dst: u8,
        src: u8,
    },
    DivFloat2Addr {
        dst: u8,
        src: u8,
    },
    RemFloat2Addr {
        dst: u8,
        src: u8,
    },
    AddDouble2Addr {
        dst: u8,
        src: u8,
    },
    SubDouble2Addr {
        dst: u8,
        src: u8,
    },
    MulDouble2Addr {
        dst: u8,
        src: u8,
    },
    DivDouble2Addr {
        dst: u8,
        src: u8,
    },
    RemDouble2Addr {
        dst: u8,
        src: u8,
    },

    // D0-E2: Literal binary operations
    AddIntLit16 {
        dst: u8,
        src: u8,
        value: i16,
    },
    RsubInt {
        dst: u8,
        src: u8,
        value: i16,
    },
    MulIntLit16 {
        dst: u8,
        src: u8,
        value: i16,
    },
    DivIntLit16 {
        dst: u8,
        src: u8,
        value: i16,
    },
    RemIntLit16 {
        dst: u8,
        src: u8,
        value: i16,
    },
    AndIntLit16 {
        dst: u8,
        src: u8,
        value: i16,
    },
    OrIntLit16 {
        dst: u8,
        src: u8,
        value: i16,
    },
    XorIntLit16 {
        dst: u8,
        src: u8,
        value: i16,
    },
    AddIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },
    RsubIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },
    MulIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },
    DivIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },
    RemIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },
    AndIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },
    OrIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },
    XorIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },
    ShlIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },
    ShrIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },
    UshrIntLit8 {
        dst: u8,
        src: u8,
        value: i8,
    },

    // FA-FF: Newer instructions (Dex version 038+)
    InvokePolymorphic {
        method_idx: u16,
        proto_idx: u16,
        args: [u8; 5],
        arg_cnt: u8,
    },
    InvokePolymorphicRange {
        method_idx: u16,
        proto_idx: u16,
        first_arg: u16,
        arg_cnt: u8,
    },
    InvokeCustom {
        call_site_idx: u16,
        args: [u8; 5],
        arg_cnt: u8,
    },
    InvokeCustomRange {
        call_site_idx: u16,
        first_arg: u16,
        arg_cnt: u8,
    },
    ConstMethodHandle {
        dst: u8,
        method_handle_idx: u16,
    },
    ConstMethodType {
        dst: u8,
        proto_idx: u16,
    },
}
