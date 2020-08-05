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

#[test]
fn map()
{
    assert_eq!(&MAP[0o666], &Permissions::from_mask(0o666));
    #[cfg(feature="speedup_hack_stable")] 
    super::generate_build_map();
}


#[cfg(target_family="unix")]
#[test]
fn real_file()
{
    use std::fs::OpenOptions;
    
    let file = OpenOptions::new()
	.read(true)
	.open("Cargo.toml").expect("File not found");

    let perms = file.metadata().expect("Couldn't stat").permissions().unix();

    assert_eq!(perms, 0o644u32);
}
