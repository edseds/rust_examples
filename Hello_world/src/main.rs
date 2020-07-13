#![allow(dead_code)]
use std::mem;

mod sh;
mod pm;
mod combination_locks;
mod data_structures;
mod collections;
mod strings;
mod functions;
mod methods;


const MEANING_OF_LIFE: u8 = 42;
static mut Z: i32 = 123;

fn scope_and_shadowing() {
    let a = 123;

    println!("a={}", a);
    {
        let b = 456;
        let a = 435;
        println!("inside, b = {}", b);
        println!("inisde, a={}", a);
    }
}

fn main() {
    /* println!("Hello, world!");
        println!("{}",MEANING_OF_LIFE);
        unsafe {
            Z=655;
            println!("{}", Z);
        }
        dataTypes();
        operators();
        scope_and_shadowing();
    */
    // sh::stack_and_heap();
    //if_statement();
    //while_and_loop();

    //for_loop();
    //match_statement();

    //combinationLock::combination_lock();
    //dataStructures::structures();
    data_structures::enums();
    data_structures::unions();
    data_structures::option_of_t();
    data_structures::arrays();
    data_structures::slices();
    data_structures::tuples();
    pm::pattern_matching();
    data_structures::generics();
    collections::vectors();
    collections::hash_map();
    collections::hash_set();
    strings::strings();
    strings::string_format();
    //strings::number_guessing();
    functions::print_value(20);

    let mut z = 1;
    functions::increase(&mut z);

    println!("Value = {}", z);

    let a = 3;
    let b = 5;
    let p = functions::product(a, b);

    println!("{} * {} = {}", a, b, p);


    methods::methods();
    functions::closure();
    functions::higher_order_functions();
}

fn data_types() {
    let a: i8 = -123; //8 bits a - unsigned 0 + u - unsigned integer i - integer 0 ..255
    println!("a= {}", a);

    //a = 555;

    //mut

    let mut b: i8 = 0; //mutable mut
    println!("b={}", b);
    b = 54;
    println!("b={}", b);

    let mut c = 123456789; //32-bit signed i32
    println!("c={}, size = {} bytes", c, mem::size_of_val(&c));

    c = -1;
    println!(
        "c={} after modification, size = {} bytes",
        c,
        mem::size_of_val(&c)
    );
    //i8 u8 i16 u16 i32 u32 i64 u64

    let z: isize = 123; //isize/usize
    let size_of_z = mem::size_of_val(&z);

    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d = 'x';
    println!("d={}, size = {} bytes", d, mem::size_of_val(&d));

    let e: f32 = 2.5; //double-precision 8 bytes or 64-bits
    println!("e={}, size = {} bytes", e, mem::size_of_val(&e));

    //true false
    let g = false;
    println!("g={}, size = {} bytes", g, mem::size_of_val(&g));
    let _f = 4 > 0; //true
}

fn operators() {
    //arithmetic
    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a = a + 1; // -- ++
    a -= 2; // a = a-2
            // -= += *= /= %=

    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed= {},^pi = {}", b, b_cubed, b_to_pi);

    //bitwise

    let c = 1 | 2; // | OR & AND ^ XOR !NOR
    println!("1|2 = {}", c); //01 OR 10 = 11 == 3_10

    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {}", two_to_10);

    //logical
    let _pi_less_4 = std::f64::consts::PI < 4.0; // true
                                                 // > <= >= ==
    let x = 5;
    let _x_is_5 = x == 5; //true
}

fn if_statement() {
    let temp = 3;

    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10
    {
        println!("really cold");
    }
    else {
        println!("temperture is ok");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("Today is {}",day);

    println!("it is {}",
    if temp>20 {"hot"} else if temp < 10 {"Cold"} else {"OK"});

    println!("it is {}",
    if temp > 20 {
        if temp >30 {"very hot"} else {"hot"}
    } else if temp < 10 {"Cold"} else {"OK"});
}

fn while_and_loop()
{
    let mut x = 1;

    while x < 1000
    {
        x *=2;

        if x == 64 {continue;}


        println!("x={}",x);
    }

    let mut y =1;
    loop //while true
    {
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 { break; }
    }
}

fn for_loop()
{
    for x in 1..11
    {
        if x==3 {continue;}

        if x==8 {break;}

        println!("x={}",x);
    }

    for (pos,y) in (30..41).enumerate()
    {
        println!("{}: {}",pos,y);
    }
}

fn match_statement()
{
    let country_code = 2000;

    let country = match country_code
    {
        44 => "UK",
        46 => "Sweden",
        7=> "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };

    println!("the country with code {} is {}",country_code,country);
}

