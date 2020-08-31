// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints


// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}


fn main() {
    let a: Option<u16> = Some(13);
    let b: Option<u16> = Some(99);
    print_number(a);
    print_number(b);

    let mut numbers: [Option<u16>; 5]= [None; 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 5) + 2) / (4 * 16)
        };

        let i = iter as usize;
        numbers[i] = Some(number_to_add);
    }
}
