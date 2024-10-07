pub fn n<N: core::str::FromStr>(input: &str) -> nom::IResult<&str, N> {
    nom::combinator::map_res(nom::character::complete::digit1, N::from_str)(input)
}

#[macro_export]
/// ```
/// fn parse(i: &str) -> nom::IResult<&str, Foo> {
///     Ok((i, Foo))
/// }
///
/// struct Foo;
///
/// libaoc::impl_from_str_from_nom_parser!(parse, Foo);
/// ```
macro_rules! impl_from_str_from_nom_parser {
    ($fn:expr, $obj:ty) => {
        impl core::str::FromStr for $obj {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let (remaining, object) = $fn(s).map_err(|e| anyhow::anyhow!("{e}"))?;
                anyhow::ensure!(remaining.is_empty() || remaining == "\n");
                Ok(object)
            }
        }
    };
}

#[macro_export]
/// ```
/// fn parse(i: &str) -> nom::IResult<&str, Foo> {
///     Ok((i, Foo::new(i)))
/// }
///
/// #[allow(dead_code)]
/// struct Foo<'a>(&'a str);
///
/// impl<'a> Foo<'a> {
///     fn new(i: &'a str) -> Self {
///         Self(i)
///     }
/// }
///
/// libaoc::impl_from_str_for_obj_with_lifetimes_from_nom_parser!(parse, Foo);
/// ```
macro_rules! impl_from_str_for_obj_with_lifetimes_from_nom_parser {
    ($fn:ident, $obj:ident) => {
        impl<'input, 'output> core::convert::TryFrom<&'input str> for $obj<'output>
        where
            'input: 'output,
        {
            type Error = anyhow::Error;

            fn try_from(value: &'input str) -> Result<Self, Self::Error> {
                let (remaining, object) = $fn(value).map_err(|e| anyhow::anyhow!("{e}"))?;
                anyhow::ensure!(remaining.is_empty() || remaining == "\n");
                Ok(object)
            }
        }
    };
}
