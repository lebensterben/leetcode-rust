use std::{ops::Index, slice::SliceIndex};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ByteString(Vec<u8>);

impl ByteString {
    #[inline]
    pub fn new() -> Self {
        ByteString(vec![])
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<&str> for ByteString {
    #[inline]
    fn from(text: &str) -> Self {
        Self(text.into())
    }
}

#[macro_export]
macro_rules! byte_string {
    ($e: expr) => {
        $crate::util::ByteString::from($e)
    };
}

impl<I: SliceIndex<[u8]>> Index<I> for ByteString {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&*self.0, index)
    }
}
