fn main() {
// To the Lighthouse
    let fine = false;
    let weather = if fine {
        1
    } else {
        3650
    };

    println!("We will go to the Lighthouse in {} days", weather);
}
