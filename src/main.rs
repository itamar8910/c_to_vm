mod cpu;
mod operating_system;

fn main() {
    println!("Hello, world!");
    let x = cpu::instructions::Instruction::from_str("NEG R1");
    println!("{:?}", x);

}
