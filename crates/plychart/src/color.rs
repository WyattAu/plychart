//! Minimal zero-dependency CSS color parser.
//!
//! Parses the color syntaxes plychart themes use in practice:
//! hex (`#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`), functional notation
//! (`rgb()`/`rgba()` with commas, spaces, percentages and `/`-separated
//! alpha), and the basic CSS named colors. Out-of-range channel values are
//! clamped, matching CSS behavior.

/// A parsed CSS color: RGBA channels in `0..=255`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
    /// Alpha channel (255 = opaque).
    pub a: u8,
}

impl Rgba {
    /// The opaque (r, g, b) triple.
    #[must_use]
    pub fn rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

/// Parse a CSS color string. Returns `None` for anything unrecognized.
#[must_use]
pub fn parse_css_color(s: &str) -> Option<Rgba> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(body) = s.strip_prefix('#').or_else(|| {
        // Hex without '#' is accepted for JSON convenience, but only when the
        // whole string is a plausible hex run (rules out "abcdef" prose).
        let looks_hex =
            !s.starts_with('(') && s.len() <= 8 && s.chars().all(|c| c.is_ascii_hexdigit());
        looks_hex.then_some(s)
    }) {
        return parse_hex(body);
    }
    let lowered = s.to_ascii_lowercase();
    if let Some(body) = lowered.strip_suffix(')') {
        return parse_functional(body);
    }
    parse_named(&lowered)
}

fn parse_hex(body: &str) -> Option<Rgba> {
    if !body.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let v = |chunk: &str| u8::from_str_radix(chunk, 16).ok();
    match body.len() {
        3 | 4 => {
            let mut it = body.chars().filter_map(|c| c.to_digit(16));
            let r = it.next()? as u8;
            let g = it.next()? as u8;
            let b = it.next()? as u8;
            let a = it.next();
            let dup = |n: u8| n * 17;
            Some(Rgba {
                r: dup(r),
                g: dup(g),
                b: dup(b),
                a: a.map_or(255, |d| dup(d as u8)),
            })
        }
        6 | 8 => {
            let a = if body.len() == 8 {
                v(&body[6..8])?
            } else {
                255
            };
            Some(Rgba {
                r: v(&body[0..2])?,
                g: v(&body[2..4])?,
                b: v(&body[4..6])?,
                a,
            })
        }
        _ => None,
    }
}

fn parse_functional(body: &str) -> Option<Rgba> {
    let (name, args) = body.split_once('(')?;
    let nums: Vec<&str> = args
        .split(|c: char| c == ',' || c == '/' || c.is_whitespace())
        .filter(|t| !t.is_empty())
        .collect();
    if !matches!(name, "rgb" | "rgba") {
        return None;
    }
    if !matches!(nums.len(), 3 | 4) {
        return None;
    }
    let chan = |t: &str| -> Option<f64> {
        let x = if let Some(pct) = t.strip_suffix('%') {
            pct.trim().parse::<f64>().ok()? * 255.0 / 100.0
        } else {
            t.parse::<f64>().ok()?
        };
        x.is_finite().then_some(x)
    };
    let c = |t: &str| -> Option<u8> { chan(t).map(|x| x.clamp(0.0, 255.0).round() as u8) };
    let alpha = if nums.len() == 4 {
        let t = nums[3];
        let a = if let Some(pct) = t.strip_suffix('%') {
            pct.trim().parse::<f64>().ok()? / 100.0
        } else {
            t.parse::<f64>().ok()?
        };
        if !a.is_finite() {
            return None;
        }
        (a.clamp(0.0, 1.0) * 255.0).round() as u8
    } else {
        255
    };
    Some(Rgba {
        r: c(nums[0])?,
        g: c(nums[1])?,
        b: c(nums[2])?,
        a: alpha,
    })
}

fn parse_named(name: &str) -> Option<Rgba> {
    let (r, g, b, a) = match name {
        "black" => (0, 0, 0, 255),
        "silver" => (192, 192, 192, 255),
        "gray" | "grey" => (128, 128, 128, 255),
        "white" => (255, 255, 255, 255),
        "maroon" => (128, 0, 0, 255),
        "red" => (255, 0, 0, 255),
        "purple" => (128, 0, 128, 255),
        "fuchsia" | "magenta" => (255, 0, 255, 255),
        "green" => (0, 128, 0, 255),
        "lime" => (0, 255, 0, 255),
        "olive" => (128, 128, 0, 255),
        "yellow" => (255, 255, 0, 255),
        "navy" => (0, 0, 128, 255),
        "blue" => (0, 0, 255, 255),
        "teal" => (0, 128, 128, 255),
        "aqua" | "cyan" => (0, 255, 255, 255),
        "orange" => (255, 165, 0, 255),
        "transparent" => (0, 0, 0, 0),
        _ => return None,
    };
    Some(Rgba { r, g, b, a })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Option<Rgba> {
        parse_css_color(s)
    }

    #[test]
    fn hex_6() {
        assert_eq!(hex("#ff0000").unwrap().rgb(), (255, 0, 0));
        assert_eq!(hex("#00e5ff").unwrap().rgb(), (0, 229, 255));
        assert_eq!(hex("#0c0c0c").unwrap().rgb(), (12, 12, 12));
    }

    #[test]
    fn hex_8() {
        let c = hex("#ff000080").unwrap();
        assert_eq!(c.rgb(), (255, 0, 0));
        assert_eq!(c.a, 128);
        assert_eq!(hex("#ffffffff").unwrap().a, 255);
        assert_eq!(hex("#00000000").unwrap().a, 0);
    }

    #[test]
    fn hex_3_and_4() {
        assert_eq!(hex("#f00").unwrap().rgb(), (255, 0, 0));
        assert_eq!(hex("#abc").unwrap().rgb(), (170, 187, 204));
        let c = hex("#f00c").unwrap();
        assert_eq!(c.rgb(), (255, 0, 0));
        assert_eq!(c.a, 204);
    }

    #[test]
    fn hex_optional_hash_and_case() {
        assert_eq!(hex("ff0000").unwrap().rgb(), (255, 0, 0));
        assert_eq!(hex("F00").unwrap().rgb(), (255, 0, 0));
        assert_eq!(hex("#AbCdEf").unwrap().rgb(), (171, 205, 239));
    }

    #[test]
    fn hex_invalid() {
        for bad in [
            "",
            "#",
            "#ff",
            "#fffff",
            "#fffffff",
            "#gggggg",
            "#ff000",
            "xyz",
            "#12345",
            "#ff0000ff00",
            "words here",
        ] {
            assert!(hex(bad).is_none(), "expected None for {bad:?}");
        }
    }

    #[test]
    fn rgb_commas() {
        assert_eq!(hex("rgb(255,0,0)").unwrap().rgb(), (255, 0, 0));
        assert_eq!(hex("rgb( 12 , 34 , 56 )").unwrap().rgb(), (12, 34, 56));
        assert_eq!(hex("RGB(10,20,30)").unwrap().rgb(), (10, 20, 30));
    }

    #[test]
    fn rgba_alpha() {
        assert_eq!(hex("rgba(0,229,255,1)").unwrap().a, 255);
        assert_eq!(hex("rgba(0,229,255,0)").unwrap().a, 0);
        assert_eq!(hex("rgba(0,0,0,0.5)").unwrap().a, 128);
        assert_eq!(hex("rgb(0 0 0 / 0.5)").unwrap().a, 128);
        assert_eq!(hex("rgb(255 255 255 / 50%)").unwrap().a, 128);
    }

    #[test]
    fn rgb_space_syntax() {
        assert_eq!(hex("rgb(255 0 0)").unwrap().rgb(), (255, 0, 0));
        assert_eq!(hex("rgb(1 2 3 / 1)").unwrap().rgb(), (1, 2, 3));
    }

    #[test]
    fn rgb_percentages() {
        assert_eq!(hex("rgb(100%,0%,0%)").unwrap().rgb(), (255, 0, 0));
        assert_eq!(hex("rgb(50% 50% 50%)").unwrap().rgb(), (128, 128, 128));
    }

    #[test]
    fn rgb_clamps_out_of_range_like_css() {
        assert_eq!(hex("rgb(300,0,0)").unwrap().rgb(), (255, 0, 0));
        assert_eq!(hex("rgb(-5,0,0)").unwrap().rgb(), (0, 0, 0));
        assert_eq!(hex("rgba(0,0,0,2)").unwrap().a, 255);
        assert_eq!(hex("rgba(0,0,0,-1)").unwrap().a, 0);
        assert_eq!(hex("rgb(120%,50%,0%)").unwrap().rgb(), (255, 128, 0));
    }

    #[test]
    fn rgb_boundary_channels() {
        assert_eq!(hex("rgb(0,0,0)").unwrap().rgb(), (0, 0, 0));
        assert_eq!(hex("rgb(255,255,255)").unwrap().rgb(), (255, 255, 255));
    }

    #[test]
    fn rgba_is_an_alias_of_rgb_in_css4() {
        assert_eq!(hex("rgba(1,2,3)").unwrap().rgb(), (1, 2, 3));
        assert_eq!(hex("rgba(1,2,3)").unwrap().a, 255);
        assert_eq!(hex("rgb(1,2,3,0.5)").unwrap().a, 128);
    }

    #[test]
    fn rgb_invalid() {
        for bad in [
            "rgb()",
            "rgb(1,2)",
            "rgb(1,2,3,4,5)",
            "rgb(a,b,c)",
            "rgb(1,2,3",
            "hsl(1,2,3)",
            "rgb(1;2;3)",
            "rgb(NaN,0,0)",
            "rgb(1,2,NaN)",
            "rgba(0,0,0,NaN)",
            "()",
        ] {
            assert!(hex(bad).is_none(), "expected None for {bad:?}");
        }
    }

    #[test]
    fn named_colors() {
        assert_eq!(hex("white").unwrap().rgb(), (255, 255, 255));
        assert_eq!(hex("black").unwrap().rgb(), (0, 0, 0));
        assert_eq!(hex("red").unwrap().rgb(), (255, 0, 0));
        assert_eq!(hex("green").unwrap().rgb(), (0, 128, 0));
        assert_eq!(hex("blue").unwrap().rgb(), (0, 0, 255));
        assert_eq!(hex("cyan").unwrap().rgb(), (0, 255, 255));
        assert_eq!(hex("aqua").unwrap().rgb(), (0, 255, 255));
        assert_eq!(hex("magenta").unwrap().rgb(), (255, 0, 255));
        assert_eq!(hex("gray").unwrap().rgb(), (128, 128, 128));
        assert_eq!(hex("grey").unwrap().rgb(), (128, 128, 128));
        assert_eq!(hex("orange").unwrap().rgb(), (255, 165, 0));
        assert_eq!(hex("transparent").unwrap().a, 0);
        assert_eq!(hex("White").unwrap().rgb(), (255, 255, 255));
        assert_eq!(hex("  RED  ").unwrap().rgb(), (255, 0, 0));
    }

    #[test]
    fn named_unknown_is_none() {
        assert!(hex("notacolor").is_none());
        assert!(hex("rebeccapurple").is_none());
        assert!(hex("whitish").is_none());
    }

    #[test]
    fn rgb_matches_hex_parse() {
        let inputs = [
            "#00e5ff",
            "rgb(0,229,255)",
            "rgb(0 229 255)",
            "rgba(0,229,255,100%)",
        ];
        let expected = hex("#00e5ff").unwrap();
        for i in inputs {
            assert_eq!(hex(i).unwrap(), expected, "mismatch for {i:?}");
        }
    }
}
