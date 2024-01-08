/// Exposes [palette](https://crates.io/crates/palette.0 * D)'s color management tools and brings the Rgb struct forward so as to be easier to import/qualify in Sled projects.
pub use palette::rgb::Rgb;
pub use palette::*;

#[cfg(feature = "named_colors")]
pub mod consts {
    //! A collection of named color constants. Can be toggled with the `named_colors` Cargo feature.
    //!
    //! Adapted from palette's [named](https://docs.rs/palette/0.7.3/palette/named/index.html) module
    //! but expressed as 32-bit rgb instead of 8-bit for better compatability with sled.
    //! 
    //! Colors are taken from the [SVG keyword
    //! colors](https://www.w3.org/TR/SVG11/types.html#ColorKeywords) (same as in
    //! CSS3) and they can be used as if they were pixel values:
    //!
    //! ```rust, ignore
    //! use sled::color::consts;
    //! // -snip-
    //! sled.set_all(consts::BLACK);
    //! ```

    use super::Rgb;
    const D: f32 = 1.0 / 255.0;
    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: aliceblue;"></div>
    pub const ALICEBLUE: Rgb = Rgb::new(240.0 * D, 248.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: antiquewhite;"></div>
    pub const ANTIQUEWHITE: Rgb = Rgb::new(250.0 * D, 235.0 * D, 215.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: aqua;"></div>
    pub const AQUA: Rgb = Rgb::new(0.0 * D, 255.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: aquamarine;"></div>
    pub const AQUAMARINE: Rgb = Rgb::new(127.0 * D, 255.0 * D, 212.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: azure;"></div>
    pub const AZURE: Rgb = Rgb::new(240.0 * D, 255.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: beige;"></div>
    pub const BEIGE: Rgb = Rgb::new(245.0 * D, 245.0 * D, 220.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: bisque;"></div>
    pub const BISQUE: Rgb = Rgb::new(255.0 * D, 228.0 * D, 196.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: black;"></div>
    pub const BLACK: Rgb = Rgb::new(0.0 * D, 0.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: blanchedalmond;"></div>
    pub const BLANCHEDALMOND: Rgb = Rgb::new(255.0 * D, 235.0 * D, 205.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: blue;"></div>
    pub const BLUE: Rgb = Rgb::new(0.0 * D, 0.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: blueviolet;"></div>
    pub const BLUEVIOLET: Rgb = Rgb::new(138.0 * D, 43.0 * D, 226.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: brown;"></div>
    pub const BROWN: Rgb = Rgb::new(165.0 * D, 42.0 * D, 42.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: burlywood;"></div>
    pub const BURLYWOOD: Rgb = Rgb::new(222.0 * D, 184.0 * D, 135.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: cadetblue;"></div>
    pub const CADETBLUE: Rgb = Rgb::new(95.0 * D, 158.0 * D, 160.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: chartreuse;"></div>
    pub const CHARTREUSE: Rgb = Rgb::new(127.0 * D, 255.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: chocolate;"></div>
    pub const CHOCOLATE: Rgb = Rgb::new(210.0 * D, 105.0 * D, 30.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: coral;"></div>
    pub const CORAL: Rgb = Rgb::new(255.0 * D, 127.0 * D, 80.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: cornflowerblue;"></div>
    pub const CORNFLOWERBLUE: Rgb = Rgb::new(100.0 * D, 149.0 * D, 237.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: cornsilk;"></div>
    pub const CORNSILK: Rgb = Rgb::new(255.0 * D, 248.0 * D, 220.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: crimson;"></div>
    pub const CRIMSON: Rgb = Rgb::new(220.0 * D, 20.0 * D, 60.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: cyan;"></div>
    pub const CYAN: Rgb = Rgb::new(0.0 * D, 255.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkblue;"></div>
    pub const DARKBLUE: Rgb = Rgb::new(0.0 * D, 0.0 * D, 139.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkcyan;"></div>
    pub const DARKCYAN: Rgb = Rgb::new(0.0 * D, 139.0 * D, 139.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkgoldenrod;"></div>
    pub const DARKGOLDENROD: Rgb = Rgb::new(184.0 * D, 134.0 * D, 11.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkgray;"></div>
    pub const DARKGRAY: Rgb = Rgb::new(169.0 * D, 169.0 * D, 169.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkgreen;"></div>
    pub const DARKGREEN: Rgb = Rgb::new(0.0 * D, 100.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkgrey;"></div>
    pub const DARKGREY: Rgb = Rgb::new(169.0 * D, 169.0 * D, 169.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkkhaki;"></div>
    pub const DARKKHAKI: Rgb = Rgb::new(189.0 * D, 183.0 * D, 107.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkmagenta;"></div>
    pub const DARKMAGENTA: Rgb = Rgb::new(139.0 * D, 0.0 * D, 139.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkolivegreen;"></div>
    pub const DARKOLIVEGREEN: Rgb = Rgb::new(85.0 * D, 107.0 * D, 47.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkorange;"></div>
    pub const DARKORANGE: Rgb = Rgb::new(255.0 * D, 140.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkorchid;"></div>
    pub const DARKORCHID: Rgb = Rgb::new(153.0 * D, 50.0 * D, 204.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkred;"></div>
    pub const DARKRED: Rgb = Rgb::new(139.0 * D, 0.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darksalmon;"></div>
    pub const DARKSALMON: Rgb = Rgb::new(233.0 * D, 150.0 * D, 122.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkseagreen;"></div>
    pub const DARKSEAGREEN: Rgb = Rgb::new(143.0 * D, 188.0 * D, 143.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkslateblue;"></div>
    pub const DARKSLATEBLUE: Rgb = Rgb::new(72.0 * D, 61.0 * D, 139.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkslategray;"></div>
    pub const DARKSLATEGRAY: Rgb = Rgb::new(47.0 * D, 79.0 * D, 79.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkslategrey;"></div>
    pub const DARKSLATEGREY: Rgb = Rgb::new(47.0 * D, 79.0 * D, 79.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkturquoise;"></div>
    pub const DARKTURQUOISE: Rgb = Rgb::new(0.0 * D, 206.0 * D, 209.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: darkviolet;"></div>
    pub const DARKVIOLET: Rgb = Rgb::new(148.0 * D, 0.0 * D, 211.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: deeppink;"></div>
    pub const DEEPPINK: Rgb = Rgb::new(255.0 * D, 20.0 * D, 147.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: deepskyblue;"></div>
    pub const DEEPSKYBLUE: Rgb = Rgb::new(0.0 * D, 191.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: dimgray;"></div>
    pub const DIMGRAY: Rgb = Rgb::new(105.0 * D, 105.0 * D, 105.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: dimgrey;"></div>
    pub const DIMGREY: Rgb = Rgb::new(105.0 * D, 105.0 * D, 105.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: dodgerblue;"></div>
    pub const DODGERBLUE: Rgb = Rgb::new(30.0 * D, 144.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: firebrick;"></div>
    pub const FIREBRICK: Rgb = Rgb::new(178.0 * D, 34.0 * D, 34.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: floralwhite;"></div>
    pub const FLORALWHITE: Rgb = Rgb::new(255.0 * D, 250.0 * D, 240.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: forestgreen;"></div>
    pub const FORESTGREEN: Rgb = Rgb::new(34.0 * D, 139.0 * D, 34.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: fuchsia;"></div>
    pub const FUCHSIA: Rgb = Rgb::new(255.0 * D, 0.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: gainsboro;"></div>
    pub const GAINSBORO: Rgb = Rgb::new(220.0 * D, 220.0 * D, 220.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: ghostwhite;"></div>
    pub const GHOSTWHITE: Rgb = Rgb::new(248.0 * D, 248.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: gold;"></div>
    pub const GOLD: Rgb = Rgb::new(255.0 * D, 215.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: goldenrod;"></div>
    pub const GOLDENROD: Rgb = Rgb::new(218.0 * D, 165.0 * D, 32.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: gray;"></div>
    pub const GRAY: Rgb = Rgb::new(128.0 * D, 128.0 * D, 128.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: grey;"></div>
    pub const GREY: Rgb = Rgb::new(128.0 * D, 128.0 * D, 128.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: green;"></div>
    pub const GREEN: Rgb = Rgb::new(0.0 * D, 128.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: greenyellow;"></div>
    pub const GREENYELLOW: Rgb = Rgb::new(173.0 * D, 255.0 * D, 47.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: honeydew;"></div>
    pub const HONEYDEW: Rgb = Rgb::new(240.0 * D, 255.0 * D, 240.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: hotpink;"></div>
    pub const HOTPINK: Rgb = Rgb::new(255.0 * D, 105.0 * D, 180.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: indianred;"></div>
    pub const INDIANRED: Rgb = Rgb::new(205.0 * D, 92.0 * D, 92.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: indigo;"></div>
    pub const INDIGO: Rgb = Rgb::new(75.0 * D, 0.0 * D, 130.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: ivory;"></div>
    pub const IVORY: Rgb = Rgb::new(255.0 * D, 255.0 * D, 240.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: khaki;"></div>
    pub const KHAKI: Rgb = Rgb::new(240.0 * D, 230.0 * D, 140.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lavender;"></div>
    pub const LAVENDER: Rgb = Rgb::new(230.0 * D, 230.0 * D, 250.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lavenderblush;"></div>
    pub const LAVENDERBLUSH: Rgb = Rgb::new(255.0 * D, 240.0 * D, 245.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lawngreen;"></div>
    pub const LAWNGREEN: Rgb = Rgb::new(124.0 * D, 252.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lemonchiffon;"></div>
    pub const LEMONCHIFFON: Rgb = Rgb::new(255.0 * D, 250.0 * D, 205.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightblue;"></div>
    pub const LIGHTBLUE: Rgb = Rgb::new(173.0 * D, 216.0 * D, 230.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightcoral;"></div>
    pub const LIGHTCORAL: Rgb = Rgb::new(240.0 * D, 128.0 * D, 128.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightcyan;"></div>
    pub const LIGHTCYAN: Rgb = Rgb::new(224.0 * D, 255.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightgoldenrodyellow;"></div>
    pub const LIGHTGOLDENRODYELLOW: Rgb = Rgb::new(250.0 * D, 250.0 * D, 210.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightgray;"></div>
    pub const LIGHTGRAY: Rgb = Rgb::new(211.0 * D, 211.0 * D, 211.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightgreen;"></div>
    pub const LIGHTGREEN: Rgb = Rgb::new(144.0 * D, 238.0 * D, 144.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightgrey;"></div>
    pub const LIGHTGREY: Rgb = Rgb::new(211.0 * D, 211.0 * D, 211.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightpink;"></div>
    pub const LIGHTPINK: Rgb = Rgb::new(255.0 * D, 182.0 * D, 193.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightsalmon;"></div>
    pub const LIGHTSALMON: Rgb = Rgb::new(255.0 * D, 160.0 * D, 122.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightseagreen;"></div>
    pub const LIGHTSEAGREEN: Rgb = Rgb::new(32.0 * D, 178.0 * D, 170.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightskyblue;"></div>
    pub const LIGHTSKYBLUE: Rgb = Rgb::new(135.0 * D, 206.0 * D, 250.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightslategray;"></div>
    pub const LIGHTSLATEGRAY: Rgb = Rgb::new(119.0 * D, 136.0 * D, 153.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightslategrey;"></div>
    pub const LIGHTSLATEGREY: Rgb = Rgb::new(119.0 * D, 136.0 * D, 153.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightsteelblue;"></div>
    pub const LIGHTSTEELBLUE: Rgb = Rgb::new(176.0 * D, 196.0 * D, 222.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lightyellow;"></div>
    pub const LIGHTYELLOW: Rgb = Rgb::new(255.0 * D, 255.0 * D, 224.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: lime;"></div>
    pub const LIME: Rgb = Rgb::new(0.0 * D, 255.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: limegreen;"></div>
    pub const LIMEGREEN: Rgb = Rgb::new(50.0 * D, 205.0 * D, 50.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: linen;"></div>
    pub const LINEN: Rgb = Rgb::new(250.0 * D, 240.0 * D, 230.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: magenta;"></div>
    pub const MAGENTA: Rgb = Rgb::new(255.0 * D, 0.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: maroon;"></div>
    pub const MAROON: Rgb = Rgb::new(128.0 * D, 0.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mediumaquamarine;"></div>
    pub const MEDIUMAQUAMARINE: Rgb = Rgb::new(102.0 * D, 205.0 * D, 170.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mediumblue;"></div>
    pub const MEDIUMBLUE: Rgb = Rgb::new(0.0 * D, 0.0 * D, 205.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mediumorchid;"></div>
    pub const MEDIUMORCHID: Rgb = Rgb::new(186.0 * D, 85.0 * D, 211.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mediumpurple;"></div>
    pub const MEDIUMPURPLE: Rgb = Rgb::new(147.0 * D, 112.0 * D, 219.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mediumseagreen;"></div>
    pub const MEDIUMSEAGREEN: Rgb = Rgb::new(60.0 * D, 179.0 * D, 113.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mediumslateblue;"></div>
    pub const MEDIUMSLATEBLUE: Rgb = Rgb::new(123.0 * D, 104.0 * D, 238.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mediumspringgreen;"></div>
    pub const MEDIUMSPRINGGREEN: Rgb = Rgb::new(0.0 * D, 250.0 * D, 154.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mediumturquoise;"></div>
    pub const MEDIUMTURQUOISE: Rgb = Rgb::new(72.0 * D, 209.0 * D, 204.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mediumvioletred;"></div>
    pub const MEDIUMVIOLETRED: Rgb = Rgb::new(199.0 * D, 21.0 * D, 133.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: midnightblue;"></div>
    pub const MIDNIGHTBLUE: Rgb = Rgb::new(25.0 * D, 25.0 * D, 112.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mintcream;"></div>
    pub const MINTCREAM: Rgb = Rgb::new(245.0 * D, 255.0 * D, 250.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: mistyrose;"></div>
    pub const MISTYROSE: Rgb = Rgb::new(255.0 * D, 228.0 * D, 225.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: moccasin;"></div>
    pub const MOCCASIN: Rgb = Rgb::new(255.0 * D, 228.0 * D, 181.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: navajowhite;"></div>
    pub const NAVAJOWHITE: Rgb = Rgb::new(255.0 * D, 222.0 * D, 173.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: navy;"></div>
    pub const NAVY: Rgb = Rgb::new(0.0 * D, 0.0 * D, 128.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: oldlace;"></div>
    pub const OLDLACE: Rgb = Rgb::new(253.0 * D, 245.0 * D, 230.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: olive;"></div>
    pub const OLIVE: Rgb = Rgb::new(128.0 * D, 128.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: olivedrab;"></div>
    pub const OLIVEDRAB: Rgb = Rgb::new(107.0 * D, 142.0 * D, 35.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: orange;"></div>
    pub const ORANGE: Rgb = Rgb::new(255.0 * D, 165.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: orangered;"></div>
    pub const ORANGERED: Rgb = Rgb::new(255.0 * D, 69.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: orchid;"></div>
    pub const ORCHID: Rgb = Rgb::new(218.0 * D, 112.0 * D, 214.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: palegoldenrod;"></div>
    pub const PALEGOLDENROD: Rgb = Rgb::new(238.0 * D, 232.0 * D, 170.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: palegreen;"></div>
    pub const PALEGREEN: Rgb = Rgb::new(152.0 * D, 251.0 * D, 152.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: paleturquoise;"></div>
    pub const PALETURQUOISE: Rgb = Rgb::new(175.0 * D, 238.0 * D, 238.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: palevioletred;"></div>
    pub const PALEVIOLETRED: Rgb = Rgb::new(219.0 * D, 112.0 * D, 147.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: papayawhip;"></div>
    pub const PAPAYAWHIP: Rgb = Rgb::new(255.0 * D, 239.0 * D, 213.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: peachpuff;"></div>
    pub const PEACHPUFF: Rgb = Rgb::new(255.0 * D, 218.0 * D, 185.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: peru;"></div>
    pub const PERU: Rgb = Rgb::new(205.0 * D, 133.0 * D, 63.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: pink;"></div>
    pub const PINK: Rgb = Rgb::new(255.0 * D, 192.0 * D, 203.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: plum;"></div>
    pub const PLUM: Rgb = Rgb::new(221.0 * D, 160.0 * D, 221.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: powderblue;"></div>
    pub const POWDERBLUE: Rgb = Rgb::new(176.0 * D, 224.0 * D, 230.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: purple;"></div>
    pub const PURPLE: Rgb = Rgb::new(128.0 * D, 0.0 * D, 128.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: rebeccapurple;"></div>
    pub const REBECCAPURPLE: Rgb = Rgb::new(102.0 * D, 51.0 * D, 153.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: red;"></div>
    pub const RED: Rgb = Rgb::new(255.0 * D, 0.0 * D, 0.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: rosybrown;"></div>
    pub const ROSYBROWN: Rgb = Rgb::new(188.0 * D, 143.0 * D, 143.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: royalblue;"></div>
    pub const ROYALBLUE: Rgb = Rgb::new(65.0 * D, 105.0 * D, 225.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: saddlebrown;"></div>
    pub const SADDLEBROWN: Rgb = Rgb::new(139.0 * D, 69.0 * D, 19.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: salmon;"></div>
    pub const SALMON: Rgb = Rgb::new(250.0 * D, 128.0 * D, 114.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: sandybrown;"></div>
    pub const SANDYBROWN: Rgb = Rgb::new(244.0 * D, 164.0 * D, 96.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: seagreen;"></div>
    pub const SEAGREEN: Rgb = Rgb::new(46.0 * D, 139.0 * D, 87.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: seashell;"></div>
    pub const SEASHELL: Rgb = Rgb::new(255.0 * D, 245.0 * D, 238.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: sienna;"></div>
    pub const SIENNA: Rgb = Rgb::new(160.0 * D, 82.0 * D, 45.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: silver;"></div>
    pub const SILVER: Rgb = Rgb::new(192.0 * D, 192.0 * D, 192.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: skyblue;"></div>
    pub const SKYBLUE: Rgb = Rgb::new(135.0 * D, 206.0 * D, 235.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: slateblue;"></div>
    pub const SLATEBLUE: Rgb = Rgb::new(106.0 * D, 90.0 * D, 205.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: slategray;"></div>
    pub const SLATEGRAY: Rgb = Rgb::new(112.0 * D, 128.0 * D, 144.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: slategrey;"></div>
    pub const SLATEGREY: Rgb = Rgb::new(112.0 * D, 128.0 * D, 144.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: snow;"></div>
    pub const SNOW: Rgb = Rgb::new(255.0 * D, 250.0 * D, 250.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: springgreen;"></div>
    pub const SPRINGGREEN: Rgb = Rgb::new(0.0 * D, 255.0 * D, 127.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: steelblue;"></div>
    pub const STEELBLUE: Rgb = Rgb::new(70.0 * D, 130.0 * D, 180.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: tan;"></div>
    pub const TAN: Rgb = Rgb::new(210.0 * D, 180.0 * D, 140.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: teal;"></div>
    pub const TEAL: Rgb = Rgb::new(0.0 * D, 128.0 * D, 128.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: thistle;"></div>
    pub const THISTLE: Rgb = Rgb::new(216.0 * D, 191.0 * D, 216.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: tomato;"></div>
    pub const TOMATO: Rgb = Rgb::new(255.0 * D, 99.0 * D, 71.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: turquoise;"></div>
    pub const TURQUOISE: Rgb = Rgb::new(64.0 * D, 224.0 * D, 208.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: violet;"></div>
    pub const VIOLET: Rgb = Rgb::new(238.0 * D, 130.0 * D, 238.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: wheat;"></div>
    pub const WHEAT: Rgb = Rgb::new(245.0 * D, 222.0 * D, 179.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: white;"></div>
    pub const WHITE: Rgb = Rgb::new(255.0 * D, 255.0 * D, 255.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: whitesmoke;"></div>
    pub const WHITESMOKE: Rgb = Rgb::new(245.0 * D, 245.0 * D, 245.0 * D);

    ///<div style="display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: yellow;"></div>
    pub const YELLOW: Rgb = Rgb::new(255.0 * D, 255.0 * D, 0.0 * D);
}
