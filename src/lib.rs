#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;


pub mod venn_genn {
    use std::error::Error;
    use std::vec::Vec;
    use handlebars::Handlebars;

    // const THIRD_DEGREE: f64 = 0.86602540378;
    const SVG_TEMPLATE: &'static str = r###"<?xml
    version="1.0"
    encoding="UTF-8"
    standalone="yes"
  ?>
  <!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.0//EN" "http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd">

  <svg
    height="{{height}}"
    width="{{width}}"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:svg="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink"
  >
    <title>
      {{title}}
    </title>

    {{#each circles}}
      <circle
        cx="{{centre.x}}"
        cy="{{centre.y}}"
        r="{{r}}"
        style="fill-opacity: 0.5; fill: {{color}}"
      />
    {{/each}}

    {{#each texts}}
      <text x="{{centre.x}}" y="{{centre.y}}" dominant-baseline="middle" text-anchor="middle">{{body}}</text>
    {{/each}}
  </svg>
"###;

    #[derive(Debug, Deserialize, Serialize)]
    struct Circle {
        centre: Point,
        r: f64,
        color: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Text {
        centre: Point,
        body: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Point {
        x: f64,
        y: f64,
    }
    impl Point {
        fn midway_to(self, other: &Point) -> Point {
            return Point {
                x: (self.x + other.x) / 2.0,
                y: (self.y + other.y) / 2.0,
            };
        }
        fn three_way_midpoint(self, other: &Point, third: &Point) -> Point {
            return Point {
                x: (self.x + other.x + third.x) / 3.0,
                y: (self.y + other.y + third.y) / 3.0,
            };
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct VennDiagram {
        pub height: f64,
        pub width: f64,
        pub center: f64,
        pub overlap: f64,
        pub radius: f64,

        pub first_title: String,
        pub second_title: String,
        pub central_title: String,

        pub third_title: String,
        pub first_third_title: String,
        pub second_third_title: String,
        pub first_second_title: String,
    }

    impl VennDiagram {
        pub fn to_svg(self) -> Result<String, Box<Error>> {
            let mut reg = Handlebars::new();
            reg.set_strict_mode(true);
            return Ok(reg.render_template(
                SVG_TEMPLATE,
                &json!({
                  "circles": self.circles(),
                  "texts": self.texts(),
                  "title": "Venn Diagram showing overlap".to_string(),
                  "width": self.width,
                  "height": self.height
                }),
            )?);
        }

        fn centre_one(&self) -> Point {
            return Point {
                x: self.center + self.overlap - self.radius,
                y: self.center,
            };
        }
        fn centre_two(&self) -> Point {
            return Point {
                x: self.center - self.overlap + self.radius,
                y: self.center,
            };
        }
        fn centre_three(&self) -> Point {
            let long_side = 2.0 * (self.radius - self.overlap);
            return Point {
                x: self.center,
                y: self.center - (long_side.powi(2) - (long_side / 2.0).powi(2)).sqrt(),
            };
        }
        fn centre_text(&self) -> Point {
            return self
                .centre_one()
                .three_way_midpoint(&self.centre_two(), &self.centre_three());
        }

        fn circles(&self) -> Vec<Box<Circle>> {
            let first = Box::new(Circle {
                centre: self.centre_one(),
                r: self.radius,
                color: "#FF00D9".to_string(),
            });
            let second = Box::new(Circle {
                centre: self.centre_two(),
                r: self.radius,
                color: "#14CCC0".to_string(),
            });
            let third = Box::new(Circle {
                centre: self.centre_three(),
                r: self.radius,
                color: "#FFD20E".to_string(),
            });

            if self.third_title != "" {
                return vec![first, second, third];
            }
            return vec![first, second];
        }

        fn texts(&self) -> Vec<Box<Text>> {
            let mut texts = vec![
                Box::new(Text {
                    // TODO: if any overlap is present, centre text in non-overlap-region
                    centre: self.centre_one(),
                    body: self.first_title.clone(),
                }),
                Box::new(Text {
                    // TODO: if any overlap is present, centre text in non-overlap-region
                    centre: self.centre_two(),
                    body: self.second_title.clone(),
                }),
            ];

            if self.overlap > 0.0 {
                texts.push(
                    // TODO: centre overlap-text in overlap-region
                    Box::new(Text {
                        centre: self.centre_one().midway_to(&self.centre_two()),
                        body: self.first_second_title.clone(),
                    })
                )
            }
            if self.third_title != "" {
                texts.push(Box::new(Text {
                    centre: self.centre_text(),
                    body: self.central_title.clone(),
                }));
                texts.push(Box::new(Text {
                    // TODO: if any overlap is present, centre text in non-overlap-region
                    centre: self.centre_three(),
                    body: self.third_title.clone(),
                }));

                if self.overlap > 0.0 {
                    // TODO: centre overlap-text in overlap-region
                    texts.push(Box::new(Text {
                        centre: self.centre_one().midway_to(&self.centre_three()),
                        body: self.first_third_title.clone(),
                    }));
                    // TODO: centre overlap-text in overlap-region
                    texts.push(Box::new(Text {
                        centre: self.centre_two().midway_to(&self.centre_three()),
                        body: self.second_third_title.clone(),
                    }));
                }
            }

            return texts;
        }
    }
}
