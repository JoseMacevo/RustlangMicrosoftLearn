pub fn run(){
    // Declare vector initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // Create an empty vector, declare vector mutable so it can grow and shrink
    let mut fruits = Vec::new();
    // Push values onto end of vector, type changes from generic 'T' to String

    fruits.push("Orange");
    fruits.push("Apples");
    fruits.push("Cherries");
    fruits.push("Bananas");
    println!("Fruits: {:?}", fruits);
    
    // Pop off value at end of vector
    // Call pop() method from inside println! macro
    println!("Pop off: {:?}", fruits.pop());
    println!("Fruits: {:?}", fruits);

    // Declare a a new vector, initalize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    // Add 5 to the value at index 1, which is 5+3 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);
    
    let beyond = index_vec[5];
    println!("{}", beyond);
}
