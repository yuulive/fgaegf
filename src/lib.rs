#![feature(external_doc)]
#![doc(include = "../README.md")]

pub trait LooseMap {
    /// Calls `f(self)`.
    fn loose_map<F, Fret>(self, mut f: F) -> Fret
    where
        Self: Sized,
        F: FnMut(Self) -> Fret,
    {
        f(self)
    }
}
impl<A> LooseMap for A {}
