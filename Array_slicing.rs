fn main() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // a. Slice of 2nd and 3rd element
    println!("2nd and 3rd elements: {:?}", &arr[1..3]);

    // b. Omit start index (upto 5th)
    println!("Omit start index (upto 5th): {:?}", &arr[..5]);

    // c. Omit end index (from 5th)
    println!("Omit end index (from 5th): {:?}", &arr[5..]);

    // d. Omit both start and end
    println!("Whole array using slice: {:?}", &arr[..]);
}
