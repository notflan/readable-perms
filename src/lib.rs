//! Unix permissions `mode_t` in readable format.
//!
//No I don't care about Microshaft wangblows
#![cfg_attr(nightly, feature(test))]
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

    #[cfg(nightly)] const fn mask_mode(&self, bit: u32) -> Bit
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
    #[cfg(not(nightly))] fn mask_mode(&self, bit: u32) -> Bit
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
	/// No permissions for this class
	const None = 0;
	/// Read permissions for this class
	const Read = 1;
	/// Write permissions for this class
	const Write = 2;
	/// Execute permissions for this class
	///
	/// # Notes
	/// For directories, this translates to `can chdir to here'
	const Execute = 4;
	
	/// Read + Write + Execute. The whole mask.
	const Mask = 7;
    }
}

bitflags!{
    // not supported, don't care
    struct StickyBit: u32{
	const Empty = 0;
	const Setuid = 0o40000;
	const Setgid = 0o20000;
	const SaveSwap = 0o10000;
    }
}

/// Permissions struct in readable format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Ord, PartialOrd)]
#[repr(C)]
pub struct Permissions
{
    /// The `S_IRWXU` mask.
    ///
    /// # In English
    /// Read, write, and execute mask for owner of the file
    pub owner: Bit,
    
    /// The `S_IRWXG` mask.
    ///
    /// # In English
    /// Read, write, and execute mask for members of the owner's group
    pub group: Bit,
    
    /// The `S_IRWXO` mask.
    ///
    /// # In English
    /// Read, write, and execute mask for anyone else
    pub other: Bit,
    
    _sticky: StickyBit, //idc about this either
}

/// The class of user that UNIX permissions care about
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum User
{
    /// Owner of the file directly
    Owner,
    /// Members of the owner's group
    Group,
    /// Anyony else
    Other,
}

impl User
{
    /// Convert this class's representation of `Bit` into the corresponding `mode_t`.
    #[cfg(nightly)] pub const fn mode(self, mask: Bit) -> u32
    {
	match self {
	    User::Owner => Class::Owner(mask).mode(),
	    User::Group => Class::Group(mask).mode(),
	    User::Other => Class::Other(mask).mode(),
	}
    }
    /// Convert this class's representation of `Bit` into the corresponding `mode_t`.
    #[cfg(not(nightly))] pub fn mode(self, mask: Bit) -> u32
    {
	match self {
	    User::Owner => Class::Owner(mask).mode(),
	    User::Group => Class::Group(mask).mode(),
	    User::Other => Class::Other(mask).mode(),
	}
    }
    

    /// Convert a `mode_t` `u32` into `Bit` with this class' user.
    #[cfg(nightly)] pub const fn from_mode(self, mask: u32) -> Bit
    {
	match self {
	    User::Owner => Class::Owner(Bit::Mask).mask_mode(mask),
	    User::Group => Class::Group(Bit::Mask).mask_mode(mask),
	    User::Other => Class::Other(Bit::Mask).mask_mode(mask),
	}
    }
    
    /// Convert a `mode_t` `u32` into `Bit` with this class' user.
    #[cfg(not(nightly))] pub fn from_mode(self, mask: u32) -> Bit
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
	    owner: Bit::None,
	    group: Bit::None,
	    other: Bit::None,
	    _sticky: StickyBit::Empty,
	}
    }
}

// Mask impl
impl Permissions
{
    #[inline] const fn c_owner(&self) -> Class
    {
	Class::Owner(self.owner)
    }
    #[inline] const fn c_group(&self) -> Class
    {
	Class::Group(self.group)
    }
    #[inline] const fn c_other(&self) -> Class
    {
	Class::Other(self.other)
    }

    /// Convert into `mode_t` representation.
    #[inline] #[cfg(nightly)]  pub const fn mask(&self) -> u32
    {
	self.c_owner().mode() |
	self.c_group().mode() |
	self.c_other().mode()
    }
    
    /// Convert into `mode_t` representation.
    #[inline] #[cfg(not(nightly))] pub fn mask(&self) -> u32
    {
	self.c_owner().mode() |
	self.c_group().mode() |
	self.c_other().mode()
    }
    
    /// Convert from `mode_t` representation (slow).
    #[inline] #[cfg(nightly)]  const fn from_mask_calc(bit: u32) -> Self
    {
	Self{
	    owner: Class::Owner(Bit::Mask).mask_mode(bit),
	    group: Class::Group(Bit::Mask).mask_mode(bit),
	    other: Class::Other(Bit::Mask).mask_mode(bit),

	    _sticky: StickyBit::Empty,
	}
    }
    
    /// Convert from `mode_t` representation.
    #[inline] pub const fn from_mask(from: u32) -> Self
    {
	MAP[(from & MODE) as usize]
    }

    /// Consume and add a mask.
    ///
    /// # Usage
    /// For builder pattern
    #[inline] #[cfg(nightly)]  pub const fn add_mask(self, class: User, bit: Bit) -> Self
    {
	match class {
	    User::Owner => {
		Self {
		    owner: Bit::from_bits_truncate(self.owner.bits() | bit.bits()),
		    ..self
		}
	    },
	    User::Group => {
		Self {
		    group: Bit::from_bits_truncate(self.group.bits() | bit.bits()),
		    ..self
		}
	    },
	    User::Other => {
		Self {
		    other: Bit::from_bits_truncate(self.other.bits() | bit.bits()),
		    ..self
		}
	    },
	}
    }
    
    /// Consume and add a mask.
    ///
    /// # Usage
    /// For builder pattern
    #[inline] #[cfg(not(nightly))] pub fn add_mask(self, class: User, bit: Bit) -> Self
    {
	match class {
	    User::Owner => {
		Self {
		    owner: Bit::from_bits_truncate(self.owner.bits() | bit.bits()),
		    ..self
		}
	    },
	    User::Group => {
		Self {
		    group: Bit::from_bits_truncate(self.group.bits() | bit.bits()),
		    ..self
		}
	    },
	    User::Other => {
		Self {
		    other: Bit::from_bits_truncate(self.other.bits() | bit.bits()),
		    ..self
		}
	    },
	}
    }

    
    /// Consume and remove a mask.
    ///
    /// # Usage
    /// For builder pattern
    #[inline] #[cfg(nightly)] pub const fn remove_mask(self, class: User, bit: Bit) -> Self
    {
	match class {
	    User::Owner => {
		Self {
		    owner: Bit::from_bits_truncate(self.owner.bits() & !bit.bits()),
		    ..self
		}
	    },
	    User::Group => {
		Self {
		    group: Bit::from_bits_truncate(self.group.bits() & !bit.bits()),
		    ..self
		}
	    },
	    User::Other => {
		Self {
		    other: Bit::from_bits_truncate(self.other.bits() & !bit.bits()),
		    ..self
		}
	    },
	}
    }

    
    /// Consume and remove a mask.
    ///
    /// # Usage
    /// For builder pattern
    #[inline] #[cfg(not(nightly))] pub fn remove_mask(self, class: User, bit: Bit) -> Self
    {
	match class {
	    User::Owner => {
		Self {
		    owner: Bit::from_bits_truncate(self.owner.bits() & !bit.bits()),
		    ..self
		}
	    },
	    User::Group => {
		Self {
		    group: Bit::from_bits_truncate(self.group.bits() & !bit.bits()),
		    ..self
		}
	    },
	    User::Other => {
		Self {
		    other: Bit::from_bits_truncate(self.other.bits() & !bit.bits()),
		    ..self
		}
	    },
	}
    }

    /// Returns true if the specified mode mask is all present
    #[inline] #[cfg(nightly)]  pub const fn has_mask(&self, class: User, bit: Bit) -> bool
    {
	match class {
	    User::Owner => {
		(self.owner.bits() & bit.bits()) == bit.bits()
	    },
	    User::Group => {
		(self.group.bits() & bit.bits()) == bit.bits()
	    },
	    User::Other => {
		(self.other.bits() & bit.bits()) == bit.bits()
	    },
	}
    }

    /// Returns true if the specified mode mask is all present
    #[inline] #[cfg(not(nightly))] pub fn has_mask(&self, class: User, bit: Bit) -> bool
    {
	match class {
	    User::Owner => {
		(self.owner.bits() & bit.bits()) == bit.bits()
	    },
	    User::Group => {
		(self.group.bits() & bit.bits()) == bit.bits()
	    },
	    User::Other => {
		(self.other.bits() & bit.bits()) == bit.bits()
	    },
	}
    }
}

impl From<Permissions> for u32
{
    #[inline] fn from(from: Permissions) -> Self
    {
	from.mask() //can we not do the `MAP` here ;~;
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
#[cfg(nightly)] mod bench;

const fn generate_struct() -> [Permissions; 512]
{
    #[cfg(nightly)] return {
	let mut i: u32 =0;
	let mut output = [Permissions::new(); 512];
	loop
	{
	    output[i as usize] = Permissions::from_mask_calc(i);
	    i+=1;
	    if i == 0o777 {break;}
	}
	output
    };

    #[cfg(not(nightly))] {
	stable::MAP
    }
}


#[cfg(feature="speedup_hack_stable")] 
mod output {
    use super::*;
    use std::{
	io::{
	    self,
	    Write,
	},
    };
    pub fn print_map_as_syntax_hack<W: Write+?Sized>(to: &mut W) -> io::Result<()>
    {
	fn print_bit<W: Write+?Sized>(to:&mut W, bit: &Bit) -> io::Result<()>
	{
	    macro_rules! dobit {
		($bit:ident, $p:path) => {
		    if $bit.contains($p) {
			write!(to, "| {}", $p.bits())?;
			
		    }
		}
	    }
	    write!(to, "Bit::from_bits_truncate(0u32 ")?;
	    dobit!(bit, Bit::Read);
	    dobit!(bit, Bit::Write);
	    dobit!(bit, Bit::Execute);
	    write!(to, "| 0)")?;
	    Ok(())
	}

	writeln!(to, "[")?;
	for perm in MAP.iter() {
	    write!(to, "Permissions {{owner: ")?;
	    print_bit(to,&perm.owner)?;
	    write!(to, ", group: ")?;
	    print_bit(to,&perm.group)?;
	    write!(to, ", other: ")?;
	    print_bit(to,&perm.other)?;
	    write!(to, ", _sticky: StickyBit::Empty}}, ")?;
	}
	writeln!(to, "]")?;
	Ok(())
    }
}

#[cfg(feature="speedup_hack_stable")]
fn print_map_as_syntax_hack()
{
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    output::print_map_as_syntax_hack(&mut stdout).unwrap();
}

/// Generates the build map for stable release.
#[cfg(feature="speedup_hack_stable")]
pub fn generate_build_map() {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    let mut file = OpenOptions::new()
	.create(true)
	.truncate(true)
	.write(true)
	.open("./src/stable/mod.rs").expect("Failed to open file");

    writeln!(&mut file, "//! Hack for fast in stable with no `const fn` stuffs\n").unwrap();
    writeln!(&mut file, "use super::*;").unwrap();

    writeln!(&mut file, "pub const MAP: [Permissions; 512] = ").unwrap();
    output::print_map_as_syntax_hack(&mut file).expect("Failed to write");
    writeln!(&mut file, ";").unwrap();
}

#[cfg(not(nightly))] mod stable;

const MAP: [Permissions; 512] = generate_struct();

#[cfg(target_family="unix")]
mod ext;
#[cfg(target_family="unix")]
pub use ext::*;

// Boilerplate
use std::{
    borrow::Borrow,
    cmp::{PartialEq,Eq},
};

impl AsRef<Permissions> for u32
{
    fn as_ref(&self) -> &Permissions
    {
	&MAP[(*self & 0o777u32) as usize]
    }
}

impl AsRef<Permissions> for Permissions
{
    fn as_ref(&self) -> &Permissions
    {
	&self
    }
}

impl Borrow<Permissions> for u32
{
    fn borrow(&self) -> &Permissions
    {
	self.as_ref()
    }
}

impl PartialEq<u32> for Permissions
{
    fn eq(&self, other: &u32) -> bool
    {
	&Self::from(*other) == self
    }
}

impl PartialEq<Permissions> for u32
{
    fn eq(&self, other: &Permissions) -> bool
    {
	&Self::from(*self) == other
    }
}

impl std::fmt::Display for Permissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
	println!("0{}", self.mask())
    }
}
