use syn::{
    parse::Parser, punctuated::Punctuated, spanned::Spanned as _, Attribute, Expr, GenericArgument,
    Lit, MacroDelimiter, Meta, PathArguments, ReturnType, Token, Type,
};

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
        // TODO(ichen): clean up this function, it contains too much logic

        // Partition the attributes into #[should_panic] and others
        fn is_should_panic(attr: &Attribute) -> bool {
            attr.meta
                .path()
                .get_ident()
                .is_some_and(|attr| attr == "should_panic")
        }
        let (should_panics, others) = attrs.into_iter().partition::<Vec<_>, _>(is_should_panic);

        // If there are two (or more) #[should_panic] attributes, error
        if let Some(second_attr) = should_panics.get(1) {
            return Err(syn::Error::new(
                second_attr.span(),
                "multiple #[should_panic] attributes are not allowed",
            ));
        }

        // Figure out what `Self` to return based on the return type of the test
        // function, and the `#[should_panic]` attribute (or lack of one)
        fn parse_expected_message(message: Expr) -> syn::Result<String> {
            match message {
                Expr::Lit(syn::ExprLit {
                    lit: Lit::Str(lit), ..
                }) => Ok(lit.value()),

                expr => Err(syn::Error::new(
                    expr.span(),
                    "expected panic message must be a string literal",
                )),
            }
        }
        let should_panic = should_panics.into_iter().next();
        let this = match (return_type, should_panic) {
            (ReturnType::Default, None) => Self::ShouldNotPanic,
            (ReturnType::Default, Some(should_panic)) => match should_panic.meta {
                Meta::Path(_) => Self::ShouldPanic,
                Meta::List(list) => {
                    // Catch weird usage like #[should_panic { expected = "?" }]
                    if !matches!(list.delimiter, MacroDelimiter::Paren(_)) {
                        return Err(syn::Error::new(
                            list.delimiter.span().span(),
                            "wrong meta list delimiters - expected `(` and `)`",
                        ));
                    }

                    let invalid_meta_item_arg_error = Err(syn::Error::new(
                        list.tokens.span(),
                        "invalid meta item argument - expected `expected = \"...\"`",
                    ));
                    let Ok(meta_list) = Parser::parse2(
                        Punctuated::<Meta, Token![,]>::parse_terminated,
                        list.tokens,
                    ) else {
                        return invalid_meta_item_arg_error;
                    };

                    let meta_args: Vec<_> = meta_list.into_iter().take(2).collect();

                    if meta_args.len() != 1 {
                        return invalid_meta_item_arg_error;
                    }

                    let Meta::NameValue(name_value) = meta_args.into_iter().next().unwrap() else {
                        return invalid_meta_item_arg_error;
                    };

                    if !name_value
                        .path
                        .get_ident()
                        .is_some_and(|ident| ident == "expected")
                    {
                        return invalid_meta_item_arg_error;
                    };

                    Self::ShouldPanicWithMessage {
                        expected_substring: parse_expected_message(name_value.value)?,
                    }
                }
                Meta::NameValue(meta) => Self::ShouldPanicWithMessage {
                    expected_substring: parse_expected_message(meta.value)?,
                },
            },
            (ReturnType::Type(_, ty), None) => {
                let invalid_return_type_error = Err(syn::Error::new(
                    ty.span(),
                    "invalid return type for adversarial test function",
                ));

                match *ty {
                    Type::Path(ty) => {
                        if ty.qself.is_some() {
                            return invalid_return_type_error;
                        }

                        if ty.path.get_ident().is_some_and(|ty| ty == "bool") {
                            Self::ShouldReturnTrue
                        } else if ty.path.leading_colon.is_none()
                            && ty.path.segments.len() == 1
                            && ty.path.segments.iter().next().unwrap().ident == "Result"
                        {
                            let invalid_form_error = Err(syn::Error::new(
                                ty.span(),
                                "`Result` return type must be of the form `Result<(), ...>`",
                            ));

                            match ty.path.segments.into_iter().next().unwrap().arguments {
                                PathArguments::AngleBracketed(args) => {
                                    if args.args.len() != 2 {
                                        return invalid_form_error;
                                    }

                                    // Error if the first generic isn't `()`
                                    match &args.args[0] {
                                        GenericArgument::Type(Type::Tuple(ty)) => {
                                            if !ty.elems.is_empty() {
                                                return invalid_form_error;
                                            }
                                        }
                                        _ => return invalid_form_error,
                                    }

                                    let GenericArgument::Type(err_ty) =
                                        args.args.into_iter().nth(1).unwrap()
                                    else {
                                        return invalid_form_error;
                                    };

                                    Self::ShouldReturnOk { err_ty }
                                }
                                _ => return invalid_form_error,
                            }
                        } else {
                            return invalid_return_type_error;
                        }
                    }
                    _ => return invalid_return_type_error,
                }
            }
            (ReturnType::Type(_, _), Some(attr)) => return Err(syn::Error::new(
                attr.span(),
                "the `#[should_panic]` attribute cannot be applied to tests based on return values",
            )),
        };

        Ok((this, others))
    }
}
