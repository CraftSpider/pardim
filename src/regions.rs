
// Region Edge Strategies:
// - Single token
//   - Free/Min borders
//   - Free/Max borders
//   - Free/Unambiguous
//   - Rect/Max horizontal
//   - Rect/Max vertical
//   - Rect/Min horizontal
//   - Rect/Min vertical
//   - Rect/Unambiguous
// - Multi token
//   - Corner
//   - Unambiguous

use std::marker::PhantomData;
use crate::container::Container;
use crate::error::Error;
use crate::input::Input;
use crate::parser::{InputRef, Parser, ParseResult};

pub struct TokRegion<I, C, E>(C, PhantomData<(I, E)>);

impl<'a, I, C, E> Parser<'a> for TokRegion<I, C, E>
where
    I: Input,
    C: Container<I>,
    E: Error<I>,
{
    type Input = I;
    type Output = ();
    type Error = E;

    fn go(&self, inp: &mut InputRef<'a, I, E>) -> ParseResult<'a, Self> {
        todo!()
    }
}

pub fn tok_region<I: Input, C: Container<I>, E: Error<I>>(tokens: C) -> TokRegion<I, C, E> {
    TokRegion(tokens, PhantomData)
}

