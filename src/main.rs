// TODO: split up this function after learning module system
fn primitives_test() {
    println!("1 + 2 = {}", 1u32 + 2);
}

fn print_test() {
    // standard print macros
    println!("Typical print with line break");
    print!("Typical print without line break");
    print!(": Proof\n\n");

    // print with args
    println!("{{}} {} print", "args");
    println!("{{1}} {{0}} {1} {0} print", "args", "positional");
    println!(
        "{{second}} {{first}} {second} {first} print\n",
        first = "args",
        second = "named"
    );

    // number formatting
    println!("{{}} base 10 {}", 12345);
    println!("{{:b}} base 2 {:b}", 12345);
    println!("{{:o}} base 8 {:o}", 12345);
    println!("{{:x}} base 16 {:x}\n", 12345);

    // align formatting
    println!("{{num:>5}} print with align");
    println!("{num:>5}", num=1);
    println!("{{num:0>5}} print with zero filling align");
    println!("{num:0>5}", num=1);
    println!("{{num:>width$}} print with specified width align");
    println!("{num:>width$}", num=1, width=6);
    let num: i32 = 1;
    let width: usize = 5;
    println!("{{num:>width$}} print with surrounding variable align");
    println!("{num:>width$}");
}
fn main() {
    primitives_test();
//    print_test();
}
