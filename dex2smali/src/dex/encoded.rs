/// https://source.android.com/docs/core/runtime/dex-format#encoded-field-format
#[allow(unused)]
#[derive(Debug)]
pub struct EncodedField {
    /// index into the `field_ids` list for the identity of this field (includes the name and descriptor), represented as a difference from the index of previous element in the list. The index of the first element in a list is represented directly.
    field_idx_diff: u32,
    /// access flags for the field (`public`, `final`, etc.). See "`access_flags` Definitions" for details.
    access_flags: u32,
}

/// https://source.android.com/docs/core/runtime/dex-format#encoded-method
#[allow(unused)]
#[derive(Debug)]
pub struct EncodedMethod {
    /// index into the `method_ids` list for the identity of this method (includes the name and descriptor), represented as a difference from the index of previous element in the list. The index of the first element in a list is represented directly.
    method_idx_diff: u32,
    /// access flags for the method (`public`, `final`, etc.). See "`access_flags` Definitions" for details.
    access_flags: u32,
    /// offset from the start of the file to the code structure for this method, or `0` if this method is either `abstract` or `native`. The offset should be to a location in the data section. The format of the data is specified by "`code_item`" below.
    code_off: u32,
}
