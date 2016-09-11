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

use std::collections::HashMap;
use color::color::Color;

lazy_static! {
    /// List of CSS3 named colors from http://www.w3.org/TR/css3-color.
    pub static ref NAMED_COLORS: HashMap<&'static str, Color> = {
        let mut m = HashMap::new();
        m.insert("transparent", Color { r: 0, g: 0, b: 0, a: 0.0 });
        m.insert("aliceblue", Color { r: 240, g: 248, b: 255, a: 1.0 });
        m.insert("antiquewhite", Color { r: 250, g: 235, b: 215, a: 1.0 });
        m.insert("aqua", Color { r: 0, g: 255, b: 255, a: 1.0 });
        m.insert("aquamarine", Color { r: 127, g: 255, b: 212, a: 1.0 });
        m.insert("azure", Color { r: 240, g: 255, b: 255, a: 1.0 });
        m.insert("beige", Color { r: 245, g: 245, b: 220, a: 1.0 });
        m.insert("bisque", Color { r: 255, g: 228, b: 196, a: 1.0 });
        m.insert("black", Color { r: 0, g: 0, b: 0, a: 1.0 });
        m.insert("blanchedalmond", Color { r: 255, g: 235, b: 205, a: 1.0 });
        m.insert("blue", Color { r: 0, g: 0, b: 255, a: 1.0 });
        m.insert("blueviolet", Color { r: 138, g: 43, b: 226, a: 1.0 });
        m.insert("brown", Color { r: 165, g: 42, b: 42, a: 1.0 });
        m.insert("burlywood", Color { r: 222, g: 184, b: 135, a: 1.0 });
        m.insert("cadetblue", Color { r: 95, g: 158, b: 160, a: 1.0 });
        m.insert("chartreuse", Color { r: 127, g: 255, b: 0, a: 1.0 });
        m.insert("chocolate", Color { r: 210, g: 105, b: 30, a: 1.0 });
        m.insert("coral", Color { r: 255, g: 127, b: 80, a: 1.0 });
        m.insert("cornflowerblue", Color { r: 100, g: 149, b: 237, a: 1.0 });
        m.insert("cornsilk", Color { r: 255, g: 248, b: 220, a: 1.0 });
        m.insert("crimson", Color { r: 220, g: 20, b: 60, a: 1.0 });
        m.insert("cyan", Color { r: 0, g: 255, b: 255, a: 1.0 });
        m.insert("darkblue", Color { r: 0, g: 0, b: 139, a: 1.0 });
        m.insert("darkcyan", Color { r: 0, g: 139, b: 139, a: 1.0 });
        m.insert("darkgoldenrod", Color { r: 184, g: 134, b: 11, a: 1.0 });
        m.insert("darkgray", Color { r: 169, g: 169, b: 169, a: 1.0 });
        m.insert("darkgreen", Color { r: 0, g: 100, b: 0, a: 1.0 });
        m.insert("darkgrey", Color { r: 169, g: 169, b: 169, a: 1.0 });
        m.insert("darkkhaki", Color { r: 189, g: 183, b: 107, a: 1.0 });
        m.insert("darkmagenta", Color { r: 139, g: 0, b: 139, a: 1.0 });
        m.insert("darkolivegreen", Color { r: 85, g: 107, b: 47, a: 1.0 });
        m.insert("darkorange", Color { r: 255, g: 140, b: 0, a: 1.0 });
        m.insert("darkorchid", Color { r: 153, g: 50, b: 204, a: 1.0 });
        m.insert("darkred", Color { r: 139, g: 0, b: 0, a: 1.0 });
        m.insert("darksalmon", Color { r: 233, g: 150, b: 122, a: 1.0 });
        m.insert("darkseagreen", Color { r: 143, g: 188, b: 143, a: 1.0 });
        m.insert("darkslateblue", Color { r: 72, g: 61, b: 139, a: 1.0 });
        m.insert("darkslategray", Color { r: 47, g: 79, b: 79, a: 1.0 });
        m.insert("darkslategrey", Color { r: 47, g: 79, b: 79, a: 1.0 });
        m.insert("darkturquoise", Color { r: 0, g: 206, b: 209, a: 1.0 });
        m.insert("darkviolet", Color { r: 148, g: 0, b: 211, a: 1.0 });
        m.insert("deeppink", Color { r: 255, g: 20, b: 147, a: 1.0 });
        m.insert("deepskyblue", Color { r: 0, g: 191, b: 255, a: 1.0 });
        m.insert("dimgray", Color { r: 105, g: 105, b: 105, a: 1.0 });
        m.insert("dimgrey", Color { r: 105, g: 105, b: 105, a: 1.0 });
        m.insert("dodgerblue", Color { r: 30, g: 144, b: 255, a: 1.0 });
        m.insert("firebrick", Color { r: 178, g: 34, b: 34, a: 1.0 });
        m.insert("floralwhite", Color { r: 255, g: 250, b: 240, a: 1.0 });
        m.insert("forestgreen", Color { r: 34, g: 139, b: 34, a: 1.0 });
        m.insert("fuchsia", Color { r: 255, g: 0, b: 255, a: 1.0 });
        m.insert("gainsboro", Color { r: 220, g: 220, b: 220, a: 1.0 });
        m.insert("ghostwhite", Color { r: 248, g: 248, b: 255, a: 1.0 });
        m.insert("gold", Color { r: 255, g: 215, b: 0, a: 1.0 });
        m.insert("goldenrod", Color { r: 218, g: 165, b: 32, a: 1.0 });
        m.insert("gray", Color { r: 128, g: 128, b: 128, a: 1.0 });
        m.insert("green", Color { r: 0, g: 128, b: 0, a: 1.0 });
        m.insert("greenyellow", Color { r: 173, g: 255, b: 47, a: 1.0 });
        m.insert("grey", Color { r: 128, g: 128, b: 128, a: 1.0 });
        m.insert("honeydew", Color { r: 240, g: 255, b: 240, a: 1.0 });
        m.insert("hotpink", Color { r: 255, g: 105, b: 180, a: 1.0 });
        m.insert("indianred", Color { r: 205, g: 92, b: 92, a: 1.0 });
        m.insert("indigo", Color { r: 75, g: 0, b: 130, a: 1.0 });
        m.insert("ivory", Color { r: 255, g: 255, b: 240, a: 1.0 });
        m.insert("khaki", Color { r: 240, g: 230, b: 140, a: 1.0 });
        m.insert("lavender", Color { r: 230, g: 230, b: 250, a: 1.0 });
        m.insert("lavenderblush", Color { r: 255, g: 240, b: 245, a: 1.0 });
        m.insert("lawngreen", Color { r: 124, g: 252, b: 0, a: 1.0 });
        m.insert("lemonchiffon", Color { r: 255, g: 250, b: 205, a: 1.0 });
        m.insert("lightblue", Color { r: 173, g: 216, b: 230, a: 1.0 });
        m.insert("lightcoral", Color { r: 240, g: 128, b: 128, a: 1.0 });
        m.insert("lightcyan", Color { r: 224, g: 255, b: 255, a: 1.0 });
        m.insert("lightgoldenrodyellow", Color { r: 250, g: 250, b: 210, a: 1.0 });
        m.insert("lightgray", Color { r: 211, g: 211, b: 211, a: 1.0 });
        m.insert("lightgreen", Color { r: 144, g: 238, b: 144, a: 1.0 });
        m.insert("lightgrey", Color { r: 211, g: 211, b: 211, a: 1.0 });
        m.insert("lightpink", Color { r: 255, g: 182, b: 193, a: 1.0 });
        m.insert("lightsalmon", Color { r: 255, g: 160, b: 122, a: 1.0 });
        m.insert("lightseagreen", Color { r: 32, g: 178, b: 170, a: 1.0 });
        m.insert("lightskyblue", Color { r: 135, g: 206, b: 250, a: 1.0 });
        m.insert("lightslategray", Color { r: 119, g: 136, b: 153, a: 1.0 });
        m.insert("lightslategrey", Color { r: 119, g: 136, b: 153, a: 1.0 });
        m.insert("lightsteelblue", Color { r: 176, g: 196, b: 222, a: 1.0 });
        m.insert("lightyellow", Color { r: 255, g: 255, b: 224, a: 1.0 });
        m.insert("lime", Color { r: 0, g: 255, b: 0, a: 1.0 });
        m.insert("limegreen", Color { r: 50, g: 205, b: 50, a: 1.0 });
        m.insert("linen", Color { r: 250, g: 240, b: 230, a: 1.0 });
        m.insert("magenta", Color { r: 255, g: 0, b: 255, a: 1.0 });
        m.insert("maroon", Color { r: 128, g: 0, b: 0, a: 1.0 });
        m.insert("mediumaquamarine", Color { r: 102, g: 205, b: 170, a: 1.0 });
        m.insert("mediumblue", Color { r: 0, g: 0, b: 205, a: 1.0 });
        m.insert("mediumorchid", Color { r: 186, g: 85, b: 211, a: 1.0 });
        m.insert("mediumpurple", Color { r: 147, g: 112, b: 219, a: 1.0 });
        m.insert("mediumseagreen", Color { r: 60, g: 179, b: 113, a: 1.0 });
        m.insert("mediumslateblue", Color { r: 123, g: 104, b: 238, a: 1.0 });
        m.insert("mediumspringgreen", Color { r: 0, g: 250, b: 154, a: 1.0 });
        m.insert("mediumturquoise", Color { r: 72, g: 209, b: 204, a: 1.0 });
        m.insert("mediumvioletred", Color { r: 199, g: 21, b: 133, a: 1.0 });
        m.insert("midnightblue", Color { r: 25, g: 25, b: 112, a: 1.0 });
        m.insert("mintcream", Color { r: 245, g: 255, b: 250, a: 1.0 });
        m.insert("mistyrose", Color { r: 255, g: 228, b: 225, a: 1.0 });
        m.insert("moccasin", Color { r: 255, g: 228, b: 181, a: 1.0 });
        m.insert("navajowhite", Color { r: 255, g: 222, b: 173, a: 1.0 });
        m.insert("navy", Color { r: 0, g: 0, b: 128, a: 1.0 });
        m.insert("oldlace", Color { r: 253, g: 245, b: 230, a: 1.0 });
        m.insert("olive", Color { r: 128, g: 128, b: 0, a: 1.0 });
        m.insert("olivedrab", Color { r: 107, g: 142, b: 35, a: 1.0 });
        m.insert("orange", Color { r: 255, g: 165, b: 0, a: 1.0 });
        m.insert("orangered", Color { r: 255, g: 69, b: 0, a: 1.0 });
        m.insert("orchid", Color { r: 218, g: 112, b: 214, a: 1.0 });
        m.insert("palegoldenrod", Color { r: 238, g: 232, b: 170, a: 1.0 });
        m.insert("palegreen", Color { r: 152, g: 251, b: 152, a: 1.0 });
        m.insert("paleturquoise", Color { r: 175, g: 238, b: 238, a: 1.0 });
        m.insert("palevioletred", Color { r: 219, g: 112, b: 147, a: 1.0 });
        m.insert("papayawhip", Color { r: 255, g: 239, b: 213, a: 1.0 });
        m.insert("peachpuff", Color { r: 255, g: 218, b: 185, a: 1.0 });
        m.insert("peru", Color { r: 205, g: 133, b: 63, a: 1.0 });
        m.insert("pink", Color { r: 255, g: 192, b: 203, a: 1.0 });
        m.insert("plum", Color { r: 221, g: 160, b: 221, a: 1.0 });
        m.insert("powderblue", Color { r: 176, g: 224, b: 230, a: 1.0 });
        m.insert("purple", Color { r: 128, g: 0, b: 128, a: 1.0 });
        m.insert("red", Color { r: 255, g: 0, b: 0, a: 1.0 });
        m.insert("rosybrown", Color { r: 188, g: 143, b: 143, a: 1.0 });
        m.insert("royalblue", Color { r: 65, g: 105, b: 225, a: 1.0 });
        m.insert("saddlebrown", Color { r: 139, g: 69, b: 19, a: 1.0 });
        m.insert("salmon", Color { r: 250, g: 128, b: 114, a: 1.0 });
        m.insert("sandybrown", Color { r: 244, g: 164, b: 96, a: 1.0 });
        m.insert("seagreen", Color { r: 46, g: 139, b: 87, a: 1.0 });
        m.insert("seashell", Color { r: 255, g: 245, b: 238, a: 1.0 });
        m.insert("sienna", Color { r: 160, g: 82, b: 45, a: 1.0 });
        m.insert("silver", Color { r: 192, g: 192, b: 192, a: 1.0 });
        m.insert("skyblue", Color { r: 135, g: 206, b: 235, a: 1.0 });
        m.insert("slateblue", Color { r: 106, g: 90, b: 205, a: 1.0 });
        m.insert("slategray", Color { r: 112, g: 128, b: 144, a: 1.0 });
        m.insert("slategrey", Color { r: 112, g: 128, b: 144, a: 1.0 });
        m.insert("snow", Color { r: 255, g: 250, b: 250, a: 1.0 });
        m.insert("springgreen", Color { r: 0, g: 255, b: 127, a: 1.0 });
        m.insert("steelblue", Color { r: 70, g: 130, b: 180, a: 1.0 });
        m.insert("tan", Color { r: 210, g: 180, b: 140, a: 1.0 });
        m.insert("teal", Color { r: 0, g: 128, b: 128, a: 1.0 });
        m.insert("thistle", Color { r: 216, g: 191, b: 216, a: 1.0 });
        m.insert("tomato", Color { r: 255, g: 99, b: 71, a: 1.0 });
        m.insert("turquoise", Color { r: 64, g: 224, b: 208, a: 1.0 });
        m.insert("violet", Color { r: 238, g: 130, b: 238, a: 1.0 });
        m.insert("wheat", Color { r: 245, g: 222, b: 179, a: 1.0 });
        m.insert("white", Color { r: 255, g: 255, b: 255, a: 1.0 });
        m.insert("whitesmoke", Color { r: 245, g: 245, b: 245, a: 1.0 });
        m.insert("yellow", Color { r: 255, g: 255, b: 0, a: 1.0 });
        m.insert("yellowgreen", Color { r: 154, g: 205, b: 50, a: 1.0 });
        m
    };
}
