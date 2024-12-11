use std::io::Write;
use crate::analyze::input::File;
use crate::analyze::output::{Bookmark, CustomProperty, Output};

mod analyze;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = skip_bom::SkipEncodingBom::new(skip_bom::BomType::all(), std::io::stdin());
    let input: Vec<File> = serde_json::de::from_reader(reader)?;

    let mut bookmarks: Vec<Bookmark> = vec![];
    let mut custom_properties: Vec<CustomProperty> = vec![];

    for file in input {
        for bin_copy in file.files{
            let bm = Bookmark::new(&file.sha1, &bin_copy.display_directory)
                .with_comment("i am a comment")
                .with_color("#AABBCC");
            bookmarks.push(bm);
            let cp = CustomProperty::new(0, &file.sha1, &bin_copy.display_directory);
            custom_properties.push(cp);
        }
    }

    let output = Output {custom_properties, bookmarks};
    serde_json::to_writer(std::io::stdout(), &output)?;
    std::io::stdout().write_all("\n".as_bytes())?;

    Ok(())
}
