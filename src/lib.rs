#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

extern crate alloc;

// Submodules
/// Remote
pub mod remote;

/// Everything about telemetry (sending live data from a microcontroller to a system)
pub mod tele;

// Custom types
/// General error type used for the crate
pub type Error = Box<dyn std::error::Error>;

/// Marker trait for declaring a type as parseable from bytes by copying
pub trait TryFromBytes : for<'b> TryFrom<&'b [u8], Error = crate::Error> { }

impl<T : for<'b> TryFrom<&'b [u8], Error = crate::Error>> TryFromBytes for T { }

/// Marker trait for declaring a type as serializable into bytes by copying
pub trait IntoBytes : Into<Vec<u8>> { }

/// Automatically implements `TryFrom<&[u8]>` for the given type if it is sized by copying and transmuting the slice
/// 
/// # Error
/// 
/// Returns an error if the given slice and the type implemented for are unequal in size
/// 
/// # Safety
/// 
/// This macro will generate an unsafe block, it will cause no problems however as it is checked if the right conditions are met 
#[macro_export]
macro_rules! impl_tryfrom_transmute {
    ( $t:ident ) => {
        impl TryFrom<&[u8]> for $t {
            type Error = syiot::Error;
        
            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                if value.len() == size_of::<Self>() {
                    unsafe {
                        let array : [u8; size_of::<Self>()] = value.clone().try_into().unwrap_unchecked();
                        Ok(core::mem::transmute::<_, Self>(array))
                    }
                } else {
                    Err("Wrong slice size!".into())
                }
            }
        }
    };
}