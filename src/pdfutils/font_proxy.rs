use printpdf::BuiltinFont;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum FontProxy {
    // first bool is Bold, second bool is Italics
    Helvetica(bool, bool),
    Times(bool, bool),
}

impl FontProxy {
    pub fn times() -> FontProxy {
        FontProxy::Times(false, false)
    }
    pub fn helvetica() -> FontProxy {
        FontProxy::Helvetica(false, false)
    }
    pub fn times_bold() -> FontProxy {
        FontProxy::times().bold(true)
    }
    pub fn helvetica_bold() -> FontProxy {
        FontProxy::helvetica().bold(true)
    }
    pub fn bold(&self, bold: bool) -> FontProxy {
        match self {
            FontProxy::Helvetica(_, it) => FontProxy::Helvetica(bold, *it),
            FontProxy::Times(_, it) => FontProxy::Times(bold, *it),
        }
    }
}

impl Default for FontProxy {
    fn default() -> Self {
        FontProxy::Times(false, false)
    }
}

impl From<FontProxy> for BuiltinFont {
    fn from(font_proxy: FontProxy) -> Self {
        match font_proxy {
            FontProxy::Helvetica(bold, italic) => {
                if bold && italic {
                    BuiltinFont::HelveticaBoldOblique
                } else if bold {
                    BuiltinFont::HelveticaBold
                } else if italic {
                    BuiltinFont::HelveticaOblique
                } else {
                    BuiltinFont::Helvetica
                }
            }
            FontProxy::Times(bold, italic) => {
                if bold && italic {
                    BuiltinFont::TimesBoldItalic
                } else if bold {
                    BuiltinFont::TimesBold
                } else if italic {
                    BuiltinFont::TimesItalic
                } else {
                    BuiltinFont::TimesRoman
                }
            }
        }
    }
}
