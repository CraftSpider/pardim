/// Multi-dimensional parsing
///
/// This library sets out to provide a solution to parsing a 2-dimensional language in a nice
/// manner. 2D languages provide some unique challenges over 1D, but this library looks to simplify
/// their handling as much as possible.
///
/// # How
///
/// In a 1D language, items are normally parsed left-to-right over the span of an entire file.
/// For a 2D language, this approach doesn't work. After all, many 2D languages impose constraints
/// over token position vertically as well as horizontally.
///
/// There are 2 main facets to parsing 2D things: 'regions' and 'paths'. A region is a contiguous
/// space, not necessarily rectangular. A path is any set of items within a region, such as lines
/// connecting boxes or identifiers attached to a region. 1D parsing can be considered a
/// special-case form of path parsing, where there is an added requirement that all path elements
/// are horizontally contiguous.
///
/// A region may expect certain paths connected to it, or a path may expect to be connected to a
/// region at its end.
///
/// ```ignore
/// /\   /\
/// \/---\/
/// ```
/// Is defined by
///
/// ```
/// let boxes = corner_region()
///     .horizontal('-')
///     .vertical('|')
///     .tl_br('/')
///     .tr_bl('\\')
///     .contains(ignore());
///
/// boxes
///     .maybe_path(simple_path().horizontal('-').vertical('|'));
/// ```
///
/// Just as 1D parsing progresses left-to-right along the input, 2D parsing progresses left-to-right
/// and top-to-bottom, trying its rules against items in this fashion.
///
/// ```ignore
/// /---------\
/// | foo bar |
/// \---------/
///
/// /---------\
/// |         |
/// \---------/
/// ```
///

mod container;
pub mod util;
pub mod span;
pub mod input;
pub mod parser;
pub mod error;
pub mod regions;
pub mod primitive;
