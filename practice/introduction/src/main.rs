use num::complex::Complex;
use std::convert::TryInto;

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn find_discriminant(a: f32, b: f32, c: f32) -> f32 {
    (b * b) - (4.0 * a * c)
}

fn test_complex_numbers() {
    let a = Complex{re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}

fn test_collections() {
    let mut collection = [1, 2, 3, 4, 5];

    for item in 0..collection.len() {
        collection[item] = item * 2;
    }

    for item in collection {
        println!("Item: {}", item);
    }
}

fn test_floats() {
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON);

    let x: f32 = 1.0 / 0.0;
    println!("x is finite = {}", x.is_finite());
}

fn try_convert_vars() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (c + d) = {}", e);

    let f: i32 = 10;
    let g: u16 = 100;

    if f < (g as i32) {
        println!("Ten is less than one hundred.");
    }

    let g_ = g.try_into().unwrap();

    if f < g_ {
        println!("Ten is less than one hundred.");
    }
}

fn test_scopes() {    // Эта переменная живёт в функции main
    let long_lived_binding = 1;
    let shadowed_variable = 1;

    // Это блок, он имеет меньшую область видимости, чем функция main
    {
        // Эта переменная существует только в этом блоке
        let short_lived_binding = 2;
        let shadowed_variable = 2;

        println!("shadowed variable: {}", shadowed_variable);

        println!("inner short: {}", short_lived_binding);
    }
    // Конец блока
    println!("shadowed variable: {}", shadowed_variable);

    println!("outer long: {}", long_lived_binding);
}

fn lift_manager() {
    let floors: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for floor in floors {
        println!("floor: {}", floor);
    }

    println!("We at highest floor");
}

fn while_test() {
    let mut floor: i32 = 0;

    while floor != 0 {
        floor += 1;
        println!("floor is {}", floor);
    }

    println!("lift off");
}

fn main() {
    println!("RustyNotes Practice\n");

    let d: f32 = find_discriminant(1.0, -10.0, 4.0);
    println!("# find_discriminant(1.0, -10.0, 4.0) = {}", d);

    print!("# complex_numbers = ");
    test_complex_numbers();

    println!("# cycles and collections");
    test_collections();

    println!("# test floats");
    test_floats();

    println!("# convert vars");
    try_convert_vars();

    println!("# test scopes");
    test_scopes();

    println!("# `while` lift");
    while_test();

    println!("# lift manager");
    lift_manager();
}
