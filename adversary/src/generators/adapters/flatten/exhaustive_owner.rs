use std::convert::Infallible;

use pair::{HasDependent, Owner, Pair};

use crate::ValueGen;

pub enum Kind {
    Exhaustive,
    Adversarial,
}

pub struct IterOwner<G: ValueGen>(pub G);

pub struct ValueGenIter<G: ValueGen>(Pair<IterOwner<G>>);

impl<G: ValueGen> ValueGenIter<G> {
    pub fn new(value_gen: G, kind: Kind) -> Self {
        Self(Pair::new_with_context(IterOwner(value_gen), kind))
    }
}

impl<G: ValueGen> Iterator for ValueGenIter<G> {
    type Item = G::Seed;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.with_dependent_mut(|iter| iter.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.with_dependent(|iter| iter.size_hint())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.with_dependent_mut(|iter| iter.nth(n))
    }
}

impl<'owner, G: ValueGen> HasDependent<'owner> for IterOwner<G> {
    // TODO: use ATPIT once stabilized
    type Dependent = Box<dyn Iterator<Item = G::Seed> + 'owner>;
}

impl<G: ValueGen> Owner for IterOwner<G> {
    type Context<'a> = Kind;
    type Error = Infallible;

    fn make_dependent<'owner>(
        &'owner self,
        kind: Self::Context<'_>,
    ) -> Result<pair::Dependent<'owner, Self>, Self::Error> {
        Ok(match kind {
            Kind::Exhaustive => Box::new(self.0.exhaustive()),
            Kind::Adversarial => Box::new(self.0.adversarial()),
        })
    }
}
