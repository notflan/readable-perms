#![cfg(test)]

use super::*;

#[test]
fn mask()
{
    assert_eq!(Permissions::new()
	       .add_mask(User::Owner, Bit::Mask)
	       .add_mask(User::Group, Bit::Read | Bit::Execute)
	       .add_mask(User::Other, Bit::Read | Bit::Write)
	       .remove_mask(User::Other, Bit::Write)
	       .mask(),
	       0o754);
}

#[test]
fn from_mask()
{
    let mask = Permissions::from_mask(0o754);
    assert!(mask.has_mask(User::Owner, Bit::Mask));
    assert!(mask.has_mask(User::Group, Bit::Read | Bit::Execute));
    assert!(mask.has_mask(User::Other, Bit::Read));
}
