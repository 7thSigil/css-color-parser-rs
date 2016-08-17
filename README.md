Easy-to-use Rust parser for CSS3 color strings.
Handles all errors to avoid panic!s.

Not 100% spec compliant in the name of convenience (see examples below):
  * allows for extra whitespaces
  * allows for floats where standard allows percentages only
 
[W3C CSS3 Color spec](http://www.w3.org/TR/css3-color/)

[Mozilla CSS3 Color spec](https://developer.mozilla.org/en-US/docs/Web/CSS/color)

Repository:
https://github.com/7thSigil/css-color-parser-rs.git

##[Link to the Documentation](https://7thSigil.github.io/css-color-parser-rs/css_color_parser/index.html)

Cargo.toml:

[dependencies]
css-color-parser = *

//Example

extern crate css_color_parser;

//...

use css_color_parser::Color as CssColor;

let transparent_black = CssColor { r: 0, g: 0, b: 0, a: 1.0 };

println!("{:?}", " rgba (255, 128, 12, 0.5)".parse::<CssColor>()
    .unwrap_or(transparent_black));
//Color { r: 255, g: 128, b: 12, a: 0.5 }

println!("{:?}", "#fff".parse::<CssColor>()
    .unwrap_or(transparent_black));
//Color { r: 255, g: 255, b: 255, a: 1 }

println!("{:?}", "#ff0011".parse::<CssColor>()
    .unwrap_or(transparent_black));
//Color { r: 255, g: 0, b: 17, a: 1 }

println!("{:?}", "slateblue".parse::<CssColor>()
    .unwrap_or(transparent_black));
//Color { r: 106, g: 90, b: 205, a: 1 }

println!("{:?}", "blah".parse::<CssColor>()
    .unwrap_or(transparent_black));
//Color { r: 0, g: 0, b: 0, a: 0 } - ColorParseError

println!("{:?}", "ffffff".parse::<CssColor>()
    .unwrap_or(transparent_black));
//Color { r: 0, g: 0, b: 0, a: 0 } - ColorParseError

println!("{:?}", "hsla(900, 15%, 90%, 0.5)".parse::<CssColor>()
    .unwrap_or(transparent_black));
//Color { r: 226, g: 233, b: 233, a: 0.5 }

println!("{:?}", "hsla(900, 15%, 90%)".parse::<CssColor>()
    .unwrap_or(transparent_black));
//Color { r: 0, g: 0, b: 0, a: 0 } - ColorParseError

println!("{:?}", "hsl(900, 15%, 90%)".parse::<CssColor>()
    .unwrap_or(transparent_black));
//Color { r: 226, g: 233, b: 233, a: 1 }

// NOTE: not spec compliant.
println!("{:?}", "hsl(900, 0.15, 90%)".parse::<CssColor>()
    .unwrap_or(transparent_black)); 
//Color { r: 226, g: 233, b: 233, a: 1 }

(c) Dean McNamee <dean@gmail.com>, 2012.
(c) Katkov Oleksandr <alexx.katkoff@gmail.com>, 2016.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to
deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
IN THE SOFTWARE.