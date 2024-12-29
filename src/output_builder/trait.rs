pub trait OutputTextTrait<'a>: Into<&'a str> {}
impl<'a, T> OutputTextTrait<'a> for T where T: Into<&'a str> + 'static {}
