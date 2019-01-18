#![feature(proc_macro_hygiene, decl_macro)]
extern crate handlebars;

use modules::venn_genn;
use std::error::Error;

// fn main() -> Result<(), Box<Error>> {
//     println!(
//         "{}",
//         venn_genn::VennDiagram {
//             first_title: "first".to_string(),
//             second_title: "second".to_string(),
//             central_title: "centre".to_string(),
//             third_title: "third".to_string(),
//             first_third_title: "1-3".to_string(),
//             second_third_title: "2-3".to_string(),
//             first_second_title: "1-2".to_string(),
//             height: 800.0,
//             width: 800.0,
//             center: 400.0,
//             overlap: 40.0,
//             radius: 160.0,
//         }
//         .to_svg()?
//     );
//     return Ok(());
// }

#[macro_use]
extern crate rocket;
#[get("/2venn.svg?<first>&<second>&<one_two>&<radius>&<size>&<overlap>")]
fn venn_2(
    first: String,
    second: String,
    one_two: String,
    radius: Option<f64>,
    size: Option<f64>,
    overlap: Option<f64>,
) -> venn_genn::VennDiagram {
    return venn_genn::VennDiagram {
        first_title: first,
        second_title: second,
        first_second_title: one_two,
        central_title: "".to_string(),
        third_title: "".to_string(),
        first_third_title: "".to_string(),
        second_third_title: "".to_string(),
        height: size.unwrap_or_else(|| 800.0),
        width: size.unwrap_or_else(|| 800.0),
        center: size.unwrap_or_else(|| 800.0) / 2_f64,
        overlap: overlap.unwrap_or_else(|| 40.0),
        radius: radius.unwrap_or_else(|| 160.0),
    };
}

#[get("/venn.svg?<first>&<second>&<third>&<one_two>&<one_three>&<two_three>&<middle>&<radius>&<size>&<overlap>")]
fn venn(
    first: String,
    second: String,
    third: Option<String>,
    one_two: Option<String>,
    one_three: Option<String>,
    two_three: Option<String>,
    middle: Option<String>,
    radius: Option<f64>,
    size: Option<f64>,
    overlap: Option<f64>,
) -> venn_genn::VennDiagram {
    return venn_genn::VennDiagram {
        first_title: first,
        second_title: second,
        first_second_title: one_two.unwrap_or_else(|| "".to_string()),
        central_title: middle.unwrap_or_else(|| "".to_string()),
        third_title: third.unwrap_or_else(|| "".to_string()),
        first_third_title: one_three.unwrap_or_else(|| "".to_string()),
        second_third_title: two_three.unwrap_or_else(|| "".to_string()),
        height: size.unwrap_or_else(|| 800.0),
        width: size.unwrap_or_else(|| 800.0),
        center: size.unwrap_or_else(|| 800.0) / 2_f64,
        overlap: overlap.unwrap_or_else(|| 40.0),
        radius: radius.unwrap_or_else(|| 160.0),
    };
}
use rocket::response::content;
#[get("/")]
fn index() -> content::Html<&'static str> {
  return content::Html(r###"<?xml
  <!DOCTYPE html><meta charset=utf-8>
  <html><head><title>Venn Genn(erator)</title></head><body>
  <style> label { display: block; max-width: 300px; margin: auto; height: 2em; } input { float: right;} </style>
  <form method='GET' action="/venn.svg">
  <label>Left circle: <input required name="first" value="ducks"/></label>
  <label>Right circle: <input required name="second" value="moles"/></label>
  <label>Union of left & right: <input name="one_two" value="platypuses"/></label>
  <label>Top circle: <input name="third"/></label>
  <label>Union of top & left: <input name="one_three"/></label>
  <label>Union of right & top: <input name="two_three"/></label>
  <label>Union of 3 circles: <input name="middle"/></label>
  <label>Circle radius: <input type="number" value="160" name="radius"/></label>
  <label>Image size: <input type="number" value="800" name="size"/></label>
  <label>Overlap size: <input type="number" value="40" name="overlap"/></label>
  <input type="hidden" value="foo.gif" name="force_unroll"/>
  <label><input type="submit" value="Show me!"/></label>
  </form>
  "###);
}
fn main() {
    rocket::ignite().mount("/", routes![venn_2, venn, index]).launch();
}
