#![no_std]
pub extern crate paste;

#[macro_export]
/// Macro to place fields at specified offsets
macro_rules! offset {
    // Guarded munched exprs
    // output
    (@guard ($current_offset:expr) -> {$(#[$attr:meta])* $vis:vis struct $name:ident $(($amount:expr, $vis_field:vis $id:ident: $ty:ty))*}) =>    {

        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])* $vis struct $name { $([<_pad $id>]: [u8;$amount], $vis_field $id: $ty),* }
        }

    };

    // add more fields
    (@guard ($current_offset:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty $(,)?) -> {$($output:tt)*}) => {
        offset!(@guard ($offset + core::mem::size_of::<$ty>()) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    (@guard ($current_offset:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty, $($next:tt)+) -> {$($output:tt)*}) => {
        offset!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)+) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    // Entry points
    ($(#[$attr:meta])* $vis:vis struct $struct_name:ident {$($input:tt)*}) => {
        offset!(@guard (0, $($input)*) -> {$(#[$attr])* $vis struct $struct_name});
        $crate::offset_checker!($struct_name {$($input)*});
    };
}


#[macro_export]
/// Same as offset except using custom Debug implementation that behaves
/// like the derived Debug implementation for this struct with the exception
/// that the padding fields are not printed
macro_rules! offset_debug {
    // Guarded munched exprs
    // output
    (@guard ($current_offset:expr) -> {$(#[$attr:meta])* $vis:vis struct $name:ident $(($amount:expr, $vis_field:vis $id:ident: $ty:ty))*}) =>    {

        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])* $vis struct $name { $([<_pad $id>]: [u8;$amount], $vis_field $id: $ty),* }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                 $(.field(stringify!($id), unsafe { &core::ptr::read_unaligned(core::ptr::addr_of!(self.$id))}))*
                 .finish()
            }
        }
    };

    // add more fields
    (@guard ($current_offset:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty $(,)?) -> {$($output:tt)*}) => {
        offset_debug!(@guard ($offset + core::mem::size_of::<$ty>()) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    (@guard ($current_offset:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty, $($next:tt)+) -> {$($output:tt)*}) => {
        offset_debug!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)+) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    // Entry points
    ($(#[$attr:meta])* $vis:vis struct $struct_name:ident {$($input:tt)*}) => {
        offset_debug!(@guard (0, $($input)*) -> {$(#[$attr])* $vis struct $struct_name});
        $crate::offset_checker!($struct_name {$($input)*});
    };
}

#[cfg(feature = "checked")]
#[macro_export]
macro_rules! offset_checker {
        ($struct_name:ident {$($offset:literal $vis_field:vis $id:ident: $ty:ty),* $(,)?}) => {
            $(const _: ()  = assert!(core::mem::offset_of!($struct_name, $id) == $offset);)*
        };

}

#[cfg(not(feature = "checked"))]
#[macro_export]
macro_rules! offset_checker {
        ($struct_name:ident {$($offset:literal $vis_field:vis $id:ident: $ty:ty),* $(,)?}) => {
        };

}
