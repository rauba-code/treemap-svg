use std::ops;
use string_builder::Builder;

pub trait Printer {
    fn print(&self) -> String;
}

#[derive(Clone, Copy)]
pub struct Point { pub x : f64, pub y : f64 }

impl Point {
    
    /// Provides (x,0) or (0,y) depending on value 
    pub fn mask(self, value : bool) -> Point {
        if !value { Point { x: self.x, y: 0.0 } }
        else { Point { x: 0.0, y: self.y }} 
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    
    fn sub(self, _rhs: Point) 
    -> <Self as std::ops::Sub<Point>>::Output {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    
    fn add(self, _rhs: Point) 
    -> <Self as std::ops::Add<Point>>::Output {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl ops::Mul<f64> for Point {
    type Output = Point;
    
    fn mul(self, _rhs: f64) 
    -> <Self as std::ops::Mul<f64>>::Output {
        Point {
            x : self.x * _rhs,
            y : self.y * _rhs
        }
    }
}

impl ops::Div<f64> for Point {
    type Output = Point;
    
    fn div(self, _rhs: f64) 
    -> <Self as std::ops::Div<f64>>::Output {
        Point {
            x : self.x / _rhs,
            y : self.y / _rhs
        }
    }
}

pub struct Text { 
    pub a : Point, 
    pub msg : String, 
    pub font_size : f64,
    pub text_length : f64 
}

#[derive(Clone, Copy)]
pub struct Line { pub a : Point, pub off : Point, pub stroke_w : f64 }
#[derive(Clone)]
pub struct Rect { pub a : Point, pub off : Point }
pub struct Svgx {
    pub id : String,
    pub g : Graphic,
    pub width : i32,
    pub height : i32,
    
}
pub struct Graphic {
    pub id : String,
    pub lines : Vec<Line>,
    pub texts : Vec<Text>,
    pub rect : Rect
}

impl Rect {
    /// Creates a ratio-based margin around entire rectangle.
    /// Negative value produces enlargened rectangle.
    /// Ratio value of 0.5 produces a flat object.
    pub fn create_margin(self, ratio : f64) -> Rect {
        Rect {
            a : self.a + (self.off * ratio),
            off : self.off - (self.off * (2.0 * ratio))
        }
    }
}

impl Printer for Line {
    fn print(&self) -> String {
        assert!(self.off.x > 0.0 || self.off.y > 0.0);
        assert!((self.off.x - self.off.y).abs() > f64::EPSILON);
        let direction = 
              if self.off.x == 0.0 { 'v' }
         else if self.off.y == 0.0 { 'h' }
         else { todo!() };
        let length = self.off.x.max(self.off.y);
        format!(r#"    <path style="fill:none;stroke:#000000;stroke-width:{}px;
          stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1"
      d="M {},{} {} {}"/>"#, self.stroke_w, self.a.x, self.a.y, direction, 
      length)
    }
}

impl Printer for Rect {
    fn print(&self) -> String {
        format!(r#"    <rect style="fill:none;stroke:#000000;stroke-width:1px;"
      width="{}"
      height="{}"
      x="{}"
      y="{}"/>"#, self.off.x,
        self.off.y, self.a.x, self.a.y)
    }
}

impl Printer for Graphic {
    fn print(&self) -> String {
        let mut builder = Builder::default();
        builder.append(self.rect.print());
        builder.append('\n');
        for line in &self.lines {
            builder.append(line.print());
            builder.append('\n');
        }
        for text in &self.texts {
            builder.append(text.print());
            builder.append('\n');
        }
        format!(r#"  <g id="{}">
{}  </g>"#, self.id, builder.string().unwrap())
    }
}

impl Printer for Svgx {
    fn print(&self) -> String {
        format!(r#"<svg
    width="{}"
    height="{}"
    viewBox="0 0 {0} {1}"
    version="2"
    id="{}"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:svg="http://www.w3.org/2000/svg">
  <defs id="defs1" />
{}
</svg>"#, self.width, self.height, self.id, self.g.print())
    }
}

impl Printer for Text {
    fn print(&self) -> String {
        format!(r#"    <text x="{}" y="{}" 
    font-size="{}" textLength="{}" 
    lengthAdjust="spacingAndGlyphs">
      {}
    </text>"#, self.a.x, self.a.y + self.font_size, 
        self.font_size, self.text_length, self.msg)
    }
}
