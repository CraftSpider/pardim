use crate::error::Error;
use crate::input::Input;
use crate::util::{Direction, Region};

pub(crate) type ParseResult<'a, P> = Result<<P as Parser<'a>>::Output, <P as Parser<'a>>::Error>;

pub struct InputRef<'a, I: Input + ?Sized, E: Error<I>> {
    input: &'a I,
    pos: I::Position,
    consumed: Vec<I::Span>,
    errors: Vec<E>,
}

impl<'a, 'parse, I, E> InputRef<'a, I, E>
where
    I: ?Sized + Input,
    E: Error<I>,
{
    pub(crate) fn new(input: &'a I) -> InputRef<'a, I, E> {
        InputRef {
            input,
            pos: input.start(),
            consumed: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub(crate) fn save(&mut self) -> I::Position {
        self.pos
    }

    pub(crate) fn rewind(&mut self, pos: I::Position) {
        self.pos = pos;
    }

    pub(crate) fn next(&mut self, direction: Direction) -> (I::Position, Option<I::Token>) {
        let (offset, token) = self.input.next(direction, self.pos);
        self.pos = offset;
        (offset, token)
    }
}

pub trait Parser<'a> {
    type Input: Input;
    type Output;
    type Error: Error<Self::Input>;

    fn parse(&self, input: &'a Self::Input) -> ParseResult<'a, Self> {
        self.go(&mut InputRef::new(input))
    }

    fn go(&self, inp: &mut InputRef<'a, Self::Input, Self::Error>) -> ParseResult<'a, Self>;
}
