// (c) Dean McNamee <dean@gmail.com>, 2012.
// (c) Rust port by Katkov Oleksandr <alexx.katkoff@gmail.com>, 2016.
//
// https://github.com/deanm/css-color-parser-js
// https://github.com/7thSigil/css-color-parser-rs
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use std::str;
use std::error;
use std::fmt;
use std::num;
use std::str::FromStr;

use color::named_colors::NAMED_COLORS;

/// Color in rgba format,
/// where {red,green,blue} in 0..255,
/// alpha in 0.0..1.0
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    /// red channel, ranges from 0 to 255
    pub r: u8,
    /// green channel, ranges from 0 to 255
    pub g: u8,
    /// blue channel, ranges from 0 to 255
    pub b: u8,
    /// alpha channel, ranges from 0.0 to 1.0
    pub a: f32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Color(r: {}, g: {}, b: {}, a: {})",
               self.r,
               self.g,
               self.b,
               self.a)
    }
}

#[derive(Debug)]
pub struct ColorParseError;

impl From<num::ParseIntError> for ColorParseError {
    #[allow(unused_variables)]
    fn from(err: num::ParseIntError) -> ColorParseError {
        return ColorParseError;
    }
}

impl From<num::ParseFloatError> for ColorParseError {
    #[allow(unused_variables)]
    fn from(err: num::ParseFloatError) -> ColorParseError {
        return ColorParseError;
    }
}

impl error::Error for ColorParseError {
    fn description(&self) -> &str {
        "Failed to parse color"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorParseError: Invalid format")
    }
}

// TODO(7thSigil): check if platform byte order affects parsing
// TODO(7thSigil): maybe rewrite error handling into something more informative?
/// Parses CSS3 color strings into rgba Color.
/// Handles all errors to avoid any panic!s
impl str::FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, ColorParseError> {
        let mut string = s.to_string();

        // Remove all whitespace, not compliant, but should just be more accepting.
        string = string.replace(" ", "")
            .to_lowercase();

        if string.is_empty() {
            return Err(ColorParseError);
        }

        let mut iterator = NAMED_COLORS.iter();

        // Color keywords (and transparent) lookup.
        while let Some(named_color) = iterator.next() {
            if named_color.name == string {
                return Ok(named_color.color);
            }
        }

        if string.starts_with("#") {
            let string_char_count = string.chars().count();

            if string_char_count == 4 {
                let (_, value_string) = string.split_at(1);

                let iv = try!(u64::from_str_radix(value_string, 16));

                // unlike original js code, NaN is impossible ()
                if !(iv <= 0xfff) {
                    return Err(ColorParseError);
                }

                return Ok(Color {
                    r: (((iv & 0xf00) >> 4) | ((iv & 0xf00) >> 8)) as u8,
                    g: ((iv & 0xf0) | ((iv & 0xf0) >> 4)) as u8,
                    b: ((iv & 0xf) | ((iv & 0xf) << 4)) as u8,
                    a: 1.0,
                });
            } else if string_char_count == 7 {
                let (_, value_string) = string.split_at(1);

                let iv = try!(u64::from_str_radix(value_string, 16));

                // (7thSigil) unlike original js code, NaN is impossible
                if !(iv <= 0xffffff) {
                    return Err(ColorParseError);
                }

                return Ok(Color {
                    r: ((iv & 0xff0000) >> 16) as u8,
                    g: ((iv & 0xff00) >> 8) as u8,
                    b: (iv & 0xff) as u8,
                    a: 1.0,
                });
            }

            return Err(ColorParseError);
        }

        let op = try!(string.find("(").ok_or(ColorParseError));
        let ep = try!(string.find(")").ok_or(ColorParseError));

        // (7thSigil) validating format
        // ')' bracket should be at the end
        // and always after the opening bracket
        if (ep + 1) != string.len() || ep < op {
            return Err(ColorParseError);
        }

        // (7thSigil) extracting format
        let (fmt, right_string_half) = string.split_at(op);

        // (7thSigil) validating format
        if fmt.is_empty() {
            return Err(ColorParseError);
        }

        // removing brackets
        let mut filtered_right_string_half = right_string_half.to_string();

        // removing brackets
        filtered_right_string_half.remove(0);
        filtered_right_string_half.pop();

        let params: Vec<&str> = filtered_right_string_half.split(",").collect();

        // (7thSigil) validating format
        if params.len() < 3 || params.len() > 4 {
            return Err(ColorParseError);
        }

        if fmt == "rgba" {
            return parse_rgba(params);
        } else if fmt == "rgb" {
            return parse_rgb(params);
        } else if fmt == "hsla" {
            return parse_hsla(params);
        } else if fmt == "hsl" {
            return parse_hsl(params);
        }

        return Err(ColorParseError);
    }
}

fn parse_rgba(mut rgba: Vec<&str>) -> Result<Color, ColorParseError> {

    if rgba.len() != 4 {
        return Err(ColorParseError);
    }

    let a_str = try!(rgba.pop().ok_or(ColorParseError));

    let a = try!(parse_css_float(a_str));

    let mut rgb_color = try!(parse_rgb(rgba));

    rgb_color = Color { a: a, ..rgb_color };

    return Ok(rgb_color);
}

fn parse_rgb(mut rgb: Vec<&str>) -> Result<Color, ColorParseError> {

    if rgb.len() != 3 {
        return Err(ColorParseError);
    }

    let b_str = try!(rgb.pop().ok_or(ColorParseError));
    let g_str = try!(rgb.pop().ok_or(ColorParseError));
    let r_str = try!(rgb.pop().ok_or(ColorParseError));

    let r = try!(parse_css_int(r_str));
    let g = try!(parse_css_int(g_str));
    let b = try!(parse_css_int(b_str));

    return Ok(Color {
        r: r,
        g: g,
        b: b,
        a: 1.0,
    });
}

fn parse_hsla(mut hsla: Vec<&str>) -> Result<Color, ColorParseError> {

    if hsla.len() != 4 {
        return Err(ColorParseError);
    }

    let a_str = try!(hsla.pop().ok_or(ColorParseError));

    let a = try!(parse_css_float(a_str));

    // (7thSigil) Parsed from hsl to rgb representation
    let mut rgb_color: Color = try!(parse_hsl(hsla));

    rgb_color = Color { a: a, ..rgb_color };

    return Ok(rgb_color);
}

fn parse_hsl(mut hsl: Vec<&str>) -> Result<Color, ColorParseError> {

    if hsl.len() != 3 {
        return Err(ColorParseError);
    }

    let l_str = try!(hsl.pop().ok_or(ColorParseError));
    let s_str = try!(hsl.pop().ok_or(ColorParseError));
    let h_str = try!(hsl.pop().ok_or(ColorParseError));

    let mut h = try!(f32::from_str(h_str));

    // 0 .. 1
    h = (((h % 360.0) + 360.0) % 360.0) / 360.0;

    // NOTE(deanm): According to the CSS spec s/l should only be
    // percentages, but we don't bother and let float or percentage.

    let s = try!(parse_css_float(s_str));
    let l = try!(parse_css_float(l_str));

    let m2: f32;

    if l <= 0.5 {
        m2 = l * (s + 1.0)
    } else {
        m2 = l + s - l * s;
    }

    let m1 = l * 2.0 - m2;

    let r = clamp_css_byte_from_float(css_hue_to_rgb(m1, m2, h + 1.0 / 3.0) * 255.0);
    let g = clamp_css_byte_from_float(css_hue_to_rgb(m1, m2, h) * 255.0);
    let b = clamp_css_byte_from_float(css_hue_to_rgb(m1, m2, h - 1.0 / 3.0) * 255.0);

    return Ok(Color {
        r: r,
        g: g,
        b: b,
        a: 1.0,
    });
}

// float or percentage.
fn parse_css_float(fv_str: &str) -> Result<f32, num::ParseFloatError> {

    let fv: f32;

    if fv_str.ends_with("%") {
        let mut percentage_string = fv_str.to_string();
        percentage_string.pop();
        fv = try!(f32::from_str(&percentage_string));
        return Ok(clamp_css_float(fv / 100.0));
    }

    fv = try!(f32::from_str(fv_str));
    return Ok(clamp_css_float(fv));
}

// int or percentage.
fn parse_css_int(iv_or_percentage_str: &str) -> Result<u8, ColorParseError> {
    if iv_or_percentage_str.ends_with("%") {

        let mut percentage_string = iv_or_percentage_str.to_string();
        percentage_string.pop();
        let fv = try!(f32::from_str(&percentage_string));
        // Seems to be what Chrome does (round vs truncation).
        return Ok(clamp_css_byte_from_float(fv / 100.0 * 255.0));
    }

    let iv = try!(u32::from_str(iv_or_percentage_str));

    return Ok(clamp_css_byte(iv));
}

// Clamp to float 0.0 .. 1.0.
fn clamp_css_float(fv: f32) -> f32 {
    // return fv < 0 ? 0 : fv > 1 ? 1 : fv;
    if fv < 0.0 {
        0.0
    } else if fv > 1.0 {
        1.0
    } else {
        fv
    }
}

fn clamp_css_byte_from_float(mut fv: f32) -> u8 {
    // Clamp to integer 0 .. 255.
    // Seems to be what Chrome does (vs truncation).
    fv = fv.round();

    // return iv < 0 ? 0 : iv > 255 ? 255 : iv;
    if fv < 0.0 {
        0
    } else if fv > 255.0 {
        255
    } else {
        fv as u8
    }
}

fn clamp_css_byte(iv: u32) -> u8 {
    // Clamp to integer 0 .. 255.
    // return iv < 0 ? 0 : iv > 255 ? 255 : iv;
    if iv > 255 { 255 } else { iv as u8 }
}

fn css_hue_to_rgb(m1: f32, m2: f32, mut h: f32) -> f32 {
    if h < 0.0 {
        h += 1.0;
    } else if h > 1.0 {
        h -= 1.0;
    }

    if h * 6.0 < 1.0 {
        return m1 + (m2 - m1) * h * 6.0;
    }
    if h * 2.0 < 1.0 {
        return m2;
    }
    if h * 3.0 < 2.0 {
        return m1 + (m2 - m1) * (2.0 / 3.0 - h) * 6.0;
    }

    return m1;
}
