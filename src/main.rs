mod cpu;

fn main() {
    println!("Hello, world!");
    let x = cpu::instructions::Instruction::from_str("NEG R1");
    println!("{:?}", x);

}
