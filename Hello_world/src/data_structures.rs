use crate::while_and_loop;
use std::mem;

struct Point {
    x:f64,
    y:f64
}

struct Line
{
    start:Point,
    end:Point
}

enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),//tuple
    Cmyk { cyan: u8, magenta: u8, yellow: u8, black: u8 } //struct
}

//32 bits
union IntOrFloat
{
    i: i32,
    f: f32
}

pub fn structures()
{
    let p = Point { x: 3.05, y: 4.0 };
    println!("Point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };

    let _myline = Line { start: p, end: p2 };
}

pub fn enums()
{
    let c:Color = Color::Cmyk {cyan:19,magenta:21,yellow:10,black:255};

    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RgbColor(0, 0, 0) => println!("Black"),
        Color::RgbColor(r, g, b) => println!("RGB ({},{},{})", r, g, b),
        Color::Cmyk {cyan:_,magenta:_,yellow:_,black:255} => println!("Black"),
        _ => ()
    }
}

pub fn unions()
{
    let mut iof = IntOrFloat {i:123};
    iof.i = 234;

    let value = unsafe { iof.i};
    println!("iof.i={}",value);

    process_value(IntOrFloat{i:6});

}

fn process_value(iof: IntOrFloat)
{
    unsafe {
        match iof {
            IntOrFloat {i:42} => {println!("meaning of life value");}
            IntOrFloat {f} => {println!("value = {}",f)}
        }
    }
}

pub fn option_of_t()
{
    let x = 3.0;
    let y = 2.0;

    //Option -> Some(v) | None
    let result =
        if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Cannot divide by zero")
    }

    if let Some(z) = result {
        println!("result = {}", z)
    }
}

pub fn arrays()
{
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}",
             a.len(), a[0]);
    a[0] = 321;
    println!("a[0]= {}", a[0]);

    println!("{:?}", a);

    if a == [321, 2, 3, 4, 5]
    {
        println!("match");
    }

    let b = [1u16; 10];

    for i in 0..b.len()
    {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i ==j
            {
                println!("mtx[{}][{}] = {}",i,j,mtx[i][j]);
            }
        }
    }
}

pub fn slices()
{
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    //use_slice(&mut data);

    println!("{:?}",data);
}

fn use_slice(slice: &mut[i32])
{
    println!("first elem = {}, len = {}",slice[0],slice.len());
    slice[0]=6666;
}


pub fn tuples()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp= {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //desctucturing
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4,7);
    let combined = (sp,sp2);

    println!("{:?}",combined);

    println!("last elem = {}",(combined.1).1);

    let((c,d),(e,f)) = combined;

    let foo = (true,42.9,-1i8);

    println!("{:?}",foo);

    let meaning = (42,);
    println!("{:?}",42);


}

fn sum_and_product(x:i32, y:i32) -> (i32,i32)
{
    (x + y, x * y)
}

//Option<T>
struct SecondPoint<T> {
    x:T,
    y:T
}
struct SecondLine<T>
{
    start: SecondPoint<T>,
    end: SecondPoint<T>
}


pub fn generics()
{
    let a = SecondPoint { x: 0.0, y: 5.8 };
    let b = SecondPoint { x: 1.2, y: 3.5 };

    let myline = SecondLine { start: a, end: b };
}
