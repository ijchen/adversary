use std::convert::Infallible;

use pair::{HasDependent, Owner};

use crate::ValueGen;

pub struct ShrinkerOwner<G: ValueGen>(pub G);

impl<'owner, G: ValueGen> HasDependent<'owner> for ShrinkerOwner<G> {
    type Dependent = G::Shrinker<'owner>;
}

impl<G: ValueGen> Owner for ShrinkerOwner<G> {
    type Context<'a> = G::Seed;
    type Error = Infallible;

    fn make_dependent<'owner>(
        &'owner self,
        context: Self::Context<'_>,
    ) -> Result<pair::Dependent<'owner, Self>, Self::Error> {
        Ok(self.0.new_shrinker(context))
    }
}
