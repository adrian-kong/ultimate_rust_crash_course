const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    // let foo = 3;
    println!("Firing {} of my {} missiles...", ready, missiles);
    // println!("{} missiles left!", missiles - ready);
    missiles -= ready;
    println!("{} missiles left!", missiles);

    // READY_AMOUNT = 1
}
