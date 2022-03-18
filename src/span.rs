use crate::util::{Point, Region};

pub trait Span {
    type Context;
    type Offset;

    /// Create a new span out of the union of the region covered by this span and another
    fn union(self, other: Self) -> Self;
    /// Create a new span out of the intersection of the region covered by this span and another
    fn intersection(self, other: Self) -> Self;
}

pub struct UsizeSpan {
    regions: Vec<Region<usize>>
}

impl UsizeSpan {
    pub(crate) fn new(region: Region<usize>) -> UsizeSpan {
        UsizeSpan { regions: vec![region] }
    }
}

impl Span for UsizeSpan {
    type Context = ();
    type Offset = Point<usize>;

    fn union(mut self, other: Self) -> Self {
        self.regions.extend(other.regions);
        self
    }

    fn intersection(self, other: Self) -> Self {
        todo!()
    }
}

impl<C, S> Span for (C, S)
where
    S: Span,
{
    type Context = C;
    type Offset = S::Offset;

    fn union(self, other: Self) -> Self {
        let new_span = self.1.union(other.1);
        (self.0, new_span)
    }

    fn intersection(self, other: Self) -> Self {
        let new_span = self.1.intersection(other.1);
        (self.0, new_span)
    }
}
