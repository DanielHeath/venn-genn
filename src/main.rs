extern crate handlebars;

use modules::venn_genn;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    println!(
        "{}",
        venn_genn::VennDiagram {
            first_title: "first".to_string(),
            second_title: "second".to_string(),
            central_title: "centre".to_string(),
            third_title: "third".to_string(),
            first_third_title: "1-3".to_string(),
            second_third_title: "2-3".to_string(),
            first_second_title: "1-2".to_string(),
            height: 800.0,
            width: 800.0,
            center: 400.0,
            overlap: 50.0,
            radius: 160.0,
        }
        .to_svg()?
    );
    return Ok(());
}
