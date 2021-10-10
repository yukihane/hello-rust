// 自前実装の map_ok
// https://stackoverflow.com/a/36370251/4506703

#[derive(Clone)]
pub struct MapOkIterator<I, F> {
    iter: I,
    f: F,
}

impl<A, B, E, I, F> Iterator for MapOkIterator<I, F>
where
    F: FnMut(A) -> B,
    I: Iterator<Item = Result<A, E>>,
{
    type Item = Result<B, E>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| x.map(&mut self.f))
    }
}

pub trait MapOkTrait {
    fn map_ok<F, A, B, E>(self, func: F) -> MapOkIterator<Self, F>
    where
        Self: Sized + Iterator<Item = Result<A, E>>,
        F: FnMut(A) -> B,
    {
        MapOkIterator {
            iter: self,
            f: func,
        }
    }
}

impl<I, T, E> MapOkTrait for I where I: Sized + Iterator<Item = Result<T, E>> {}

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .map_ok(|i| i * 2)
        .collect();
    println!("Results: {:?}", numbers);
}
