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


