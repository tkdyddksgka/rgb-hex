# RGB to Hex

Simple Convert RGB color to hexadecimal color

## Usage
```toml
[dependencies]
rgb2hex = "0.2.0"
````

## Example

```rust
use rgb2hex::{rgb2hex::*, RGB};

assert_eq!(
    new(RGB {
        r: 251,
        g: 169,
        b: 12
    }),
    Ok(0xfba90c)
); // RGB -> Hex

assert_eq!(new_from_str("251 ,169,12"), Ok(0xfba90c)); // RGB (string) -> Hex

assert_eq!(new_from_arr(vec![251, 169, 12]), Ok(0xfba90c)); // RGB (array) -> Hex
```