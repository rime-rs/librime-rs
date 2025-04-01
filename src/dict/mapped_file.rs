use std::marker::PhantomData;

struct OffsetPtr<T, Offset = i32> {
    offset: Offset,
    _marker: PhantomData<*mut T>,
}

impl<T, Offset> Default for OffsetPtr<T, Offset>
where
    Offset: From<i32>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, Offset> OffsetPtr<T, Offset>
where
    Offset: From<i32>,
{
    pub fn new() -> Self {
        Self {
            offset: Offset::from(0),
            _marker: PhantomData,
        }
    }
    fn from_offset(offset: Offset) -> Self {
        Self {
            offset,
            _marker: PhantomData,
        }
    }
}

pub struct List<T, Size = u32> {
    size: Size,
    at: Vec<T>,
}
impl<T, Size> List<T, Size>
where
    Size: Into<usize> + Clone + Copy,
{
    fn new(size: Size) -> Self {
        let size_usize: usize = size.into();
        List {
            size,
            at: Vec::with_capacity(size_usize),
        }
    }

    fn begin(&self) -> Option<&T> {
        self.at.get(0)
    }

    fn end(&self) -> Option<&T> {
        self.at.get(self.size.into() - 1)
    }

    fn begin_mut(&mut self) -> Option<&mut T> {
        self.at.get_mut(0)
    }

    fn end_mut(&mut self) -> Option<&mut T> {
        self.at.get_mut(self.size.into() - 1)
    }
}
