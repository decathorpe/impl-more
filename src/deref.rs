/// Implement [`Deref`] for a struct.
///
/// The first argument is that of the newtype struct to create the impl for and the second is the
/// deref target type. The third argument is required for non-newtype structs and is the name of the
/// field to deref to.
///
/// Also see [`impl_deref_mut`], [`impl_deref_and_mut`], and [`forward_deref_and_mut`].
///
/// # Examples
/// ```
/// use impl_more::impl_deref;
///
/// struct Foo(String);
/// impl_deref!(Foo, String);
///
/// let mut foo = Foo("bar".to_owned());
/// assert_eq!(foo.len(), 3);
/// ```
///
/// ```
/// use impl_more::impl_deref;
///
/// struct Foo { msg: String }
/// impl_deref!(Foo, String, msg);
///
/// let mut foo = Foo { msg: "bar".to_owned() };
/// assert_eq!(foo.len(), 3);
/// ```
///
/// [`Deref`]: std::ops::Deref
/// [`impl_deref_mut`]: crate::impl_deref_mut
/// [`impl_deref_and_mut`]: crate::impl_deref_and_mut
/// [`forward_deref_and_mut`]: crate::forward_deref_and_mut
#[macro_export]
macro_rules! impl_deref {
    ($ty:ty, $target:ty) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    ($ty:ty, $target:ty, $field:ident) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }
    };
}

/// Implement [`DerefMut`] for a struct.
///
/// The first argument is that of the struct to create the impl for and this type must also
/// implement [`Deref`]. The second argument is required for non-newtype structs and is the field
/// to deref to.
///
/// Also see [`impl_deref`], [`impl_deref_and_mut`], and [`forward_deref_and_mut`].
///
/// # Examples
/// ```
/// use impl_more::{impl_deref, impl_deref_mut};
///
/// struct Foo(String);
///
/// impl_deref!(Foo, String);
/// impl_deref_mut!(Foo);
///
/// let mut foo = Foo("bar".to_owned());
/// foo.push('!');
///
/// assert_eq!(*foo, "bar!");
/// ```
///
/// [`Deref`]: std::ops::Deref
/// [`DerefMut`]: std::ops::DerefMut
/// [`impl_deref`]: crate::impl_deref
/// [`impl_deref_and_mut`]: crate::impl_deref_and_mut
/// [`forward_deref_and_mut`]: crate::forward_deref_and_mut
#[macro_export]
macro_rules! impl_deref_mut {
    ($ty:ty) => {
        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    ($ty:ty, $field:ident) => {
        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}

/// Implements [`Deref`] and [`DerefMut`] by forwarding through an inner field's implementation.
///
/// Use the `ref <type>` form for deref-ing to types with lifetimes like `&str`. For newtype
/// structs, only the struct name and deref target type is necessary.
///
/// Also see [`forward_deref_and_mut`].
///
/// # Examples
/// ```
/// struct MyNewTypeStruct(String);
/// impl_more::impl_deref_and_mut!(MyNewTypeStruct, String);
///
/// let foo = MyNewTypeStruct("one".to_owned());
/// let foo_ref: &String = &foo;
///
/// // Unlike `forward_deref_and_mut`, this macro will not forward the deref implementation
/// // through the named type. Even so, in some cases Rust will be able to support these cases.
///
/// let foo_ref: &str = &foo;
///
/// fn accepts_string_slice(_: &str) {}
/// accepts_string_slice(&foo);
/// ```
///
/// [`Deref`]: std::ops::Deref
/// [`DerefMut`]: std::ops::DerefMut
/// [`forward_deref_and_mut`]: crate::forward_deref_and_mut
#[macro_export]
macro_rules! impl_deref_and_mut {
    ($ty:ty, $target:ty) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    ($ty:ty, $target:ty, $field:ident) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}

/// Implements [`Deref`] and [`DerefMut`] by forwarding through an inner field's implementation.
///
/// Use the `ref <type>` form for deref-ing to types with lifetimes like `&str`. For newtype
/// structs, only the struct name and deref target type is necessary.
///
/// Also see [`impl_deref_and_mut`].
///
/// # Examples
/// ```
/// fn accepts_string_slice(_: &str) {}
/// fn accepts_mut_string_slice(_: &str) {}
///
/// struct MyNewTypeStruct(String);
/// impl_more::forward_deref_and_mut!(MyNewTypeStruct, ref str);
/// let foo = MyNewTypeStruct("one".to_owned());
/// let foo_ref: &str = &foo;
/// accepts_string_slice(&foo);
/// accepts_mut_string_slice(&foo);
///
/// struct MyStruct { message: String };
/// impl_more::forward_deref_and_mut!(MyStruct, ref str, message);
/// let foo = MyStruct { message: "two".to_owned() };
/// accepts_string_slice(&foo);
/// accepts_mut_string_slice(&foo);
/// ```
///
/// [`impl_deref_and_mut`]: crate::impl_deref_and_mut
/// [`Deref`]: std::ops::Deref
/// [`DerefMut`]: std::ops::DerefMut
#[macro_export]
macro_rules! forward_deref_and_mut {
    ($ty:ty, $target:ty) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.0)
            }
        }

        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::core::ops::DerefMut::deref_mut(&mut self.0)
            }
        }
    };

    ($ty:ty, ref $target:ty) => {
        impl<'__impl_more_a> ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.0)
            }
        }

        impl<'__impl_more_a> ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::core::ops::DerefMut::deref_mut(&mut self.0)
            }
        }
    };

    ($ty:ty, $target:ty, $field:ident) => {
        impl ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.$field)
            }
        }

        impl ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::core::ops::DerefMut::deref_mut(&mut self.$field)
            }
        }
    };

    ($ty:ty, ref $target:ty, $field:ident) => {
        impl<'__impl_more_a> ::core::ops::Deref for $ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.$field)
            }
        }

        impl<'__impl_more_a> ::core::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::core::ops::DerefMut::deref_mut(&mut self.$field)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    fn accepts_string_slice(_: &str) {}
    fn accepts_mut_string_slice(_: &mut str) {}

    struct Foo1(String);
    impl_deref_and_mut!(Foo1, String);
    static_assertions::assert_impl_all!(
        Foo1:
        // impls
        Deref<Target = String>,
        DerefMut<Target = String>,
    );

    struct Foo2(String);
    forward_deref_and_mut!(Foo2, ref str);
    static_assertions::assert_impl_all!(
        Foo2:
        // impls
        Deref,
        DerefMut,
    );

    #[test]
    fn foo2_impls() {
        let mut foo = Foo2("".to_owned());
        accepts_string_slice(&foo);
        accepts_mut_string_slice(&mut foo);
    }

    struct Foo3 {
        msg: String,
    }
    forward_deref_and_mut!(Foo3, ref str, msg);
    static_assertions::assert_impl_all!(
        Foo3:
        // impls
        Deref,
        DerefMut,
    );

    #[test]
    fn foo3_impls() {
        let mut foo = Foo3 { msg: "".to_owned() };
        accepts_string_slice(&foo);
        accepts_mut_string_slice(&mut foo);
    }
}
