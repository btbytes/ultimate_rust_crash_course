const NUM_MISSILES: i32 = 8;
const NUM_READY: i32 = 2;

fn main() {
    let mut missiles: i32 = NUM_MISSILES;
    let ready: i32 = NUM_READY;
    println!("Firing {} of my {} missiles", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
