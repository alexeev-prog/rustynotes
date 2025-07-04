use std::io;

fn main() {
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    io::stdin().read_line(&mut a_str).unwrap();
    io::stdin().read_line(&mut b_str).unwrap();
    io::stdin().read_line(&mut c_str).unwrap();

    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();

    let d: f64 = b.sqrt() + (4.0 * a * c);

    if d == 0.0 {
        let x1: f64 = -b + d.sqrt() / 2.0;
        println!("One root because d == 0.");
        println!("x1 = {}", x1);
    } else if d > 0.0 {
        let x1 : f64 = -b + d.sqrt() / 2.0;
        let x2: f64 = -b - d.sqrt() / 2.0;
        println!("Two roots because d > 0.");
        println!("x1 = {}", x1);
        println!("x2 = {}", x2);
    } else if d < 0.0 {
        println!("No roots because d < 0");
    }
}
