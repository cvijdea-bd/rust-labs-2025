use crate::errors::DexParseError;

pub trait TryParseFromBytes {
    const NAME: &str;
    const SIZE: usize;

    /// Attempts to parse a struct from a byte slice. If the slice is too small, it returns an error.
    fn try_parse_from_bytes(buffer: &[u8]) -> Result<Self, DexParseError>
    where
        Self: Sized,
    {
        if buffer.len() < Self::SIZE {
            return Err(DexParseError::InvalidElementSize {
                field: Self::NAME,
                expected: Self::SIZE,
                actual: buffer.len(),
            });
        }

        Ok(Self::parse_from_bytes(buffer))
    }

    /// Parses a struct from a byte slice.
    /// # Panics
    /// Panics if the slice is too small.
    fn parse_from_bytes(buffer: &[u8]) -> Self
    where
        Self: Sized;
}
