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

use color::color::Color;

pub struct NamedColor {
	pub name : &'static str,
	pub color: Color
}

//(7thSigil) converted straight from css-color-parser-js with this:
// for (var key in kCSSColorTable) {
//     var obj = kCSSColorTable[key]; 
//     console.log("NamedColor { name: \"" + key + "\", color: Color { r: " + obj[0] + ", g: " + obj[1] + ", b: " + obj[2] + ", a: " + obj[3] + ".0 } },");
// }
/// List of CSS3 named colors
/// http://www.w3.org/TR/css3-color/
pub static NAMED_COLORS: [NamedColor; 148] = [
	NamedColor { name: "transparent", color: Color { r: 0, g: 0, b: 0, a: 0.0 } },
	NamedColor { name: "aliceblue", color: Color { r: 240, g: 248, b: 255, a: 1.0 } },
	NamedColor { name: "antiquewhite", color: Color { r: 250, g: 235, b: 215, a: 1.0 } },
	NamedColor { name: "aqua", color: Color { r: 0, g: 255, b: 255, a: 1.0 } },
	NamedColor { name: "aquamarine", color: Color { r: 127, g: 255, b: 212, a: 1.0 } },
	NamedColor { name: "azure", color: Color { r: 240, g: 255, b: 255, a: 1.0 } },
	NamedColor { name: "beige", color: Color { r: 245, g: 245, b: 220, a: 1.0 } },
	NamedColor { name: "bisque", color: Color { r: 255, g: 228, b: 196, a: 1.0 } },
	NamedColor { name: "black", color: Color { r: 0, g: 0, b: 0, a: 1.0 } },
	NamedColor { name: "blanchedalmond", color: Color { r: 255, g: 235, b: 205, a: 1.0 } },
	NamedColor { name: "blue", color: Color { r: 0, g: 0, b: 255, a: 1.0 } },
	NamedColor { name: "blueviolet", color: Color { r: 138, g: 43, b: 226, a: 1.0 } },
	NamedColor { name: "brown", color: Color { r: 165, g: 42, b: 42, a: 1.0 } },
	NamedColor { name: "burlywood", color: Color { r: 222, g: 184, b: 135, a: 1.0 } },
	NamedColor { name: "cadetblue", color: Color { r: 95, g: 158, b: 160, a: 1.0 } },
	NamedColor { name: "chartreuse", color: Color { r: 127, g: 255, b: 0, a: 1.0 } },
	NamedColor { name: "chocolate", color: Color { r: 210, g: 105, b: 30, a: 1.0 } },
	NamedColor { name: "coral", color: Color { r: 255, g: 127, b: 80, a: 1.0 } },
	NamedColor { name: "cornflowerblue", color: Color { r: 100, g: 149, b: 237, a: 1.0 } },
	NamedColor { name: "cornsilk", color: Color { r: 255, g: 248, b: 220, a: 1.0 } },
	NamedColor { name: "crimson", color: Color { r: 220, g: 20, b: 60, a: 1.0 } },
	NamedColor { name: "cyan", color: Color { r: 0, g: 255, b: 255, a: 1.0 } },
	NamedColor { name: "darkblue", color: Color { r: 0, g: 0, b: 139, a: 1.0 } },
	NamedColor { name: "darkcyan", color: Color { r: 0, g: 139, b: 139, a: 1.0 } },
	NamedColor { name: "darkgoldenrod", color: Color { r: 184, g: 134, b: 11, a: 1.0 } },
	NamedColor { name: "darkgray", color: Color { r: 169, g: 169, b: 169, a: 1.0 } },
	NamedColor { name: "darkgreen", color: Color { r: 0, g: 100, b: 0, a: 1.0 } },
	NamedColor { name: "darkgrey", color: Color { r: 169, g: 169, b: 169, a: 1.0 } },
	NamedColor { name: "darkkhaki", color: Color { r: 189, g: 183, b: 107, a: 1.0 } },
	NamedColor { name: "darkmagenta", color: Color { r: 139, g: 0, b: 139, a: 1.0 } },
	NamedColor { name: "darkolivegreen", color: Color { r: 85, g: 107, b: 47, a: 1.0 } },
	NamedColor { name: "darkorange", color: Color { r: 255, g: 140, b: 0, a: 1.0 } },
	NamedColor { name: "darkorchid", color: Color { r: 153, g: 50, b: 204, a: 1.0 } },
	NamedColor { name: "darkred", color: Color { r: 139, g: 0, b: 0, a: 1.0 } },
	NamedColor { name: "darksalmon", color: Color { r: 233, g: 150, b: 122, a: 1.0 } },
	NamedColor { name: "darkseagreen", color: Color { r: 143, g: 188, b: 143, a: 1.0 } },
	NamedColor { name: "darkslateblue", color: Color { r: 72, g: 61, b: 139, a: 1.0 } },
	NamedColor { name: "darkslategray", color: Color { r: 47, g: 79, b: 79, a: 1.0 } },
	NamedColor { name: "darkslategrey", color: Color { r: 47, g: 79, b: 79, a: 1.0 } },
	NamedColor { name: "darkturquoise", color: Color { r: 0, g: 206, b: 209, a: 1.0 } },
	NamedColor { name: "darkviolet", color: Color { r: 148, g: 0, b: 211, a: 1.0 } },
	NamedColor { name: "deeppink", color: Color { r: 255, g: 20, b: 147, a: 1.0 } },
	NamedColor { name: "deepskyblue", color: Color { r: 0, g: 191, b: 255, a: 1.0 } },
	NamedColor { name: "dimgray", color: Color { r: 105, g: 105, b: 105, a: 1.0 } },
	NamedColor { name: "dimgrey", color: Color { r: 105, g: 105, b: 105, a: 1.0 } },
	NamedColor { name: "dodgerblue", color: Color { r: 30, g: 144, b: 255, a: 1.0 } },
	NamedColor { name: "firebrick", color: Color { r: 178, g: 34, b: 34, a: 1.0 } },
	NamedColor { name: "floralwhite", color: Color { r: 255, g: 250, b: 240, a: 1.0 } },
	NamedColor { name: "forestgreen", color: Color { r: 34, g: 139, b: 34, a: 1.0 } },
	NamedColor { name: "fuchsia", color: Color { r: 255, g: 0, b: 255, a: 1.0 } },
	NamedColor { name: "gainsboro", color: Color { r: 220, g: 220, b: 220, a: 1.0 } },
	NamedColor { name: "ghostwhite", color: Color { r: 248, g: 248, b: 255, a: 1.0 } },
	NamedColor { name: "gold", color: Color { r: 255, g: 215, b: 0, a: 1.0 } },
	NamedColor { name: "goldenrod", color: Color { r: 218, g: 165, b: 32, a: 1.0 } },
	NamedColor { name: "gray", color: Color { r: 128, g: 128, b: 128, a: 1.0 } },
	NamedColor { name: "green", color: Color { r: 0, g: 128, b: 0, a: 1.0 } },
	NamedColor { name: "greenyellow", color: Color { r: 173, g: 255, b: 47, a: 1.0 } },
	NamedColor { name: "grey", color: Color { r: 128, g: 128, b: 128, a: 1.0 } },
	NamedColor { name: "honeydew", color: Color { r: 240, g: 255, b: 240, a: 1.0 } },
	NamedColor { name: "hotpink", color: Color { r: 255, g: 105, b: 180, a: 1.0 } },
	NamedColor { name: "indianred", color: Color { r: 205, g: 92, b: 92, a: 1.0 } },
	NamedColor { name: "indigo", color: Color { r: 75, g: 0, b: 130, a: 1.0 } },
	NamedColor { name: "ivory", color: Color { r: 255, g: 255, b: 240, a: 1.0 } },
	NamedColor { name: "khaki", color: Color { r: 240, g: 230, b: 140, a: 1.0 } },
	NamedColor { name: "lavender", color: Color { r: 230, g: 230, b: 250, a: 1.0 } },
	NamedColor { name: "lavenderblush", color: Color { r: 255, g: 240, b: 245, a: 1.0 } },
	NamedColor { name: "lawngreen", color: Color { r: 124, g: 252, b: 0, a: 1.0 } },
	NamedColor { name: "lemonchiffon", color: Color { r: 255, g: 250, b: 205, a: 1.0 } },
	NamedColor { name: "lightblue", color: Color { r: 173, g: 216, b: 230, a: 1.0 } },
	NamedColor { name: "lightcoral", color: Color { r: 240, g: 128, b: 128, a: 1.0 } },
	NamedColor { name: "lightcyan", color: Color { r: 224, g: 255, b: 255, a: 1.0 } },
	NamedColor { name: "lightgoldenrodyellow", color: Color { r: 250, g: 250, b: 210, a: 1.0 } },
	NamedColor { name: "lightgray", color: Color { r: 211, g: 211, b: 211, a: 1.0 } },
	NamedColor { name: "lightgreen", color: Color { r: 144, g: 238, b: 144, a: 1.0 } },
	NamedColor { name: "lightgrey", color: Color { r: 211, g: 211, b: 211, a: 1.0 } },
	NamedColor { name: "lightpink", color: Color { r: 255, g: 182, b: 193, a: 1.0 } },
	NamedColor { name: "lightsalmon", color: Color { r: 255, g: 160, b: 122, a: 1.0 } },
	NamedColor { name: "lightseagreen", color: Color { r: 32, g: 178, b: 170, a: 1.0 } },
	NamedColor { name: "lightskyblue", color: Color { r: 135, g: 206, b: 250, a: 1.0 } },
	NamedColor { name: "lightslategray", color: Color { r: 119, g: 136, b: 153, a: 1.0 } },
	NamedColor { name: "lightslategrey", color: Color { r: 119, g: 136, b: 153, a: 1.0 } },
	NamedColor { name: "lightsteelblue", color: Color { r: 176, g: 196, b: 222, a: 1.0 } },
	NamedColor { name: "lightyellow", color: Color { r: 255, g: 255, b: 224, a: 1.0 } },
	NamedColor { name: "lime", color: Color { r: 0, g: 255, b: 0, a: 1.0 } },
	NamedColor { name: "limegreen", color: Color { r: 50, g: 205, b: 50, a: 1.0 } },
	NamedColor { name: "linen", color: Color { r: 250, g: 240, b: 230, a: 1.0 } },
	NamedColor { name: "magenta", color: Color { r: 255, g: 0, b: 255, a: 1.0 } },
	NamedColor { name: "maroon", color: Color { r: 128, g: 0, b: 0, a: 1.0 } },
	NamedColor { name: "mediumaquamarine", color: Color { r: 102, g: 205, b: 170, a: 1.0 } },
	NamedColor { name: "mediumblue", color: Color { r: 0, g: 0, b: 205, a: 1.0 } },
	NamedColor { name: "mediumorchid", color: Color { r: 186, g: 85, b: 211, a: 1.0 } },
	NamedColor { name: "mediumpurple", color: Color { r: 147, g: 112, b: 219, a: 1.0 } },
	NamedColor { name: "mediumseagreen", color: Color { r: 60, g: 179, b: 113, a: 1.0 } },
	NamedColor { name: "mediumslateblue", color: Color { r: 123, g: 104, b: 238, a: 1.0 } },
	NamedColor { name: "mediumspringgreen", color: Color { r: 0, g: 250, b: 154, a: 1.0 } },
	NamedColor { name: "mediumturquoise", color: Color { r: 72, g: 209, b: 204, a: 1.0 } },
	NamedColor { name: "mediumvioletred", color: Color { r: 199, g: 21, b: 133, a: 1.0 } },
	NamedColor { name: "midnightblue", color: Color { r: 25, g: 25, b: 112, a: 1.0 } },
	NamedColor { name: "mintcream", color: Color { r: 245, g: 255, b: 250, a: 1.0 } },
	NamedColor { name: "mistyrose", color: Color { r: 255, g: 228, b: 225, a: 1.0 } },
	NamedColor { name: "moccasin", color: Color { r: 255, g: 228, b: 181, a: 1.0 } },
	NamedColor { name: "navajowhite", color: Color { r: 255, g: 222, b: 173, a: 1.0 } },
	NamedColor { name: "navy", color: Color { r: 0, g: 0, b: 128, a: 1.0 } },
	NamedColor { name: "oldlace", color: Color { r: 253, g: 245, b: 230, a: 1.0 } },
	NamedColor { name: "olive", color: Color { r: 128, g: 128, b: 0, a: 1.0 } },
	NamedColor { name: "olivedrab", color: Color { r: 107, g: 142, b: 35, a: 1.0 } },
	NamedColor { name: "orange", color: Color { r: 255, g: 165, b: 0, a: 1.0 } },
	NamedColor { name: "orangered", color: Color { r: 255, g: 69, b: 0, a: 1.0 } },
	NamedColor { name: "orchid", color: Color { r: 218, g: 112, b: 214, a: 1.0 } },
	NamedColor { name: "palegoldenrod", color: Color { r: 238, g: 232, b: 170, a: 1.0 } },
	NamedColor { name: "palegreen", color: Color { r: 152, g: 251, b: 152, a: 1.0 } },
	NamedColor { name: "paleturquoise", color: Color { r: 175, g: 238, b: 238, a: 1.0 } },
	NamedColor { name: "palevioletred", color: Color { r: 219, g: 112, b: 147, a: 1.0 } },
	NamedColor { name: "papayawhip", color: Color { r: 255, g: 239, b: 213, a: 1.0 } },
	NamedColor { name: "peachpuff", color: Color { r: 255, g: 218, b: 185, a: 1.0 } },
	NamedColor { name: "peru", color: Color { r: 205, g: 133, b: 63, a: 1.0 } },
	NamedColor { name: "pink", color: Color { r: 255, g: 192, b: 203, a: 1.0 } },
	NamedColor { name: "plum", color: Color { r: 221, g: 160, b: 221, a: 1.0 } },
	NamedColor { name: "powderblue", color: Color { r: 176, g: 224, b: 230, a: 1.0 } },
	NamedColor { name: "purple", color: Color { r: 128, g: 0, b: 128, a: 1.0 } },
	NamedColor { name: "red", color: Color { r: 255, g: 0, b: 0, a: 1.0 } },
	NamedColor { name: "rosybrown", color: Color { r: 188, g: 143, b: 143, a: 1.0 } },
	NamedColor { name: "royalblue", color: Color { r: 65, g: 105, b: 225, a: 1.0 } },
	NamedColor { name: "saddlebrown", color: Color { r: 139, g: 69, b: 19, a: 1.0 } },
	NamedColor { name: "salmon", color: Color { r: 250, g: 128, b: 114, a: 1.0 } },
	NamedColor { name: "sandybrown", color: Color { r: 244, g: 164, b: 96, a: 1.0 } },
	NamedColor { name: "seagreen", color: Color { r: 46, g: 139, b: 87, a: 1.0 } },
	NamedColor { name: "seashell", color: Color { r: 255, g: 245, b: 238, a: 1.0 } },
	NamedColor { name: "sienna", color: Color { r: 160, g: 82, b: 45, a: 1.0 } },
	NamedColor { name: "silver", color: Color { r: 192, g: 192, b: 192, a: 1.0 } },
	NamedColor { name: "skyblue", color: Color { r: 135, g: 206, b: 235, a: 1.0 } },
	NamedColor { name: "slateblue", color: Color { r: 106, g: 90, b: 205, a: 1.0 } },
	NamedColor { name: "slategray", color: Color { r: 112, g: 128, b: 144, a: 1.0 } },
	NamedColor { name: "slategrey", color: Color { r: 112, g: 128, b: 144, a: 1.0 } },
	NamedColor { name: "snow", color: Color { r: 255, g: 250, b: 250, a: 1.0 } },
	NamedColor { name: "springgreen", color: Color { r: 0, g: 255, b: 127, a: 1.0 } },
	NamedColor { name: "steelblue", color: Color { r: 70, g: 130, b: 180, a: 1.0 } },
	NamedColor { name: "tan", color: Color { r: 210, g: 180, b: 140, a: 1.0 } },
	NamedColor { name: "teal", color: Color { r: 0, g: 128, b: 128, a: 1.0 } },
	NamedColor { name: "thistle", color: Color { r: 216, g: 191, b: 216, a: 1.0 } },
	NamedColor { name: "tomato", color: Color { r: 255, g: 99, b: 71, a: 1.0 } },
	NamedColor { name: "turquoise", color: Color { r: 64, g: 224, b: 208, a: 1.0 } },
	NamedColor { name: "violet", color: Color { r: 238, g: 130, b: 238, a: 1.0 } },
	NamedColor { name: "wheat", color: Color { r: 245, g: 222, b: 179, a: 1.0 } },
	NamedColor { name: "white", color: Color { r: 255, g: 255, b: 255, a: 1.0 } },
	NamedColor { name: "whitesmoke", color: Color { r: 245, g: 245, b: 245, a: 1.0 } },
	NamedColor { name: "yellow", color: Color { r: 255, g: 255, b: 0, a: 1.0 } },
	NamedColor { name: "yellowgreen", color: Color { r: 154, g: 205, b: 50, a: 1.0 } },
];