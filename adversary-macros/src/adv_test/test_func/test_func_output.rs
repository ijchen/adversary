use syn::{spanned::Spanned as _, Attribute, ReturnType, Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestFuncOutput {
    /// The test is expected to run without panicking.
    ///
    /// The test will pass if it runs without panicking, or fail if it panics.
    ///
    /// # Examples
    /// ```rs
    /// #[adv_test]
    /// fn example() {
    ///     assert_eq!(2 + 2, 4);
    /// }
    /// ```
    ShouldNotPanic,

    /// The test is expected to panic.
    ///
    /// The test will pass if it panics, or fail if it does not panic.
    ///
    /// # Examples
    /// ```rs
    /// #[adv_test]
    /// #[should_panic]
    /// fn example() {
    ///     "oops".parse::<f32>().unwrap();
    /// }
    /// ```
    ShouldPanic,

    /// The test is expected to panic with a specific message (more precisely,
    /// the panic message is expected to contain some substring).
    ///
    /// # Examples
    /// ```rs
    /// #[adv_test]
    /// #[should_panic(expected = "foobar")]
    /// fn example() {
    ///     panic!("message that contains foobar in it");
    /// }
    /// ```
    ShouldPanicWithMessage {
        /// The expected panic message substring. Any panic message that
        /// contains `expected_substring` as a substring will be considered a
        /// match, and the test will pass.
        expected_substring: String,
    },

    /// The test is expected to return `true`.
    ///
    /// The test will pass if it returns `true`, or fail if it returns `false`
    /// or panics.
    ///
    /// # Examples
    /// ```rs
    /// #[adv_test]
    /// fn example() -> bool {
    ///     2 + 2 == 4
    /// }
    /// ```
    ShouldReturnTrue,

    /// The test is expected to return the [`Ok(())`] variant of a
    /// [`Result<(), _>`].
    ///
    /// The test will pass if it returns the [`Ok(())`] variant, or fail if it
    /// returns the [`Err(_)`] variant or panics.
    ///
    /// # Examples
    /// ```rs
    /// #[adv_test]
    /// fn example() -> Result<(), String> {
    ///     Ok(())
    /// }
    ShouldReturnOk {
        /// The error type `E` in the returned [`Result<(), E>`].
        err_ty: Type,
    },
}

impl TestFuncOutput {
    pub fn parse(
        return_type: ReturnType,
        attrs: Vec<Attribute>,
    ) -> syn::Result<(Self, Vec<Attribute>)> {
        // Find (and remove) the #[should_panic] attribute, if it exists
        // for

        fn is_should_panic(attr: &Attribute) -> bool {
            todo!()
        }

        let (should_panics, others) = attrs.into_iter().partition::<Vec<_>, _>(is_should_panic);

        let should_panic = match should_panics.len() {
            0 => None,
            1 => {
                let mut should_panics = should_panics;
                should_panics.pop()
            }
            2.. => {
                return Err(syn::Error::new(
                    should_panics[1].span(),
                    "TODO: cool message here",
                ))
            }
        };

        Ok((todo!(), others))
    }
}
