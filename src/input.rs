use crate::span::{Span, UsizeSpan};
use crate::util::{Direction, Point, Region};

pub trait Input {
    type Position: Copy;
    type Token;
    type Span: Span;

    fn start(&self) -> Self::Position;

    fn next(&self, direction: Direction, pos: Self::Position) -> (Self::Position, Option<Self::Token>);

    fn rect_span(&self, tl: Self::Position, br: Self::Position) -> Self::Span;
}

impl<const N: usize, const M: usize> Input for [[char; N]; M] {
    type Position = Point<usize>;
    type Token = char;
    type Span = UsizeSpan;

    fn start(&self) -> Self::Position {
        Point::from((0, 0))
    }

    fn next(&self, direction: Direction, pos: Self::Position) -> (Self::Position, Option<Self::Token>) {
        if let Some(new_pos) = pos.shift(direction) {
            let token = self
                .get(new_pos.x)
                .and_then(|a| a.get(new_pos.y))
                .copied();

            (new_pos, token)
        } else {
            (pos, None)
        }
    }

    fn rect_span(&self, tl: Self::Position, br: Self::Position) -> Self::Span {
        UsizeSpan::new(Region::new(tl, br))
    }
}

impl Input for [&[char]] {
    type Position = Point<usize>;
    type Token = char;
    type Span = UsizeSpan;

    fn start(&self) -> Self::Position {
        Point::from((0, 0))
    }

    fn next(&self, direction: Direction, pos: Self::Position) -> (Self::Position, Option<Self::Token>) {
        if let Some(new_pos) = pos.shift(direction) {
            let token = self
                .get(new_pos.x)
                .and_then(|a| a.get(new_pos.y))
                .copied();

            (new_pos, token)
        } else {
            (pos, None)
        }
    }

    fn rect_span(&self, tl: Self::Position, br: Self::Position) -> Self::Span {
        UsizeSpan::new(Region::new(tl, br))
    }
}

impl Input for [Vec<char>] {
    type Position = Point<usize>;
    type Token = char;
    type Span = UsizeSpan;

    fn start(&self) -> Self::Position {
        Point::from((0, 0))
    }

    fn next(&self, direction: Direction, pos: Self::Position) -> (Self::Position, Option<Self::Token>) {
        if let Some(new_pos) = pos.shift(direction) {
            let token = self
                .get(new_pos.x)
                .and_then(|a| a.get(new_pos.y))
                .copied();

            (new_pos, token)
        } else {
            (pos, None)
        }
    }

    fn rect_span(&self, tl: Self::Position, br: Self::Position) -> Self::Span {
        UsizeSpan::new(Region::new(tl, br))
    }
}

impl Input for [&str] {
    type Position = Point<usize>;
    type Token = char;
    type Span = UsizeSpan;

    fn start(&self) -> Self::Position {
        Point::from((0, 0))
    }

    fn next(&self, direction: Direction, pos: Self::Position) -> (Self::Position, Option<Self::Token>) {
        if let Some(new_pos) = pos.shift(direction) {
            let token = self
                .get(new_pos.x)
                .and_then(|a| a.chars().nth(new_pos.y));

            (new_pos, token)
        } else {
            (pos, None)
        }
    }

    fn rect_span(&self, tl: Self::Position, br: Self::Position) -> Self::Span {
        UsizeSpan::new(Region::new(tl, br))
    }
}

impl Input for [String] {
    type Position = Point<usize>;
    type Token = char;
    type Span = UsizeSpan;

    fn start(&self) -> Self::Position {
        Point::from((0, 0))
    }

    fn next(&self, direction: Direction, pos: Self::Position) -> (Self::Position, Option<Self::Token>) {
        if let Some(new_pos) = pos.shift(direction) {
            let token = self
                .get(new_pos.x)
                .and_then(|a| a.chars().nth(new_pos.y));

            (new_pos, token)
        } else {
            (pos, None)
        }
    }

    fn rect_span(&self, tl: Self::Position, br: Self::Position) -> Self::Span {
        UsizeSpan::new(Region::new(tl, br))
    }
}

pub struct WithCtx<'a, Ctx, I>(Ctx, &'a I);

impl<'a, Ctx, I> Input for WithCtx<'a, Ctx, I>
where
    Ctx: Clone,
    I: Input,
{
    type Position = I::Position;
    type Token = I::Token;
    type Span = (Ctx, I::Span);

    fn start(&self) -> Self::Position {
        self.1.start()
    }

    fn next(&self, direction: Direction, pos: Self::Position) -> (Self::Position, Option<Self::Token>) {
        self.1.next(direction, pos)
    }

    fn rect_span(&self, tl: Self::Position, br: Self::Position) -> Self::Span {
        (self.0.clone(), self.1.rect_span(tl, br))
    }
}
