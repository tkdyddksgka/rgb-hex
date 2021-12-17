pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub mod rgb2hex {
    use crate::RGB;

    /// # Convert RGB to Hex
    /// ## Example
    /// ```rs
    /// assert_eq!(new(RGB {r: 251, g: 169, b: 12}), Ok(0xfba90c));
    /// ```
    pub fn new(value: RGB) -> Result<u32, String> {
        let colors = [value.r, value.g, value.b];
        let mut result = String::new();
        for color in colors.iter() {
            result.push_str(format!("{:02x?}", color).as_str());
        }
        Ok(match u32::from_str_radix(result.as_str(), 16) {
            Ok(v) => v,
            Err(e) => Err(e.to_string())?,
        })
    }

    /// # Convert RGB (str) to Hex
    /// ## Example
    /// ```rs
    /// assert_eq!(new_from_str("251 ,169, 12"), Ok(0xfba90c));
    /// ```
    pub fn new_from_str(value: &str) -> Result<u32, String> {
        let color = value
            .split(",")
            .map(|x| x.replace(" ", ""))
            .collect::<Vec<_>>();
        if color.len() != 3 {
            Err(format!("Invalid color format: {}", value))
        } else {
            let to_color = |v: &str| -> Result<u8, String> {
                match v.parse::<u8>() {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.to_string())?,
                }
            };
            new(RGB {
                r: to_color(color[0].as_str())?,
                g: to_color(color[1].as_str())?,
                b: to_color(color[2].as_str())?,
            })
        }
    }

    /// # Convert RGB (array) to Hex
    /// ## Example
    /// ```rs
    /// assert_eq!(new_from_arr(vec![251, 169, 12]), Ok(0xfba90c));
    /// ```
    pub fn new_from_arr(value: Vec<u8>) -> Result<u32, String> {
        if value.len() != 3 {
            Err(format!("Invalid color format: {:?}", value))
        } else {
            new(RGB {
                r: value[0],
                g: value[1],
                b: value[2],
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{rgb2hex::*, RGB};

    #[test]
    fn test() {
        assert_eq!(
            new(RGB {
                r: 251,
                g: 169,
                b: 12
            }),
            Ok(0xfba90c)
        );

        assert_eq!(new_from_str("251 ,169,12"), Ok(0xfba90c));

        assert_eq!(new_from_arr(vec![251, 169, 12]), Ok(0xfba90c));
    }
}
