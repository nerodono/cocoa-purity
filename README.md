# cocoa-purity

cocoa-purity is a collection of the useful procedural macros. 

P.S. named after the beautiful cat Cocoa

# Example

## map_enum

```rust
use cocoa_purity::map_enum;

#[derive(Debug)]
pub enum Quirk {
    AbilityToSleep,
    Nothing,
}

#[map_enum(Quirk)]
pub enum Character {
    Nero = AbilityToSleep,
    Custom { name: &'static str } = Nothing
}

let nero = Character::Nero;
let custom = Character::Custom { name: "Friend" };

let nero_quirk: Quirk = nero.into();
let custom_quirk: Quirk = custom.into();

dbg!(nero_quirk, custom_quirk);

```
