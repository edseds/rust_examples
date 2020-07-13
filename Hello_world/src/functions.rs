pub fn print_value(x:i32)
{
    println!("value = {}",x);
}

pub fn product(x:i32, y:i32) -> i32
{
    x * y
}

pub fn increase(x: &mut i32)
{
    *x +=1;
}

pub fn closure()
{
    let plus_one = |x: i32| -> i32{ x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    {
        let plus_two = |x|
            {
                let mut z = x;
                z += two;
                z
            };

        println!("{} + 2 = {}", 3, plus_two(3));
    }

    let borrow_two = &mut two;

    // T: by value
    //T&
    //&mut &

    let plus_three = |x:&mut i32| *x +=3;
    let mut f= 12;
    plus_three(&mut f);
    println!("f = {}", f);
}

pub fn higher_order_functions()
{
    let limit = 500;
    let mut sum = 0;

    //let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i * i;

        if above_limit(isq)
        {
            break;
        } else if is_even(isq)
        {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum,x|sum + x);

    println!("hof sum = {}",sum2);

}

fn is_even(x:u32) -> bool
{
    x % 2 == 0
}

fn greater_than(limit: u32)
 -> impl Fn(u32) -> bool
{
   move |y| y > limit
}