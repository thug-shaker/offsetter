#![no_std]
pub extern crate paste;
#[macro_export]
macro_rules! offset {
    // Guarded munched exprs
    // output
    (@guard ($current_offset:expr,) -> {$(#[$attr:meta])* struct $name:ident $(($amount:expr, $id:ident: $ty:ty))*}) => {
        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])* struct $name { $([<_pad $id>]: [u8;$amount], $id: $ty),* }
        }
    };
    // add more fields
    (@guard ($current_offset:expr, $offset:literal $id:ident: $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        offset!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)*) -> {$($output)* ($offset - ($current_offset), $id: $ty)});
    };

    // Entry points
    ($(#[$attr:meta])* struct $struct_name:ident {$($input:tt)*}) => {

        offset!(@guard (0, $($input)*) -> {$(#[$attr])* struct $struct_name});
    };
}

#[macro_export]
macro_rules! offset_debug {
    // Guarded munched exprs
    // output
    (@guard ($current_offset:expr,) -> {$(#[$attr:meta])* struct $name:ident $(($amount:expr, $id:ident: $ty:ty))*}) => {

        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])* struct $name { $([<_pad $id>]: [u8;$amount], $id: $ty),* }
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
    (@guard ($current_offset:expr, $offset:literal $id:ident: $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        offset_debug!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)*) -> {$($output)* ($offset - ($current_offset), $id: $ty)});
    };

    // Entry points
    ($(#[$attr:meta])* struct $struct_name:ident {$($input:tt)*}) => {

        offset_debug!(@guard (0, $($input)*) -> {$(#[$attr])* struct $struct_name});
    };
}
