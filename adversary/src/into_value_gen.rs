use crate::ValueGen;

// TODO(ichen): should this be fallible, or maybe have both a fallible and
// infallible version? Consider integer ranges - nothing stops 5..3 from being
// created or attempted to be turned into a ValueGen. The current impl panics in
// this case - is that really the best we can do?
pub trait IntoValueGen<Value> {
    type Gen: ValueGen<Value = Value>;

    fn into_value_gen(self) -> Self::Gen;
}

/// Generic impl of `IntoValueGen` for any `G: ValueGen`
impl<G: ValueGen> IntoValueGen<G::Value> for G {
    type Gen = Self;

    #[inline]
    fn into_value_gen(self) -> Self::Gen {
        self
    }
}
