//! Limb from conversions

use super::{Inner, Limb, Wide};

impl Limb {
    /// Create a [`Limb`] from a `u8` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u8>` when stable
    pub const fn from_u8(n: u8) -> Self {
        Limb(n as Inner)
    }

    /// Create a [`Limb`] from a `u16` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u16>` when stable
    pub const fn from_u16(n: u16) -> Self {
        Limb(n as Inner)
    }

    /// Create a [`Limb`] from a `u32` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u32>` when stable
    pub const fn from_u32(n: u32) -> Self {
        Limb(n as Inner)
    }

    /// Create a [`Limb`] from a `u64` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u64>` when stable
    #[cfg(target_pointer_width = "64")]
    pub const fn from_u64(n: u64) -> Self {
        Limb(n)
    }
}

impl From<u8> for Limb {
    #[inline]
    fn from(n: u8) -> Limb {
        Limb(n.into())
    }
}

impl From<u16> for Limb {
    #[inline]
    fn from(n: u16) -> Limb {
        Limb(n.into())
    }
}

impl From<u32> for Limb {
    #[inline]
    fn from(n: u32) -> Limb {
        Limb(n.into())
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u64> for Limb {
    #[inline]
    fn from(n: u64) -> Limb {
        Limb(n)
    }
}

impl From<Limb> for Inner {
    #[inline]
    fn from(limb: Limb) -> Inner {
        limb.0
    }
}

impl From<Limb> for Wide {
    #[inline]
    fn from(limb: Limb) -> Wide {
        limb.0.into()
    }
}
