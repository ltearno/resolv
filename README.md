# Rust Adventure

The idea is to learn the Rust language.

We will build a `make`-like tool. We will call it `resolv`

`resolve` will maintain a state in `.resolv` folder in the directory from where it is executed. The user can reset the state by removing this directory.

Input files will be named `Resolvfile`.

It will be a text file, with blocs of text separated by empty lines (at least one).

Each block describes a dependency rule composed of :

- one line for its name,
- one line for the name of the rules it depends on,
- the rest of the lines as a text chunk to pass to a `sh` command

## Rust installation

See the book : https://doc.rust-lang.org/stable/book/ch01-01-installation.html

## Project initialisation

Create the directory and go inside.

    rustup update
    rustc --version

Create `main.rs` file with this content :

```rust
fn main() {
    println!("Hello, world!");
}
```

Compilation :

    rustc main.rs
    ./main

## Cargo

    cargo init

    cargo build
    cargo build --release
    cargo check

    target/debug/rust-adventure

    # or cargo run

## First exercise of the book

You end up writing this :

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number !");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## The language

Raw identifier : prefix identifier with `r#`

### Types

Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize

f32, f64

bool

char

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

#### Tuples

`let tup: (i32, f64, u8) = (500, 6.4, 1);`

Can be exploded : `let (x, y, z) = tup;`

Access through `.0`, `.1` etc...

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

#### Arrays

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

### If

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}

### Loops

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}


while number != 0 {
    println!("{}!", number);

    number = number - 1;
}

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}




```rust
#![allow(unused_variables)]
fn main() {
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
}
```

Impossible to create a dandling pointer :

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}




#![allow(unused_variables)]
fn main() {
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
}

### Slices

String slices :

#![allow(unused_variables)]
fn main() {
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
}

Other slices :


#![allow(unused_variables)]
fn main() {
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
}

// `slice` has the type `&[i32]`

### Structs


#![allow(unused_variables)]
fn main() {
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
}



#![allow(unused_variables)]
fn main() {
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
}

Copying fields from another instance :


#![allow(unused_variables)]
fn main() {

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
}

### Tuple struct

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);









#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
    // can also use {:#}
}




### Enums


#![allow(unused_variables)]
fn main() {
enum IpAddrKind {
    V4,
    V6,
}
}









## First problems

With that code, because of course of borrowing :

```rust
fn main() {
    println!("Resolv v0.1, welcome\n");

    let rules: Vec<Rule> = Vec::new();
    let mut current_rule: Option<Rule> = Option::None;

    let f = File::open("Resolvefile").expect("did not find Resolvefile");
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        if l.trim().is_empty() {
            if current_rule.is_some() {
                current_rule = Option::None;
            }
        } else {
            if current_rule.is_none() {
                current_rule = Some(Rule {
                    name: l,
                    dependencies: Vec::new(),
                    script: String::from(""),
                });
            } else {
                let rule = current_rule.unwrap();
                if rule.dependencies.len() == 0 {
                    rule.dependencies.push(l.trim().split(" ").collect())
                };
            }
        }
        println!("{}", l);
    }
}
```

Apparently this is called _fighting with the borrow checker_ in the Rust community...
That's a sign of progress, keep on !

### Resolution

Makes me thinking that it's incredible the number of errors the Rust borrow checker
sees in what I thought as universal idiomatic code ! I like that ! The result is way
better, more logical. And it is satisfying !

```rust
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Rule<'a> {
    name: Option<&'a str>,
    dependencies: Option<&'a str>,
    script: Option<&'a str>,
}

impl<'a> Rule<'a> {
    fn new() -> Rule<'a> {
        Rule {
            name: None,
            dependencies: None,
            script: None,
        }
    }
}

enum State {
    Waiting,
    Completing,
}

fn fetch_lines() -> Vec<String> {
    let f = File::open("Resolvfile").expect("did not find Resolvefile");
    let buf_reader = BufReader::new(&f);
    let lines = buf_reader.lines();

    let mut result = Vec::new();
    for line in lines {
        result.push(line.unwrap());
    }

    result.push(String::from(""));

    result
}

fn main() {
    println!("Resolv v0.1, welcome\n");

    let mut rules: Vec<Rule> = Vec::new();

    let lines = fetch_lines();

    let mut current_rule = Rule::new();
    let mut state: State = State::Waiting;

    fn push_and_prepare<'a>(rules: &mut Vec<Rule<'a>>, rule: Rule<'a>) -> Rule<'a> {
        rules.push(rule);
        Rule::new()
    }

    for line in &lines {
        if let State::Waiting = state {
            if !line.is_empty() {
                state = State::Completing;
            }
        }

        if let State::Completing = state {
            if line.is_empty() {
                println!("processed rule {:?}", current_rule);
                current_rule = push_and_prepare(&mut rules, current_rule);

                state = State::Waiting;
            } else {
                complete_rule(&mut current_rule, line);
            }
        }
    }

    println!("rules: {:?}", rules);
}

fn complete_rule<'a>(r: &mut Rule<'a>, line: &'a str) {
    if let None = r.name {
        r.name = Some(line);
    } else if let None = r.dependencies {
        r.dependencies = Some(line);
    } else if let None = r.script {
        r.script = Some(line);
    }
}
```