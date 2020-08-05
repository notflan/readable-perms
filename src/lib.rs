//! Unix permissions `mode_t` in readable format.
//!
//No I don't care about Microshaft wangblows
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use bitflags::bitflags;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Class
{
    Owner(Bit),
    Group(Bit),
    Other(Bit),
}

impl Class
{
    /// Lmfao. Fuck bit masks and unix permissiong, `const fn` so idc
    #[cfg(nightly)]
    const fn mode(&self) -> u32
    {
	macro_rules! map {
	    ($bit:expr, $bt:path, $constant:ident) => {
		if $bit.contains($bt) {
		    $constant
		} else {
		    0
		}
	    }
	}
	match self {
	    Self::Owner(bit) => {
		0u32 |
		map!(bit, Bit::Read, MODE_USER_READ) |
		map!(bit, Bit::Write, MODE_USER_WRITE) |
		map!(bit, Bit::Execute, MODE_USER_EXECUTE)
	    },
	    Self::Group(bit) => {
		0u32 |
		map!(bit, Bit::Read, MODE_GROUP_READ) |
		map!(bit, Bit::Write, MODE_GROUP_WRITE) |
		map!(bit, Bit::Execute, MODE_GROUP_EXECUTE)
	    },
	    Self::Other(bit) => {
		0u32 |
		map!(bit, Bit::Read, MODE_OTHER_READ) |
		map!(bit, Bit::Write, MODE_OTHER_WRITE) |
		map!(bit, Bit::Execute, MODE_OTHER_EXECUTE)
	    },
	}
    }
    #[cfg(not(nightly))]
    fn mode(&self) -> u32 //rip stable
    {
	macro_rules! map {
	    ($bit:expr, $bt:path, $constant:ident) => {
		if $bit.contains($bt) {
		    $constant
		} else {
		    0
		}
	    }
	}
	match self {
	    Self::Owner(bit) => {
		0u32 |
		map!(bit, Bit::Read, MODE_USER_READ) |
		map!(bit, Bit::Write, MODE_USER_WRITE) |
		map!(bit, Bit::Execute, MODE_USER_EXECUTE)
	    },
	    Self::Group(bit) => {
		0u32 |
		map!(bit, Bit::Read, MODE_GROUP_READ) |
		map!(bit, Bit::Write, MODE_GROUP_WRITE) |
		map!(bit, Bit::Execute, MODE_GROUP_EXECUTE)
	    },
	    Self::Other(bit) => {
		0u32 |
		map!(bit, Bit::Read, MODE_OTHER_READ) |
		map!(bit, Bit::Write, MODE_OTHER_WRITE) |
		map!(bit, Bit::Execute, MODE_OTHER_EXECUTE)
	    },
	}
    }

    #[cfg(nightly)]
    const fn mask_mode(&self, bit: u32) -> Bit
    {
	macro_rules! map {
	    ($bit:expr, $bt:path, $constant:ident) => {
		if ($bit & $constant) == $constant {$bt.bits()} else {0u32}
	    }
	}
	
	Bit::from_bits_truncate(match self {
	    Self::Owner(_) => {
		map!(bit, Bit::Read, MODE_USER_READ) |
		map!(bit, Bit::Write, MODE_USER_WRITE) |
		map!(bit, Bit::Execute, MODE_USER_EXECUTE)
	    },
	    Self::Group(_) => {
		map!(bit, Bit::Read, MODE_GROUP_READ) |
		map!(bit, Bit::Write, MODE_GROUP_WRITE) |
		map!(bit, Bit::Execute, MODE_GROUP_EXECUTE)
	    },
	    Self::Other(_) => {
		map!(bit, Bit::Read, MODE_OTHER_READ) |
		map!(bit, Bit::Write, MODE_OTHER_WRITE) |
		map!(bit, Bit::Execute, MODE_OTHER_EXECUTE)
	    },
	})
    }
    
    #[cfg(not(nightly))] 
    fn mask_mode(&self, bit: u32) -> Bit
    {
	macro_rules! map {
	    ($bit:expr, $bt:path, $constant:ident) => {
		if ($bit & $constant) == $constant {$bt.bits()} else {0u32}
	    }
	}
	
	Bit::from_bits_truncate(match self {
	    Self::Owner(_) => {
		map!(bit, Bit::Read, MODE_USER_READ) |
		map!(bit, Bit::Write, MODE_USER_WRITE) |
		map!(bit, Bit::Execute, MODE_USER_EXECUTE)
	    },
	    Self::Group(_) => {
		map!(bit, Bit::Read, MODE_GROUP_READ) |
		map!(bit, Bit::Write, MODE_GROUP_WRITE) |
		map!(bit, Bit::Execute, MODE_GROUP_EXECUTE)
	    },
	    Self::Other(_) => {
		map!(bit, Bit::Read, MODE_OTHER_READ) |
		map!(bit, Bit::Write, MODE_OTHER_WRITE) |
		map!(bit, Bit::Execute, MODE_OTHER_EXECUTE)
	    },
	})
    }
}

const MODE_USER: u32 = 0o700;
const MODE_USER_READ: u32 = 0o400;
const MODE_USER_WRITE: u32 = 0o200;
const MODE_USER_EXECUTE: u32 = 0o100;

const MODE_GROUP: u32 = 0o70;
const MODE_GROUP_READ: u32 = 0o40;
const MODE_GROUP_WRITE: u32 = 0o20;
const MODE_GROUP_EXECUTE: u32 = 0o10;

const MODE_OTHER: u32 = 0o7;
const MODE_OTHER_READ: u32 = 0o4;
const MODE_OTHER_WRITE: u32 = 0o2;
const MODE_OTHER_EXECUTE: u32 = 0o1;

const MODE: u32 = 0o777;

bitflags!{
    /// The mode mask for each individual class of `User`.
    ///
    /// # Notes
    /// No this does not map directly to `mode_t`. Use `User.mode(bits)` and `User.from_mode(mode)`.
    pub struct Bit: u32 {
	const None = 0;
	const Read = 1;
	const Write = 2;
	const Execute = 4;
	const Mask = 7;
    }
}

bitflags!{
    // not supported, don't care
    struct StickyBit: u32{
	const None = 0;
	const Setuid = 0o40000;
	const Setgid = 0o20000;
	const SaveSwap = 0o10000;
	
    }
}

/// Permissions struct in readable format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Ord, PartialOrd)]
pub struct Permissions
{
    
    pub u_owner: Bit,
    pub u_group: Bit,
    pub u_other: Bit,
    
    _u_sticky: StickyBit, //idc about this either
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum User
{
    Owner,
    Group,
    Other,
}

impl User
{
    /// Convert this class's representation of `Bit` into the corresponding `mode_t`.
    pub const fn mode(self, mask: Bit) -> u32
    {
	match self {
	    User::Owner => Class::Owner(mask).mode(),
	    User::Group => Class::Group(mask).mode(),
	    User::Other => Class::Other(mask).mode(),
	}
    }

    /// Convert a `mode_t` `u32` into `Bit` with this class' user.
    pub const fn from_mode(self, mask: u32) -> Bit
    {
	match self {
	    User::Owner => Class::Owner(Bit::Mask).mask_mode(mask),
	    User::Group => Class::Group(Bit::Mask).mask_mode(mask),
	    User::Other => Class::Other(Bit::Mask).mask_mode(mask),
	}
    }
}

impl Default for Permissions
{
    #[inline] fn default() -> Self
    {
	Self::new()
    }
}

impl Permissions
{
    /// Create a new empty `Permissions` struct
    pub const fn new() -> Self
    {
	Self {
	    u_owner: Bit::None,
	    u_group: Bit::None,
	    u_other: Bit::None,
	    _u_sticky: StickyBit::None,
	}
    }
}

// Mask impl
impl Permissions
{
    #[inline] const fn c_owner(&self) -> Class
    {
	Class::Owner(self.u_owner)
    }
    #[inline] const fn c_group(&self) -> Class
    {
	Class::Group(self.u_group)
    }
    #[inline] const fn c_other(&self) -> Class
    {
	Class::Other(self.u_other)
    }

    /// Convert into `mode_t` repr.
    #[inline] pub const fn mask(&self) -> u32
    {
	self.c_owner().mode() |
	self.c_group().mode() |
	self.c_other().mode()
    }

    /// Convert from `mode_t` repr.
    #[inline] pub const fn from_mask(bit: u32) -> Self
    {
	Self{
	    u_owner: Class::Owner(Bit::Mask).mask_mode(bit),
	    u_group: Class::Group(Bit::Mask).mask_mode(bit),
	    u_other: Class::Other(Bit::Mask).mask_mode(bit),

	    _u_sticky: StickyBit::None,
	}
    }

    /// Consume and add a mask.
    ///
    /// # Usage
    /// For builder pattern
    #[inline] pub const fn add_mask(self, class: User, bit: Bit) -> Self
    {
	match class {
	    User::Owner => {
		Self {
		    u_owner: Bit::from_bits_truncate(self.u_owner.bits() | bit.bits()),
		    ..self
		}
	    },
	    User::Group => {
		Self {
		    u_group: Bit::from_bits_truncate(self.u_group.bits() | bit.bits()),
		    ..self
		}
	    },
	    User::Other => {
		Self {
		    u_other: Bit::from_bits_truncate(self.u_other.bits() | bit.bits()),
		    ..self
		}
	    },
	}
    }

    
    /// Consume and remove a mask.
    ///
    /// # Usage
    /// For builder pattern
    #[inline] pub const fn remove_mask(self, class: User, bit: Bit) -> Self
    {
	match class {
	    User::Owner => {
		Self {
		    u_owner: Bit::from_bits_truncate(self.u_owner.bits() & !bit.bits()),
		    ..self
		}
	    },
	    User::Group => {
		Self {
		    u_group: Bit::from_bits_truncate(self.u_group.bits() & !bit.bits()),
		    ..self
		}
	    },
	    User::Other => {
		Self {
		    u_other: Bit::from_bits_truncate(self.u_other.bits() & !bit.bits()),
		    ..self
		}
	    },
	}
    }

    /// Returns true if the specified mode mask is all present
    #[inline] pub const fn has_mask(&self, class: User, bit: Bit) -> bool
    {
	match class {
	    User::Owner => {
		(self.u_owner.bits() & bit.bits()) == bit.bits()
	    },
	    User::Group => {
		(self.u_group.bits() & bit.bits()) == bit.bits()
	    },
	    User::Other => {
		(self.u_other.bits() & bit.bits()) == bit.bits()
	    },
	}
    }
}

impl From<Permissions> for u32
{
    #[inline] fn from(from: Permissions) -> Self
    {
	from.mask()
    }
}

impl From<u32> for Permissions
{
    #[inline] fn from(from: u32) -> Self
    {
	Self::from_mask(from)
    }
}

mod test;
