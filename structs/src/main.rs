/*
#[derive(Debug)]
struct User {
    first_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(first_name: String, email: String) -> User {
    User {
        email, // I'm impressed they adopted this shorthand
        first_name,
        active: true,
        sign_in_count: 1,
    }
}

fn from_user(user: &User, first_name: String, email: String) -> User {
    User {
        email, // I'm impressed they adopted these shorthands
        first_name,
        ..*user
    }
}

struct Vertice(i64, i64, String);
struct Edge(i64, i64, f64);

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_contain(&self, rect: &Rectangle) -> bool {
        self.length > rect.length && self.width > rect.width
    }
}
impl Rectangle {
    fn empty() -> Rectangle {
        Rectangle {
            length: 0,
            width: 0,
        }
    }
}
fn main() {
    let user1 = build_user(String::from("Cole"), String::from("cole@google.com"));
    let user2 = from_user(
        &user1,
        String::from("Jeremy"),
        String::from("jeremy@coderocket.org"),
    );

    let r2 = Rectangle {
        width: 129,
        length: 99,
    };
    println!(
        "{}",
        Rectangle {
            width: 100,
            length: 100,
        }.can_contain(&r2)
    );

    println!("{:?}", Rectangle::empty());
    let a = SketchMessage::PostMessage(String::from("Hello!"));
    println!("Var {:?}", a);
    a.print();
    let a = SketchMessage::StartTyping;
    a.print();
    let a = SketchMessage::StopTyping;
    a.print();
    let a = SketchMessage::ChangeColor(12, 34, 209);
    a.print();
    let a = SketchMessage::Start { x: 12, y: 24 };
    a.print();
    let a = SketchMessage::Move { x: 15, y: 34 };
    a.print();
    let a = SketchMessage::Move { x: 16, y: 42 };
    a.print();
    let a = SketchMessage::Stop;
    a.print();
    println!("Call {:?}", SketchMessage::ChangeColor(252, 254, 12));
    println!("{:0>3}", 12);

    let test_values = vec![
        "112 23",
        "124 29",
        "12429",
        "12a 429",
        "224 29",
        "22429",
        "22a 429",
        "312 23 67",
        "312a 2391 6721",
    ];
    for m in test_values.iter() {
        match SketchMessage::deserialize(&m) {
            Ok(sm) => sm.print(),
            Err(err_str) => println!("Error parsing message:\n> {}", err_str),
        }
    }
}

#[derive(Debug)]
enum SketchMessage {
    Stop,
    Start { x: i32, y: i32 },
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
    ChangeStroke(u8),
    PostMessage(String),
    StartTyping,
    StopTyping,
}

fn get_x_and_y<I: std::str::FromStr>(s: &str) -> Result<(I, I), String>
where
    <I as std::str::FromStr>::Err: std::fmt::Display,
{
    let split = ' ';
    let mut it = s.split(split);
    match (it.next(), it.next()) {
        (Some(xs), Some(ys)) => {
            let parsed = (xs.parse::<I>(), ys.parse::<I>());
            match parsed {
                (Ok(x), Ok(y)) => Ok((x, y)),
                _ => {
                    let mut errors: String = String::from("");
                    let (xe, ye) = parsed;
                    if let Err(err) = xe {
                        errors.push_str(&format!("failed to parse X value {:?}: {}, ", xs, err));
                    }
                    if let Err(err) = ye {
                        errors.push_str(&format!("failed to parse Y value {:?}: {}, ", ys, err));
                    }
                    Err(format!("failed to parse {:?}: ({})", s, errors))
                }
            }
        }
        _ => Err(format!(
            "invalid string {:?} needs to be split by {:?} into 2 parts",
            s,
            split
        )),
    }
}

fn parse_with_error<I>(s: &str, err_message: &str) -> Result<I, String>
where
    I: std::str::FromStr,
    <I as std::str::FromStr>::Err: std::fmt::Display,
{
    match s.parse::<I>() {
        Ok(x) => Ok(x),
        Err(err) => Err(format!("{}: {}", err_message, err)),
    }
}

fn get_r_g_and_b<I: std::str::FromStr>(s: &str) -> Result<(I, I, I), String>
where
    <I as std::str::FromStr>::Err: std::fmt::Display,
{
    let split = ' ';
    let mut it = s.split(split);

    match (it.next(), it.next(), it.next()) {
        (Some(rs), Some(gs), Some(bs)) => {
            let parsed = (rs.parse::<I>(), gs.parse::<I>(), bs.parse::<I>());
            match parsed {
                (Ok(r), Ok(g), Ok(b)) => Ok((r, g, b)),
                _ => {
                    let mut errors: String = String::from("");
                    let (re, ge, be) = parsed;
                    if let Err(err) = re {
                        errors.push_str(&format!("failed to parse Red value {:?}: {}, ", rs, err));
                    }
                    if let Err(err) = ge {
                        errors.push_str(&format!("failed to parse Green value {:?}: {}, ", gs, err));
                    }
                    if let Err(err) = be {
                        errors.push_str(&format!("failed to parse Blue value {:?}: {}, ", bs, err));
                    }
                    Err(format!("failed to parse {:?}: ({})", s, errors))
                }
            }
        }
        _ => Err(format!(
            "invalid string {:?} needs to be split by {:?} into 3 parts",
            s,
            split
        )),
    }
}

impl SketchMessage {
    fn print(&self) {
        let message = match self {
            &SketchMessage::Stop => format!("Stop drawing!"),
            &SketchMessage::Start { x, y } => format!("Start drawing from {}, {}", x, y),
            &SketchMessage::Move { x, y } => format!("I'm moving to {}, {}", x, y),
            &SketchMessage::ChangeColor(r, g, b) => format!("Change color to {}, {}, {}", r, g, b),
            &SketchMessage::ChangeStroke(w) => format!("Change stroke to width {}", w),
            &SketchMessage::PostMessage(ref message) => format!("I say, {}", message),
            &SketchMessage::StartTyping => format!("I'm typing!"),
            &SketchMessage::StopTyping => format!("I'm not typing!"),
        };
        println!("{}", message)
    }
    fn serialize(&self) -> String {
        match self {
            &SketchMessage::Stop => format!("0"),
            &SketchMessage::Start { x, y } => format!("1{x} {y}", x = x, y = y),
            &SketchMessage::Move { x, y } => format!("2{x} {y}", x = x, y = y),
            &SketchMessage::ChangeColor(r, g, b) => format!("3{} {} {}", r, g, b),
            &SketchMessage::ChangeStroke(w) => format!("4{}", w),
            &SketchMessage::PostMessage(ref message) => format!("5{}", message),
            &SketchMessage::StartTyping => format!("6"),
            &SketchMessage::StopTyping => format!("7"),
        }
    }
    fn deserialize(s: &str) -> Result<SketchMessage, String> {
        let code = s.chars().nth(0).unwrap();
        match code {
            '0' => Ok(SketchMessage::Stop),
            '1' => {
                match get_x_and_y::<i32>(&s[1..]) {
                    Ok((x, y)) => Ok(SketchMessage::Start { x, y }),
                    Err(err) => Err(format!("failed to parse Start Message {:?}: {}", s, err)),
                }
            }
            '2' => {
                match get_x_and_y::<i32>(&s[1..]) {
                    Ok((x, y)) => Ok(SketchMessage::Move { x, y }),
                    Err(err) => Err(format!("failed to parse Move Message {:?}: {}", s, err)),
                }
            }
            '3' => {
                match get_r_g_and_b::<u8>(&s[1..]) {
                    Ok((r, g, b)) => Ok(SketchMessage::ChangeColor(r, g, b)),
                    Err(err) => Err(format!("failed to parse ChangeColor {:?}: {}", s, err)),
                }
            }
            /*
            {x, y} => format!("1{x}_{y}|", x=x, y=y) => SketchMessage::Start,
            {x, y} => format!("2{x}_{y}|", x=x, y=y) => SketchMessage::Move,
            (r, g, b) => format!("3{:0>3}{:0>3}{:0>3}|", r, g, b) => SketchMessage::ChangeColor,
            (w) => format!("4{:0>3}|", w) => SketchMessage::ChangeStroke,
            (ref message) => format!("5{}|", message) => SketchMessage::PostMessage,
             => format!("6") => SketchMessage::StartTyping,
             => format!("7") => SketchMessage::StopTyping,
             */
            _ => Err(String::from("no bueno")),
        }
    }
}
*/
mod lib;
fn main() {
    println!("Hello, world!");
}
