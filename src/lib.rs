#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

/// Procedural macros that simply maps one enum variants to another.
///
/// # Usage
///
/// ```
/// use cocoa_purity::map_enum;
///
/// #[derive(PartialEq, Eq)]
/// pub enum Destination {
///     A,
///     B,
///     C,
/// }
///
/// #[map_enum(Destination)]
/// pub enum Source0 {
///     Test = A, // A variant of the Destination
/// }
///
/// // explicit destination variant can be elided if
/// // variant in `Source` and `Destination` shares same name
/// #[map_enum(Destination)]
/// pub enum Source {
///     A { power: f64 },
///     B(f64),
///     C,
/// }
/// ```
///
/// let src0 = Source0::Test;
/// let a = Source::A { power: 1.0 };
/// let b = Source::B(1.0);
/// let c = Source::C;
///
/// assert_eq!(src0.into(), Destination::A);
/// assert_eq!(a.into(), Destination::A);
/// assert_eq!(b.into(), Destination::B);
/// assert_eq!(c.into(), Destination::C);
/// # Limitations
///
/// 1. Source enum cannot have explicit discriminant values
/// 2. Destination enum variant cannot be named/tuple like
///    (e.g. mapping from `Variant` to `Base::Variant { field1: T }` is not possible)
#[cfg(feature = "map_enum")]
#[proc_macro_error]
#[proc_macro_attribute]
pub fn map_enum(args: TokenStream, body: TokenStream) -> TokenStream {
    macros::map_enum::map_enum(args, body)
}

mod macros;
