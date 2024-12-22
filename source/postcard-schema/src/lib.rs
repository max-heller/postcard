#![cfg_attr(not(any(test, feature = "use-std")), no_std)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # Postcard Schema

pub mod impls;
pub mod max_size;
pub mod schema;
pub mod const_helpers;

/// Derive [`Schema`] for a struct or enum
///
/// # Examples
///
/// ```
/// use postcard_schema::Schema;
///
/// #[derive(Schema)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// ```
///
/// # Attributes
///
/// ## `#[postcard(crate = ...)]`
///
/// The `#[postcard(crate = ...)]` attribute can be used to specify a path to the `postcard_schema`
/// crate instance to use when referring to [`Schema`] and [schema types](schema) from generated
/// code. This is normally only applicable when invoking re-exported derives from a different crate.
///
/// ```
/// # use postcard_schema::Schema;
/// use postcard_schema as reexported_postcard_schema;
///
/// #[derive(Schema)]
/// #[postcard(crate = reexported_postcard_schema)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// ```
#[cfg(feature = "derive")]
pub use postcard_derive::Schema;

/// A trait that represents a compile time calculated schema
pub trait Schema {
    /// A recursive data structure that describes the schema of the given
    /// type.
    const SCHEMA: &'static schema::NamedType;

    /// A manually calculated maximum size hint.
    ///
    /// This is useful in the case that a foreign type is expressed
    /// in some form that has a technically unbounded max size, for
    /// example a byte slice, but in reality it never sends more than
    /// some calcuable size.
    ///
    /// A good example is `heapless::Vec<T, N>`, which is expressed
    /// on the wire as a `Seq(N)`, which is unbounded, however the
    /// true max size is the length to store N as a varint plus
    /// `N * max_size::<T>`, which is calcuable.
    ///
    /// This value should not typically be used directly, instead use
    /// [`max_size()`][crate::max_size::max_size()], which will EITHER
    /// use the manual max size, or calculate the max size from the
    /// schema.
    ///
    /// You must not rely on this value for safety reasons, as implementations
    /// could be wrong.
    const MANUAL_MAX_SIZE: Option<usize> = None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_path() {
        #[allow(unused)]
        #[derive(Schema)]
        #[postcard(crate = crate)]
        struct Point {
            x: i32,
            y: i32,
        }

        assert_eq!(
            Point::SCHEMA,
            &schema::NamedType {
                name: "Point",
                ty: &schema::DataModelType::Struct(&[
                    &schema::NamedValue {
                        name: "x",
                        ty: i32::SCHEMA
                    },
                    &schema::NamedValue {
                        name: "y",
                        ty: i32::SCHEMA
                    },
                ])
            }
        );
    }
}
