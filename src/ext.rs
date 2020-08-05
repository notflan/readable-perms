//! Ext for unixes
use super::*;

use std::os::unix::fs::PermissionsExt as UnixPermsExt;

use std::borrow::Borrow;

pub trait PermissionsExt
{
    fn unix(&self) -> Permissions;
    fn set_unix(&mut self, perm: impl Borrow<Permissions>);
    fn from_unix(perm: impl Into<Permissions>) -> Self;
}

impl PermissionsExt for std::fs::Permissions
{
    #[inline] fn unix(&self) -> Permissions
    {
	self.mode().into()
    }
    #[inline] fn set_unix(&mut self, perm: impl Borrow<Permissions>)
    {
	self.set_mode(perm.borrow().mask());
    }
    #[inline] fn from_unix(perm: impl Into<Permissions>) -> Self
    {
	Self::from_mode(perm.into().into())
    }
}


#[cfg(feature="chmod")] 
mod chmod
{
    use libc::{
	fchmod,
	chmod,
    };

    use std::{
	path::Path,
	io::{
	    self,
	    ErrorKind,
	},
    };
    
    pub trait FChmodExt
    {
	fn chmod(&mut self, mode: impl Into<u32>) -> io::Result<()>;
    }

    impl FChmodExt for std::fs::File
    {
	
	/// Perform `chmod` on this file to `mode`.
	///
	/// Mode can be anything that implements `Into<u32>`. `Permissions` does this, you can also pass raw `mode_t` values.
	/// # Notes
	/// If you pass raw `mode_t` that is outside the range (0..=0o777), any extra bits are ignored.
	fn chmod(&mut self, mode: impl Into<u32>) -> io::Result<()>
	{
	    use std::os::unix::io::*;
	    unsafe {
		if fchmod(self.as_raw_fd(), mode.into() & 0o777) == 0 {
		    Ok(())
		} else {
		    Err(io::Error::new(ErrorKind::Other, "fchmod failed"))
		}
	    }
	}
    }

    pub trait ChmodExt
    {
	fn chmod(&self, mode: impl Into<u32>) -> io::Result<()>;
    }

    impl<P> ChmodExt for P
    where P: AsRef<Path>
    {
	/// Perform `chmod` on this Path to `mode`.
	///
	/// Mode can be anything that implements `Into<u32>`. `Permissions` does this, you can also pass raw `mode_t` values.
	/// # Notes
	/// If you pass raw `mode_t` that is outside the range (0..=0o777), any extra bits are ignored.
	fn chmod(&self, mode: impl Into<u32>) -> io::Result<()>
	{
	    use std::os::unix::ffi::OsStrExt;
	    let bytes = self.as_ref().as_os_str().as_bytes();
	    unsafe {
		let path = std::ffi::CString::new(bytes).map_err(|_| io::Error::new(ErrorKind::Other, "invalid path"))?;
		if chmod(path.as_ptr(), mode.into() & 0o777) == 0 {
		    Ok(())
		} else {
		    Err(io::Error::new(ErrorKind::Other, "chmod failed"))
		}
	    }
	}
    }
}
#[cfg(feature="chmod")] 
pub use chmod::*;
