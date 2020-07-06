use std::collections::{HashMap, HashSet};
use std::collections::hash_set;

pub fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("{:?}",a);

    a.push(44);

    println!("{:?}",a);

    //usize isize

    let idx:usize = 0;
    a[idx] = 321;
    println!("a[0] = {}",a[idx]);

    //Options
    match a.get(3)
    {
        Some(x) => println!("a[3] = {}",x),
        None => println!("Error, no such element")
    }

    for x in &a {println!("{}",x);}

    a.push(66);
    println!("{:?}",a);

    let last_elem = a.pop(); //Option
    println!("last elem is {:?}, a= {:?}",last_elem,a);

    while let Some(x) = a.pop()
    {
        println!("{}", x);
    }
}

pub fn hash_map()
{
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"),3);
    shapes.insert(String::from("square"),4);

    println!("a square has {} sides",shapes["square".into()]);

    for(key,value) in &shapes {
        println!("{} : {}", key,value);
    }

    shapes.insert("square".into(),5);
    println!("{:?}",shapes);

    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes
            .entry("circle".into())
            .or_insert(2);
        *actual = 0;
    }

    println!("{:?}",shapes);
}

pub fn hash_set()
{
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}",greeks);

    let added_vega = greeks.insert("vega");

    if added_vega {
        println!("We added vega!");
    }

    if !greeks.contains("kappa") {
        println!("We don't have this value!");
    }

    let removed = greeks.remove("delta");

    if removed {
        println!("We removed delta");
    }

    println!("{:?}",greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8, _1_10,
        _2_8.is_subset(&_1_10)
    );

    //disjoint = no common elemts
    println!(
        "is {:?} a subset of {:?}? {}",
        _1_5, _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    //union, intersection
    println!(
        "items in either {:?} and {:?} are {:?}",
        _2_8, _6_10,
        _2_8.union(&_6_10)
    );

    //difference
    //symetric_difference



}