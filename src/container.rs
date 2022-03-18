
pub trait Container<T> {
    type Iter: Iterator<Item = T>;
    fn get_iter(&self) -> Self::Iter;
}

impl<T: Clone> Container<T> for T {
    type Iter = std::iter::Once<T>;

    fn get_iter(&self) -> Self::Iter {
        std::iter::once(self.clone())
    }
}

impl Container<char> for String {
    type Iter = std::vec::IntoIter<char>;

    fn get_iter(&self) -> Self::Iter {
        self.chars().collect::<Vec<_>>().into_iter()
    }
}

impl<'a> Container<char> for &'a str {
    type Iter = std::str::Chars<'a>;

    fn get_iter(&self) -> Self::Iter {
        self.chars()
    }
}

impl<'a, T: Clone> Container<T> for &'a [T] {
    type Iter = std::iter::Cloned<std::slice::Iter<'a, T>>;

    fn get_iter(&self) -> Self::Iter {
        self.iter().cloned()
    }
}

impl<'a, T: Clone, const N: usize> Container<T> for &'a [T; N] {
    type Iter = std::iter::Cloned<std::slice::Iter<'a, T>>;

    fn get_iter(&self) -> Self::Iter {
        self.iter().cloned()
    }
}
