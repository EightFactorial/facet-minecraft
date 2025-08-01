use core::convert::Infallible;

pub trait SnbtWriter {
    type WriteError;

    fn write_str(&mut self, value: &str) -> Result<(), Self::WriteError>;
    fn write_char(&mut self, value: char) -> Result<(), Self::WriteError>;
    fn reserve(&mut self, additional: usize) -> Result<(), Self::WriteError>;
}

#[expect(clippy::unit_arg)]
impl SnbtWriter for alloc::string::String {
    type WriteError = Infallible;

    fn write_str(&mut self, value: &str) -> Result<(), Self::WriteError> {
        Ok(self.push_str(value))
    }

    fn write_char(&mut self, value: char) -> Result<(), Self::WriteError> { Ok(self.push(value)) }

    fn reserve(&mut self, value: usize) -> Result<(), Self::WriteError> { Ok(self.reserve(value)) }
}

impl SnbtWriter for alloc::borrow::Cow<'_, str> {
    type WriteError = Infallible;

    fn write_str(&mut self, value: &str) -> Result<(), Self::WriteError> {
        SnbtWriter::write_str(self.to_mut(), value)
    }

    fn write_char(&mut self, value: char) -> Result<(), Self::WriteError> {
        SnbtWriter::write_char(self.to_mut(), value)
    }

    fn reserve(&mut self, value: usize) -> Result<(), Self::WriteError> {
        SnbtWriter::reserve(self.to_mut(), value)
    }
}
