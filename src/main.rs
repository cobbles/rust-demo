use std::error::Error;

// mod ui;

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn add_larger(a: usize, b: usize) -> Result<usize, String> {
    if b <= a {
        return Err("fuckyou".to_string());
    }
    return Ok(add(a, b));
}

struct Thing {
    a: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // println!("{}, {:?}", add_larger(1, 2)?, vec![1, 2, 3]);
    let mut my_var = Thing { a: 1 };
    my_var = Thing { a: 2 };
    my_func(my_var);
    my_var = Thing { a: 3 };
    my_func(my_var);
    Ok(())
}

fn my_func(ref thing: Thing) {
    println!("{}", thing.a);
}

// fn main() {
//     println!("hello");
// }
