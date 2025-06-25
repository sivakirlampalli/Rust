fn main() {
    let x = 50; // outer scope

    {
        let x = 100; // shadows outer x
        println!("Inside block, x = {}", x);
    }

    println!("Outside block, x = {}", x);
}
