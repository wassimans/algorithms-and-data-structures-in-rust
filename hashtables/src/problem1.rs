pub fn identify_identical(values: Vec<u32>) {
    for (i, e1) in values.iter().enumerate() {
        for e2 in &values[i + 1..] {
            if e1 == e2 {
                println!("E1: {}, E2: {}", e1, e2);
                println!("Twin integers found!");
                return;
            }
        }
    }
    println!("No twin integers are alike.");
}
