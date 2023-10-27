
pub enum Shapes{
    Circle{ x: i32, y: i32, radius: i32 }, 
    Rectangle{ x: i32, y: i32, width: i32, height: i32},
    Triangle{ x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32 }
}

#[derive(Clone)]
struct Circle {
    x: i32,
    y: i32,
    radius: i32,
}

#[derive(Clone)]
struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Clone)]
struct Triangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
}

trait Shape {
    fn rep_string(&self) -> String;
    fn area(&self) -> f32;
    fn clone_box(&self) -> Box<dyn Shape>;
}


impl Circle {
    fn new(x: i32, y: i32, radius: i32) -> Box<Circle> {
        Box::new(Circle { x, y, radius })
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Shape for Circle {
    fn rep_string(&self) -> String {
        format!("<Circle: {}, {}, {}>", self.x, self.y, self.radius)
    }
    fn area(&self) -> f32 {
        std::f32::consts::PI * (self.radius as f32) * (self.radius as f32)
    }
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl Rectangle {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Box<Rectangle> {
        Box::new(Rectangle { x, y, width, height })
    }
}

impl Shape for Rectangle {
    fn rep_string(&self) -> String {
        format!("<Rectangle: {}, {}, {}, {}>", self.x, self.y, self.width, self.height)
    }
    fn area(&self) -> f32 {
        (self.width as f32) * (self.height as f32)
    }
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl Triangle{
    fn new(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> Box<Triangle>{
        Box::new(Triangle { x1, y1, x2, y2, x3, y3 })
    }
}

impl Shape for Triangle{
    fn rep_string(&self) -> String{
        format!("<Triangle: ({}, {}), ({}, {}), ({}, {})>", self.x1, self.y1, self.x2, self.y2, self.x3, self.y3)
    }
    fn area(&self) -> f32{
        0.5 * ((self.x1 - self.x2) * (self.y2 - self.y1) - (self.x1 - self.x2) * (self.y3 - self.y1)) as f32
    }
    fn clone_box(&self) -> Box<dyn Shape>{
        Box::new(self.clone())
    }
}

impl Shapes{
    fn rep_string(&self) -> String{
        match self{
            Shapes::Circle{ x, y, radius } => {
                format!("<Circle: {}, {}, {}>", x, y, radius)
            }
            Shapes::Rectangle{ x, y, width, height }=> {
                format!("<Rectangle: {}, {}, {}, {}>", x, y, width, height)
            }
            Shapes::Triangle{ x1, y1, x2, y2, x3, y3} => {
                format!("<Triangle: ({}, {}), ({}, {}), ({}, {})>", x1, y1, x2, y2, x3, y3)
            }
        }
    }

    fn area(&self) -> f32{
        match self{
            Shapes::Circle{ x: i32, y, radius } => {
                std::f32::consts::PI  * (*radius as f32) * (*radius as f32)
            }
            Shapes::Rectangle { x, y, width, height } => {
                (*width as f32) * (*height as f32)
            }
            Shapes::Triangle { x1, y1, x2, y2, x3, y3 } => {
                0.5 * ((x1 - x3) * (y2 - y1) - (x1 - x2) * (y3 - y1)) as f32
            }
        }
    }
}


fn input_shape_list() -> Vec<Box<dyn Shape>> {
    vec![
        Circle::new(0, 0, 1),
        Rectangle::new(40, 40, 20, 20),
        Triangle::new(0, 20, 30, 70, 90, 120)
    ]
}

const EXPECTED_001: &[&str] = &[
    "<Circle: 0, 0, 1>",
    "<Rectangle: 40, 40, 20, 20>",
    "<Triangle: (0, 20), (30, 70), (90, 120)>",
];

const EXPECTED_002: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Triangle: (0, 20), (30, 70), (90, 120)>, area: 750.00",
];

#[test]
fn test_shapes_001() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| s.rep_string());
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_001);
}

#[test]
fn test_shapes_002() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(
        |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}


#[test]
fn test_shapes_003() {
    let input_list = input_shape_list();
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(
        |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}

// 2.1
#[derive(Clone)]
enum Text {
    Plain(String),
    Repeated(Box<Text>, usize),
    Joined(Vec<Box<Text>>, String)
}

impl Text {
    fn value(&self) -> String {
        match self {
            Text::Plain(t) => t.clone(),
            Text::Repeated(elem, n) => {
                elem.value().repeat(*n)
            }
            Text::Joined(data, sep) => {
                let result = data.iter().map(|t| t.value()).collect::<Vec<_>>().join(sep);
                result
            }
        }
    }
}

impl From<&Text> for Box<Text> {
    fn from(t: &Text) -> Box<Text> {
        Box::new(t.clone()) 
    }
}

impl AsRef<Text> for Text {
    fn as_ref(&self) -> &Text { &self }
}

impl Into<String> for Text{
    fn into(self) -> String{
        self.value()
    }
}

#[test]
fn test_text_repeated() {
    let t1 = Text::Plain("Hi".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    assert_eq!(t1.value(), "Hi");
    assert_eq!(t2.value(), "[+]");
    assert_eq!(t3.value(), "[+]".repeat(3));
    assert_eq!(t4.value(), "[+]".repeat(15));
}

#[test]
fn test_text_composition() {
    let t1 = Text::Plain("x|x".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    let mut tvec: Vec<Box<Text>> = Vec::new();
    tvec.push(t1.into());
    tvec.push(t2.into());
    tvec.push(t3.into());
    tvec.push(t4.into());
    let t5 = Text::Plain("--".into());
    let t6 = Text::Joined(tvec, t5.into());
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}


//2.2
trait Texts {
    fn value(&self) -> String;
    fn clone_box(&self) -> Box<dyn Texts>;
}
    
impl Clone for Box<dyn Texts> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
    
#[derive(Clone)]
struct PlainText { 
    chars: String 
}


impl From<&str> for PlainText {
    fn from(text: &str) -> PlainText {
        PlainText{ chars: text.to_string() }
    }
}

impl Texts for PlainText {
    fn value(&self) -> String { 
        self.chars.clone() 
    }
    fn clone_box(&self) -> Box<dyn Texts> { 
        Box::new(self.clone()) 
    }
}

impl AsRef<dyn Texts> for PlainText {
    fn as_ref(&self) -> &(dyn Texts + 'static) { 
        self 
    }
}


impl From<&PlainText> for Box<dyn Texts> {
    fn from(t: &PlainText) -> Box<dyn Texts> {
        Box::new(t.clone()) 
    }
}

#[derive(Clone)]
struct RepeatedText {
    text: Box<dyn Texts>,
    n: usize,
}

impl From<&RepeatedText> for Box<dyn Texts> {
    fn from(t: &RepeatedText) -> Box<dyn Texts> {
        Box::new(t.clone()) 
    }
}

impl RepeatedText {
    fn with_parts<T: Into<Box<dyn Texts>>>(text: T, num: usize) -> RepeatedText {
        RepeatedText {
            text: text.into(),
            n: num,
        }
    }
}

impl Texts for RepeatedText {
    fn value(&self) -> String {
        self.text.value().repeat(self.n)
    }
    fn clone_box(&self) -> Box<dyn Texts> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct JoinedText{
    data: Vec<Box<dyn Texts>>, 
    sep: PlainText
}

impl Texts for JoinedText {
    fn value(&self) -> String {
        self.data
            .iter()
            .map(|t| t.value())
            .collect::<Vec<String>>()
            .join(&self.sep.value())
    }

    fn clone_box(&self) -> Box<dyn Texts> {
        Box::new(self.clone())
    }
}


impl JoinedText {
    fn with_parts(data: &Vec<Box<dyn Texts>> , sep: &PlainText) -> JoinedText{
        JoinedText {
            data: data.to_vec(),
            sep: sep.clone(),
        }
    }
}


#[test]
fn test_text_repeated2() {
    let t1 = PlainText::from("Hi");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    assert_eq!(t1.value(), "Hi");
    assert_eq!(t2.value(), "[+]");
    assert_eq!(t3.value(), "[+]".repeat(3));
    assert_eq!(t4.value(), "[+]".repeat(15));
}

#[test]
fn test_text_composition2() {
    let t1 = PlainText::from("x|x");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    let mut tvec: Vec<Box<dyn Texts>> = Vec::new();
    tvec.push(t1.clone_box());
    tvec.push(t2.clone_box());
    tvec.push(t3.clone_box());
    tvec.push(t4.clone_box());
    let t5= PlainText::from("--");
    let t6 = JoinedText::with_parts(&tvec, &t5);
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}


