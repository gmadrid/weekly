use crate::pdfutils::font_proxy::FontProxy;
use crate::pdfutils::Instruction;
use crate::Instructions;
use printpdf::{IndirectFontRef, PdfDocumentReference};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct FontMap(HashMap<FontProxy, IndirectFontRef>);

impl FontMap {
    pub fn resolve_fonts(
        mut self,
        doc: &PdfDocumentReference,
        instructions: &Instructions,
    ) -> crate::Result<FontMap> {
        // Look for all of the fonts referenced in the Instructions,
        // add them to the PdfDocument, adding the fonts to map.
        instructions
            .instructions
            .iter()
            .filter_map(|i| match i {
                Instruction::Text(tv) => Some(tv.font),
                _ => None,
            })
            .try_for_each::<_, crate::Result<()>>(|font| {
                let entry = self.0.entry(font);

                // Basically doing or_insert_with(), but I need to propagate an error.
                if let std::collections::hash_map::Entry::Vacant(ve) = entry {
                    let indirect_font = doc.add_builtin_font(font.into())?;
                    ve.insert(indirect_font);
                }
                Ok(())
            })?;
        Ok(self)
    }

    pub fn lookup(&self, font_proxy: FontProxy) -> &IndirectFontRef {
        // unwrap: can we get rid of this?
        self.0.get(&font_proxy).unwrap()
    }
}
