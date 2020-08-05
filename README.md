# Readable Unix Permissions

Data types for the UNIX file permissions model. Convertable to and from `mode_t`.

## Usage

Crate is designed to give finer and more sane control over `mode_t`, so there are multiple ways of using the `Permissions` struct.

All the below functions are `const fn` on nightly.

### Creating from mode

``` rust
let perms = Permissions::from_mask(0o644); // Any value above 0u777 is truncated.
```

### Creating through builder-pattern

You can add masks with `add_mask()`
``` rust
let perms = Permissions::new()
	.add_mask(User::Owner, Bit::Read | Bit::Write)
	.add_mask(User::Group, Bit::Read);
```
And remove them with `remove_mask()`

``` rust
let perms = Permissions::from_mask(0o777)
	.remove_mask(User::Other, Bit::Write | Bit::Execute)
	.remove_mask(User::Group, Bit::Write);
```

### Checking 
You can check which modes are set with `has_mask()`.

``` rust
let perms = Permissions::from_mask(0o754);
assert!(perms.has_mask(User::Owner, Bit::Mask));
assert!(perms.has_mask(User::Group, Bit::Read | Bit::Execute));
assert!(perms.has_mask(User::Other, Bit::Read));
```

We also derive `PartialEq<u32>`, to compare with `mode_t` directly.

``` rust
let perms = Permissions::from_mask(0o644);

assert_eq!(perms, 0o644);
```

## Extension trait

We also define an extension trait on target family `unix` that follows `std::os::unix::fs::PermissionsExt`.
See [ext.rs] for details.

[ext.rs]: ./src/ext.rs

``` rust
use readable_perms::PermissionsExt as UnixPermsExt;
use std::os::unix::fs::PermissionsExt;

fn do_thing(file: &mut std::fs::File)
{
	let perms = file.metadata().unwrap().permissions().unix();
	println!("Perms are {}", perms);
}
```

## Performance
On nightly, most functions are `const fn` incuring no runtime cost for constant definitions. On stable, not so. Either way, we define a global `const` lookup table, so that conversions are as fast as a memory lookup.

Adding and removing the masks is usually 1 or two bitwise operations. TODO: Benchmark these?

### Benchmarks

 | Type                  | Value                |
 |-----------------------|----------------------|
 | Conversions from mode | 159 ns/iter (+/- 15) |
 | Conversions to mode   | 162 ns/iter (+/- 15) |

# License
GPL'd with <3
