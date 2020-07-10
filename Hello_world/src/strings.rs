use crate::while_and_loop;
use rand::Rng;
use std::io::stdin;

pub fn strings()
{
    // utf-8
    let s:&'static str = "hello there!";

    for c in s.chars().rev()
    {
        println!("{}",c);
    }

    if let Some(first_char) = s.chars().nth(0)
    {
        println!("first letter is {}",first_char);
    }

    // String
    //heap
    let mut letters = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}",letters);

    //let u:&str = &letters;
    // str from String
    let u: &str = &letters; // deref conversion
    // there are situations when the coercion does NOT happen

    // concatenation
    // String + str
    // String + &String


    let mut abc = String::from("hello world");
    abc.remove(0);
    abc.push_str("!!!!!!");
    println!("{}",abc.replace("ello", "goodbye"));
}

pub fn string_format()
{
    let name = "Edgars";
    let greeting = format!("Hi!, I'm {}",name);

    println!("{}",greeting);

    let hello = "Hello";
    let rust = "rust";

    let hello_rust = format!("{} {}!",hello,rust);
    println!("{}",hello_rust);

    let run = "run";
    let forest = "forest";

    let rfr = format!("{0}, {1}, {0}",
    run,forest);

    println!("{}",rfr);

    let info = format!(
        "the name's {last}. {first} {last}.",
        first = "james",
        last = "bond"
    );
    println!("{}",info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        data = "delta"
    );

    println!("{}",mixed);
}

pub fn number_guessing()
{
    let number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Enter your guess:");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess >100 {
                            println!("Your guess is out of range");
                        } else if guess < number {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Corect!!!");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!",e);
                    }
                }
            },
            Err(_) => continue,
        }
    }
}