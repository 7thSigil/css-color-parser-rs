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

use color::Color;

#[test]
fn rgb() {
	let c = " rgb(255, 128, 12)".parse::<Color>().unwrap();
	assert_eq!(c, Color { r: 255, g: 128, b: 12, a: 1.0 });
}

#[test]
fn rgba() {
	let c = " rgba (255, 128, 12, 0.5)".parse::<Color>().unwrap();
	assert_eq!(c, Color { r: 255, g: 128, b: 12, a: 0.5 });
}

#[test]
fn abc() {
	let c = "#fff".parse::<Color>().unwrap();
	assert_eq!(c, Color { r: 255, g: 255, b: 255, a: 1.0 });
}

#[test]
fn abc123() {
	let c = "#ff0011".parse::<Color>().unwrap();
	assert_eq!(c, Color { r: 255, g: 0, b: 17, a: 1.0 });
}

#[test]
fn named_color() {
	let c = "slateblue".parse::<Color>().unwrap();
	assert_eq!(c, Color { r: 106, g: 90, b: 205, a: 1.0 });
}

#[test]
#[should_panic]
#[allow(unused_variables)]
fn invalid_color1() {
	let c = "blah".parse::<Color>().unwrap();
}

#[test]
#[should_panic]
#[allow(unused_variables)]
fn invalid_color2() {
	let c = "ffffff".parse::<Color>().unwrap();
}

#[test]
fn hsla() {
	let c = "hsla(900, 15%, 90%, 0.5)".parse::<Color>().unwrap();
	assert_eq!(c, Color { r: 226, g: 233, b: 233, a: 0.5 });
}

#[test]
#[should_panic]
#[allow(unused_variables)]
fn hsla_invalid() {
	let c = "hsla(900, 15%, 90%)".parse::<Color>().unwrap();
}

#[test]
fn hsl() {
	let c = "hsl(900, 15%, 90%)".parse::<Color>().unwrap();
	assert_eq!(c, Color { r: 226, g: 233, b: 233, a: 1.0 });
}

#[test]
fn hsl_non_spec_compliant() {
	let c = "hsl(900, 0.15, 90%)".parse::<Color>().unwrap();
	assert_eq!(c, Color { r: 226, g: 233, b: 233, a: 1.0 });
}