use crate::{IntoValueGen, ValueGen, ValueGenExt};

impl<T, IntoGen: IntoValueGen<T>> IntoValueGen<(T,)> for (IntoGen,) {
    fn into_value_gen(self) -> impl ValueGen<Value = (T,)> {
        self.0.into_value_gen().adv_map(|value| (value,))
    }
}
