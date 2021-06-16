//! [Color] conversion functions.

use super::{
    Color, ColorError,
    ColorMode::{self, *},
};
use std::{borrow::Cow, convert::TryFrom, str::FromStr};

/// Convert to [Rgb] to [Hsb] format.
#[allow(clippy::many_single_char_names)]
fn rgb_to_hsb([r, g, b, a]: [f64; 4]) -> [f64; 4] {
    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let chr = c_max - c_min;
    if chr.abs() < f64::EPSILON {
        [0.0, 0.0, c_max, a]
    } else {
        let mut h = if (r - c_max).abs() < f64::EPSILON {
            // Magenta to yellow
            (g - b) / chr
        } else if (g - c_max).abs() < f64::EPSILON {
            // Yellow to cyan
            2.0 + (b - r) / chr
        } else {
            // Cyan to magenta
            4.0 + (r - g) / chr
        };
        if h < 0.0 {
            h += 6.0;
        } else if h >= 6.0 {
            h -= 6.0;
        }
        let s = chr / c_max;
        [h / 6.0, s, c_max, a]
    }
}

/// Convert to [Rgb] to [Hsl] format.
#[allow(clippy::many_single_char_names)]
fn rgb_to_hsl([r, g, b, a]: [f64; 4]) -> [f64; 4] {
    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let l = c_max + c_min;
    let chr = c_max - c_min;
    if chr.abs() < f64::EPSILON {
        [0.0, 0.0, l / 2.0, a]
    } else {
        let mut h = if (r - c_max).abs() < f64::EPSILON {
            // Magenta to yellow
            (g - b) / chr
        } else if (g - c_max).abs() < f64::EPSILON {
            // Yellow to cyan
            2.0 + (b - r) / chr
        } else {
            // Cyan to magenta
            4.0 + (r - g) / chr
        };
        if h < 0.0 {
            h += 6.0;
        } else if h >= 6.0 {
            h -= 6.0;
        }
        let s = if l < 1.0 { chr / l } else { chr / (2.0 - l) };
        [h / 6.0, s, l / 2.0, a]
    }
}

/// Convert to [Hsb] to [Rgb] format.
#[inline]
#[allow(clippy::many_single_char_names)]
fn hsb_to_rgb([h, s, b, a]: [f64; 4]) -> [f64; 4] {
    if b.abs() < f64::EPSILON {
        [0.0, 0.0, 0.0, a]
    } else if s.abs() < f64::EPSILON {
        [b, b, b, a]
    } else {
        let h = h * 6.0;
        let sector = h.floor() as usize;
        let tint1 = b * (1.0 - s);
        let tint2 = b * (1.0 - s * (h - sector as f64));
        let tint3 = b * (1.0 - s * (1.0 + sector as f64 - h));
        let (r, g, b) = match sector {
            // Yellow to green
            1 => (tint2, b, tint1),
            // Green to cyan
            2 => (tint1, b, tint3),
            // Cyan to blue
            3 => (tint1, tint2, b),
            // Blue to magenta
            4 => (tint3, tint1, b),
            // Magenta to red
            5 => (b, tint1, tint2),
            // Red to yellow (sector is 0 or 6)
            _ => (b, tint3, tint1),
        };
        [r, g, b, a]
    }
}

/// Convert to [Hsl] to [Rgb] format.
#[inline]
#[allow(clippy::many_single_char_names)]
fn hsl_to_rgb([h, s, l, a]: [f64; 4]) -> [f64; 4] {
    if s.abs() < f64::EPSILON {
        [l, l, l, a]
    } else {
        let b = if l < 0.5 {
            (1.0 + s) * l
        } else {
            l + s - l * s
        };
        let zest = 2.0 * l - b;
        let hzb_to_rgb = |mut h, z, b| -> f64 {
            if h < 0.0 {
                h += 6.0;
            } else if h >= 6.0 {
                h -= 6.0;
            }
            match h {
                // Red to yellow (increasing green)
                _ if h < 1.0 => z + (b - z) * h,
                // Yellow to cyan (greatest green)
                _ if h < 3.0 => b,
                // Cyan to blue (decreasing green)
                _ if h < 4.0 => z + (b - z) * (4.0 - h),
                // Blue to red (least green)
                _ => z,
            }
        };
        [
            hzb_to_rgb(h + 2.0, zest, b),
            hzb_to_rgb(h, zest, b),
            hzb_to_rgb(h - 2.0, zest, b),
            a,
        ]
    }
}

/// Convert to [Hsl] to [Hsb] format.
#[allow(clippy::many_single_char_names)]
fn hsl_to_hsb([h, s, l, a]: [f64; 4]) -> [f64; 4] {
    let b = if l < 0.5 {
        (1.0 + s) * l
    } else {
        l + s - l * s
    };
    let s = 2.0 * (b - l) / b;
    [h, s, b, a]
}

/// Convert to [Hsb] to [Hsl] format.
#[allow(clippy::many_single_char_names)]
fn hsb_to_hsl([h, s, b, a]: [f64; 4]) -> [f64; 4] {
    let l = (2.0 - s) * b / 2.0;
    let s = match l {
        _ if (l - 1.0).abs() < f64::EPSILON => 0.0,
        _ if l < 0.5 => s / 2.0 - s,
        _ => s * b / (2.0 - l * 2.0),
    };
    [h, s, l, a]
}

#[inline]
pub(crate) fn maxes(mode: ColorMode) -> [f64; 4] {
    match mode {
        Rgb => [255.0; 4],
        Hsb => [360.0, 100.0, 100.0, 1.0],
        Hsl => [360.0, 100.0, 100.0, 1.0],
    }
}

/// Convert levels from one format to another.
#[inline]
pub(crate) fn convert_levels(levels: [f64; 4], from: ColorMode, to: ColorMode) -> [f64; 4] {
    match (from, to) {
        (Hsb, Rgb) => hsb_to_rgb(levels),
        (Hsl, Rgb) => hsl_to_rgb(levels),
        (Rgb, Hsb) => rgb_to_hsb(levels),
        (Rgb, Hsl) => rgb_to_hsl(levels),
        (Hsb, Hsl) => hsb_to_hsl(levels),
        (Hsl, Hsb) => hsl_to_hsb(levels),
        (_, _) => levels,
    }
}

/// Convert levels to RGB channels.
#[inline]
pub(crate) fn calculate_channels(levels: [f64; 4]) -> [u8; 4] {
    let [r, g, b, a] = levels;
    let [r_max, g_max, b_max, a_max] = maxes(Rgb);
    [
        (r * r_max).round() as u8,
        (g * g_max).round() as u8,
        (b * b_max).round() as u8,
        (a * a_max).round() as u8,
    ]
}

impl Color {
    /// Constructs a `Color` by linear interpolating between two colors by a given amount between
    /// 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let from = rgb!(255, 0, 0);
    /// let to = rgb!(0, 100, 255);
    /// let lerped = from.lerp(to, 0.5);
    /// assert_eq!(lerped.channels(), [128, 50, 128, 255]);
    ///
    /// let from = rgb!(255, 0, 0);
    /// let to = hsb!(120.0, 80.0, 100.0, 0.5);
    /// let lerped = from.lerp(to, 0.25); // `to` is implicity converted to RGB
    /// assert_eq!(lerped.channels(), [204, 64, 13, 223]);
    /// ```
    pub fn lerp(self, other: Color, amt: f64) -> Self {
        let lerp = |start, stop, amt| amt * (stop - start) + start;

        let amt = amt.clamp(0.0, 1.0);
        let [v1, v2, v3, a] = self.levels();
        let [ov1, ov2, ov3, oa] = other.levels();
        let v1 = lerp(v1, ov1, amt).clamp(0.0, 1.0);
        let v2 = lerp(v2, ov2, amt).clamp(0.0, 1.0);
        let v3 = lerp(v3, ov3, amt).clamp(0.0, 1.0);
        let a = lerp(a, oa, amt).clamp(0.0, 1.0);
        // SAFETY: We clamp the inputs to between 0.0..=1.0 - upholding the levels invariant.
        unsafe { Self::from_raw(self.mode, v1, v2, v3, a) }
    }
}

impl FromStr for Color {
    type Err = ColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('#') {
            return Err(ColorError::InvalidString(Cow::from(s.to_owned())));
        }

        let mut channels: [u8; 4] = [0, 0, 0, 255];
        let parse_hex = |hex: &str| {
            if let Ok(value) = u8::from_str_radix(hex, 16) {
                Ok(value)
            } else {
                Err(ColorError::InvalidString(Cow::from(hex.to_owned())))
            }
        };

        let s = s.trim().to_lowercase();
        match s.len() - 1 {
            3 | 4 => {
                for (i, _) in s[1..].char_indices() {
                    let hex = parse_hex(&s[i + 1..i + 2])?;
                    channels[i] = (hex << 4) | hex;
                }
            }
            6 | 8 => {
                for (i, _) in s[1..].char_indices().step_by(2) {
                    channels[i / 2] = parse_hex(&s[i + 1..i + 3])?;
                }
            }
            _ => return Err(ColorError::InvalidString(Cow::from(s))),
        }

        Ok(Self::rgba(
            channels[0],
            channels[1],
            channels[2],
            channels[3],
        ))
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl From<u8> for Color {
    fn from(gray: u8) -> Self {
        Self::rgb(gray, gray, gray)
    }
}

impl From<(u8, u8)> for Color {
    fn from((gray, alpha): (u8, u8)) -> Self {
        Self::rgba(gray, gray, gray, alpha)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<[u8; 2]> for Color {
    fn from([gray, alpha]: [u8; 2]) -> Self {
        Self::rgba(gray, gray, gray, alpha)
    }
}

impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<[u8; 4]> for Color {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<Color> for [f64; 4] {
    fn from(color: Color) -> Self {
        color.levels()
    }
}

impl From<f32> for Color {
    fn from(gray: f32) -> Self {
        let gray = gray as f64;
        Self::hsb(gray, gray, gray)
    }
}

impl From<(f32, f32)> for Color {
    fn from((gray, alpha): (f32, f32)) -> Self {
        let gray = gray as f64;
        Self::hsba(gray, gray, gray, alpha as f64)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((h, s, v): (f32, f32, f32)) -> Self {
        Self::hsb(h as f64, s as f64, v as f64)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((h, s, v, a): (f32, f32, f32, f32)) -> Self {
        Self::hsba(h as f64, s as f64, v as f64, a as f64)
    }
}

impl From<[f32; 1]> for Color {
    fn from([gray]: [f32; 1]) -> Self {
        let gray = gray as f64;
        Self::hsb(gray, gray, gray)
    }
}

impl From<[f32; 2]> for Color {
    fn from([gray, alpha]: [f32; 2]) -> Self {
        let gray = gray as f64;
        Self::hsba(gray, gray, gray, alpha as f64)
    }
}

impl From<[f32; 3]> for Color {
    fn from([h, s, v]: [f32; 3]) -> Self {
        Self::hsb(h as f64, s as f64, v as f64)
    }
}

impl From<[f32; 4]> for Color {
    fn from([h, s, v, a]: [f32; 4]) -> Self {
        Self::hsba(h as f64, s as f64, v as f64, a as f64)
    }
}

impl From<f64> for Color {
    fn from(gray: f64) -> Self {
        Self::hsb(gray, gray, gray)
    }
}

impl From<(f64, f64)> for Color {
    fn from((gray, alpha): (f64, f64)) -> Self {
        Self::hsba(gray, gray, gray, alpha)
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from((h, s, v): (f64, f64, f64)) -> Self {
        Self::hsb(h, s, v)
    }
}

impl From<(f64, f64, f64, f64)> for Color {
    fn from((h, s, v, a): (f64, f64, f64, f64)) -> Self {
        Self::hsba(h, s, v, a)
    }
}

impl From<[f64; 1]> for Color {
    fn from([gray]: [f64; 1]) -> Self {
        Self::hsb(gray, gray, gray)
    }
}

impl From<[f64; 2]> for Color {
    fn from([gray, alpha]: [f64; 2]) -> Self {
        Self::hsba(gray, gray, gray, alpha)
    }
}

impl From<[f64; 3]> for Color {
    fn from([h, s, v]: [f64; 3]) -> Self {
        Self::hsb(h, s, v)
    }
}

impl From<[f64; 4]> for Color {
    fn from([h, s, v, a]: [f64; 4]) -> Self {
        Self::hsba(h, s, v, a)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{hsb, hsl, rgb};

    macro_rules! assert_approx_eq {
        ($c1:expr, $c2:expr) => {
            let [v1, v2, v3, a] = $c1.levels();
            let [ov1, ov2, ov3, oa] = $c2.levels();
            let v1d = v1 - ov1;
            let v2d = v2 - ov2;
            let v3d = v3 - ov3;
            let ad = a - oa;
            let e = 0.002;
            assert!(v1d < e, "v1: ({} - {}) < {}", v1, ov1, e);
            assert!(v2d < e, "v2: ({} - {}) < {}", v2, ov2, e);
            assert!(v3d < e, "v3: ({} - {}) < {}", v3, ov3, e);
            assert!(ad < e, "a: ({} - {}) < {}", a, oa, e);
        };
    }

    #[test]
    fn test_hsb_to_rgb() {
        assert_approx_eq!(hsb!(0.0, 0.0, 0.0), rgb!(0, 0, 0));
        assert_approx_eq!(hsb!(0.0, 0.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsb!(0.0, 100.0, 100.0), rgb!(255, 0, 0));
        assert_approx_eq!(hsb!(120.0, 100.0, 100.0), rgb!(0, 255, 0));
        assert_approx_eq!(hsb!(240.0, 100.0, 100.0), rgb!(0, 0, 255));
        assert_approx_eq!(hsb!(60.0, 100.0, 100.0), rgb!(255, 255, 0));
        assert_approx_eq!(hsb!(180.0, 100.0, 100.0), rgb!(0, 255, 255));
        assert_approx_eq!(hsb!(300.0, 100.0, 100.0), rgb!(255, 0, 255));
        assert_approx_eq!(hsb!(0.0, 0.0, 75.0), rgb!(191, 191, 191));
        assert_approx_eq!(hsb!(0.0, 0.0, 50.0), rgb!(128, 128, 128));
        assert_approx_eq!(hsb!(0.0, 100.0, 50.0), rgb!(128, 0, 0));
        assert_approx_eq!(hsb!(60.0, 100.0, 50.0), rgb!(128, 128, 0));
        assert_approx_eq!(hsb!(120.0, 100.0, 50.0), rgb!(0, 128, 0));
        assert_approx_eq!(hsb!(300.0, 100.0, 50.0), rgb!(128, 0, 128));
        assert_approx_eq!(hsb!(180.0, 100.0, 50.0), rgb!(0, 128, 128));
        assert_approx_eq!(hsb!(240.0, 100.0, 50.0), rgb!(0, 0, 128));
    }

    #[test]
    fn test_hsb_to_hsl() {
        assert_approx_eq!(hsb!(0.0, 0.0, 0.0), hsl!(0.0, 0.0, 0.0));
        assert_approx_eq!(hsb!(0.0, 0.0, 100.0), hsl!(0.0, 0.0, 100.0));
        assert_approx_eq!(hsb!(0.0, 100.0, 100.0), hsl!(0.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(120.0, 100.0, 100.0), hsl!(120.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(240.0, 100.0, 100.0), hsl!(240.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(60.0, 100.0, 100.0), hsl!(60.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(180.0, 100.0, 100.0), hsl!(180.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(300.0, 100.0, 100.0), hsl!(300.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(0.0, 0.0, 75.0), hsl!(0.0, 0.0, 75.0));
        assert_approx_eq!(hsb!(0.0, 0.0, 50.0), hsl!(0.0, 0.0, 50.0));
        assert_approx_eq!(hsb!(0.0, 100.0, 50.0), hsl!(0.0, 100.0, 25.0));
        assert_approx_eq!(hsb!(60.0, 100.0, 50.0), hsl!(60.0, 100.0, 25.0));
        assert_approx_eq!(hsb!(120.0, 100.0, 50.0), hsl!(120.0, 100.0, 25.0));
        assert_approx_eq!(hsb!(300.0, 100.0, 50.0), hsl!(300.0, 100.0, 25.0));
        assert_approx_eq!(hsb!(180.0, 100.0, 50.0), hsl!(180.0, 100.0, 25.0));
        assert_approx_eq!(hsb!(240.0, 100.0, 50.0), hsl!(240.0, 100.0, 25.0));
    }

    #[test]
    fn test_hsl_to_rgb() {
        assert_approx_eq!(hsl!(0.0, 0.0, 0.0), rgb!(0, 0, 0));
        assert_approx_eq!(hsl!(0.0, 0.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(0.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(120.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(240.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(60.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(180.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(300.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(0.0, 0.0, 75.0), rgb!(191, 191, 191));
        assert_approx_eq!(hsl!(0.0, 0.0, 50.0), rgb!(128, 128, 128));
        assert_approx_eq!(hsl!(0.0, 100.0, 50.0), rgb!(255, 0, 0));
        assert_approx_eq!(hsl!(60.0, 100.0, 50.0), rgb!(255, 255, 0));
        println!("{:?}", hsl!(120.0, 100.0, 50.0));
        println!("{:?}", rgb!(0, 255, 0));
        assert_approx_eq!(hsl!(120.0, 100.0, 50.0), rgb!(0, 255, 0));
        assert_approx_eq!(hsl!(300.0, 100.0, 50.0), rgb!(255, 0, 255));
        assert_approx_eq!(hsl!(180.0, 100.0, 50.0), rgb!(0, 255, 255));
        assert_approx_eq!(hsl!(240.0, 100.0, 50.0), rgb!(0, 0, 255));
    }

    #[test]
    fn test_hsl_to_hsb() {
        assert_approx_eq!(hsl!(0.0, 0.0, 0.0), hsb!(0.0, 0.0, 0.0));
        assert_approx_eq!(hsl!(0.0, 0.0, 100.0), hsb!(255.0, 255.0, 255.0));
        assert_approx_eq!(hsl!(0.0, 100.0, 100.0), hsb!(255.0, 0.0, 0.0));
        assert_approx_eq!(hsl!(120.0, 100.0, 100.0), hsb!(0.0, 255.0, 0.0));
        assert_approx_eq!(hsl!(240.0, 100.0, 100.0), hsb!(0.0, 0.0, 255.0));
        assert_approx_eq!(hsl!(60.0, 100.0, 100.0), hsb!(255.0, 255.0, 0.0));
        assert_approx_eq!(hsl!(180.0, 100.0, 100.0), hsb!(0.0, 255.0, 255.0));
        assert_approx_eq!(hsl!(300.0, 100.0, 100.0), hsb!(255.0, 0.0, 255.0));
        assert_approx_eq!(hsl!(0.0, 0.0, 75.0), hsb!(191.0, 191.0, 191.0));
        assert_approx_eq!(hsl!(0.0, 0.0, 50.0), hsb!(128.0, 128.0, 128.0));
        assert_approx_eq!(hsl!(0.0, 100.0, 50.0), hsb!(128.0, 0.0, 0.0));
        assert_approx_eq!(hsl!(60.0, 100.0, 50.0), hsb!(128.0, 128.0, 0.0));
        assert_approx_eq!(hsl!(120.0, 100.0, 50.0), hsb!(0.0, 128.0, 0.0));
        assert_approx_eq!(hsl!(300.0, 100.0, 50.0), hsb!(128.0, 0.0, 128.0));
        assert_approx_eq!(hsl!(180.0, 100.0, 50.0), hsb!(0.0, 128.0, 128.0));
        assert_approx_eq!(hsl!(240.0, 100.0, 50.0), hsb!(0.0, 0.0, 128.0));
    }

    #[test]
    fn test_rgb_to_hsb() {
        assert_approx_eq!(rgb!(0, 0, 0), hsb!(0.0, 0.0, 0.0));
        assert_approx_eq!(rgb!(255, 255, 255), hsb!(0.0, 0.0, 100.0));
        assert_approx_eq!(rgb!(255, 0, 0), hsb!(0.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 255, 0), hsb!(120.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 0, 255), hsb!(240.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(255, 255, 0), hsb!(60.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 255, 255), hsb!(180.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(255, 0, 255), hsb!(300.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(191, 191, 191), hsb!(0.0, 0.0, 74.9));
        assert_approx_eq!(rgb!(128, 128, 128), hsb!(0.0, 0.0, 50.0));
        assert_approx_eq!(rgb!(128, 0, 0), hsb!(0.0, 100.0, 50.2));
        assert_approx_eq!(rgb!(128, 128, 0), hsb!(60.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 128, 0), hsb!(120.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(128, 0, 128), hsb!(300.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 128, 128), hsb!(180.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 0, 128), hsb!(240.0, 100.0, 50.0));
    }

    #[test]
    fn test_rgb_to_hsl() {
        assert_approx_eq!(rgb!(0, 0, 0), hsl!(0.0, 0.0, 0.0));
        assert_approx_eq!(rgb!(255, 255, 255), hsl!(0.0, 0.0, 100.0));
        assert_approx_eq!(rgb!(255, 0, 0), hsl!(0.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 255, 0), hsl!(120.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 0, 255), hsl!(240.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(255, 255, 0), hsl!(60.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 255, 255), hsl!(180.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(255, 0, 255), hsl!(300.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(191, 191, 191), hsl!(0.0, 0.0, 74.9));
        assert_approx_eq!(rgb!(128, 128, 128), hsl!(0.0, 0.0, 50.0));
        assert_approx_eq!(rgb!(128, 0, 0), hsl!(0.0, 100.0, 50.2));
        assert_approx_eq!(rgb!(128, 128, 0), hsl!(60.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 128, 0), hsl!(120.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(128, 0, 128), hsl!(300.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 128, 128), hsl!(180.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 0, 128), hsl!(240.0, 100.0, 50.0));
    }
}
