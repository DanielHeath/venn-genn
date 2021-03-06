#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod venn_genn {
    use handlebars::Handlebars;
    use std::error::Error;
    use std::vec::Vec;

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
        fn frac_between(self, other: &Point, frac: f64) -> Point {
            return Point {
                x: (self.x + other.x * frac) / (frac + 1.0),
                y: (self.y + other.y * frac) / (frac + 1.0),
            };
        }
        fn three_way_midpoint(self, other: &Point, third: &Point) -> Point {
            return Point {
                x: (self.x + other.x + third.x) / 3.0,
                y: (self.y + other.y + third.y) / 3.0,
            };
        }
        fn distance_to(self, other: Point) -> f64 {
            (self.x - other.x).hypot(self.y - other.y)
        }
        fn intersect_circle(self, other: Point, r1: f64, r2: f64) -> (Point, Point) {
            let centerdx = self.x - other.x;
            let centerdy = self.y - other.y;
            let dist = centerdx.hypot(centerdy);
            if !((r1 - r2).abs() <= dist && dist <= r1 + r2) {
                // no intersection
                return (self, other);
            }
            // intersection(s) should exist
            let dist_sq = dist * dist;
            let a = (r1 * r1 - r2 * r2) / (2.0 * dist_sq);
            let r2r2 = r1 * r1 - r2 * r2;
            let c =
                (2.0 * (r1 * r1 + r2 * r2) / dist_sq - (r2r2 * r2r2) / (dist_sq * dist_sq) - 1.0)
                    .sqrt();

            let fx = (self.x + other.x) / 2.0 + a * (other.x - self.x);
            let gx = c * (other.y - self.y) / 2.0;
            let ix1 = fx + gx;
            let ix2 = fx - gx;

            let fy = (self.y + other.y) / 2.0 + a * (other.y - self.y);
            let gy = c * (self.x - other.x) / 2.0;
            let iy1 = fy + gy;
            let iy2 = fy - gy;

            // note if gy == 0 and gx == 0 then the circles are tangent and there is only one solution
            // but that one solution will just be duplicated as the code is currently written
            return (Point { x: ix1, y: iy1 }, Point { x: ix2, y: iy2 });
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

use std::io::Cursor;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};

impl<'r> Responder<'r> for VennDiagram {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(self.to_svg().unwrap_or_else(|_| "".to_string())))
            .raw_header("Content-Type", "image/svg+xml")
            .raw_header("Content-Disposition", "inline;filename=venn-diagram.svg")
            .ok()
    }
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
        fn circle_one_botleft(&self) -> Point {
            let mut centre = self.centre_one();
            centre.x -= self.radius * 30_f64.to_radians().cos();
            centre.y += self.radius * 30_f64.to_radians().sin();
            return centre;
        }
        fn circle_one_topright(&self) -> Point {
            let mut centre = self.centre_one();
            centre.x += self.radius * 30_f64.to_radians().cos();
            centre.y -= self.radius * 30_f64.to_radians().sin();
            return centre;
        }
        fn centre_two(&self) -> Point {
            return Point {
                x: self.center - self.overlap + self.radius,
                y: self.center,
            };
        }
        fn circle_two_botright(&self) -> Point {
            let mut centre = self.centre_two();
            centre.x += self.radius * 30_f64.to_radians().cos();
            centre.y += self.radius * 30_f64.to_radians().sin();
            return centre;
        }
        fn circle_two_topleft(&self) -> Point {
            let mut centre = self.centre_two();
            centre.x -= self.radius * 30_f64.to_radians().cos();
            centre.y -= self.radius * 30_f64.to_radians().sin();
            return centre;
        }
        fn centre_three(&self) -> Point {
            let long_side = 2.0 * (self.radius - self.overlap);
            return Point {
                x: self.center,
                y: self.center - (long_side.powi(2) - (long_side / 2.0).powi(2)).sqrt(),
            };
        }
        fn circle_three_top(&self) -> Point {
            let mut centre = self.centre_three();
            centre.y -= self.radius;
            return centre;
        }
        fn circle_three_bot(&self) -> Point {
            let mut centre = self.centre_three();
            centre.y += self.radius;
            return centre;
        }
        fn centre_text(&self) -> Point {
            return self
                .centre_one()
                .three_way_midpoint(&self.centre_two(), &self.centre_three());
        }
        fn centre_one_text(&self) -> Point {
            if self.overlap <= 0.0 {
                return self.centre_one();
            }

            if self.only_two_circles() {
                let mut c2 = self.centre_one();
                c2.x = c2.x - self.overlap;
                return c2;
            }

            let (p1, p2) =
                self.centre_two()
                    .intersect_circle(self.centre_three(), self.radius, self.radius);
            // use the one with the lowest x.
            let nearpoint = if p1.x < p2.x { p1 } else { p2 };
            return self.circle_one_botleft().midway_to(&nearpoint);
        }

        fn centre_two_text(&self) -> Point {
            if self.overlap <= 0.0 {
                return self.centre_two();
            }

            if self.only_two_circles() {
                let mut c2 = self.centre_two();
                c2.x = c2.x + self.overlap;
                return c2;
            }

            let (p1, p2) =
                self.centre_one()
                    .intersect_circle(self.centre_three(), self.radius, self.radius);
            // use the one with the highest x.
            let nearpoint = if p1.x > p2.x { p1 } else { p2 };
            return self.circle_two_botright().midway_to(&nearpoint);
        }

        fn centre_three_text(&self) -> Point {
            if self.overlap <= 0.0 {
                return self.centre_three();
            }

            let (p1, p2) =
                self.centre_one()
                    .intersect_circle(self.centre_two(), self.radius, self.radius);
            // use the one with the highest y.
            let nearpoint = if p1.y < p2.y { p1 } else { p2 };
            // find the distance to the midpoint
            return self.circle_three_top().midway_to(&nearpoint);
        }
        fn one_two_text(&self) -> Point {
            if self.overlap <= 0.0 || self.only_two_circles() {
                return self.centre_one().midway_to(&self.centre_two());
            }
            let (p12a, p12b) =
                self.centre_two()
                    .intersect_circle(self.centre_one(), self.radius, self.radius);
            let nearpoint = if p12a.y > p12b.y { p12a } else { p12b };
            return self.circle_three_bot().midway_to(&nearpoint);
        }
        fn one_three_text(&self) -> Point {
            if self.overlap <= 0.0 {
                return self.centre_one().midway_to(&self.centre_three());
            }
            let (p1, p2) =
                self.centre_three()
                    .intersect_circle(self.centre_one(), self.radius, self.radius);
            let nearpoint = if p1.y < p2.y { p1 } else { p2 };
            return self.circle_two_topleft().midway_to(&nearpoint);
        }
        fn two_three_text(&self) -> Point {
            if self.overlap <= 0.0 {
                return self.centre_two().midway_to(&self.centre_three());
            }
            let (p1, p2) =
                self.centre_three()
                    .intersect_circle(self.centre_two(), self.radius, self.radius);
            let nearpoint = if p1.y < p2.y { p1 } else { p2 };
            return self.circle_one_topright().midway_to(&nearpoint);
        }

        fn only_two_circles(&self) -> bool {
            self.third_title == ""
        }

        fn circles(&self) -> Vec<Circle> {
            let first = Circle {
                centre: self.centre_one(),
                r: self.radius,
                color: "#FF00D9".to_string(),
            };
            let second = Circle {
                centre: self.centre_two(),
                r: self.radius,
                color: "#14CCC0".to_string(),
            };
            let third = Circle {
                centre: self.centre_three(),
                r: self.radius,
                color: "#FFD20E".to_string(),
            };

            if !self.only_two_circles() {
                return vec![first, second, third];
            }
            return vec![first, second];
        }

        fn texts(&self) -> Vec<Text> {
            let mut texts = vec![
                Text {
                    centre: self.centre_one_text(),
                    body: self.first_title.clone(),
                },
                Text {
                    centre: self.centre_two_text(),
                    body: self.second_title.clone(),
                },
            ];

            if self.overlap > 0.0 {
                texts.push(Text {
                    centre: self.one_two_text(),
                    body: self.first_second_title.clone(),
                })
            }
            if !self.only_two_circles() {
                texts.push(Text {
                    centre: self.centre_text(),
                    body: self.central_title.clone(),
                });
                texts.push(Text {
                    centre: self.centre_three_text(),
                    body: self.third_title.clone(),
                });

                if self.overlap > 0.0 {
                    texts.push(Text {
                        centre: self.one_three_text(),
                        body: self.first_third_title.clone(),
                    });
                    texts.push(Text {
                        centre: self.two_three_text(),
                        body: self.second_third_title.clone(),
                    });
                }
            }

            return texts;
        }
    }
}
