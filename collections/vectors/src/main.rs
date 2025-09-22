fn average(v: &Vec<i32>) -> f64 {
    let sum: i32 = v.iter().sum();
    let count = v.len() as f64;
    sum as f64 / count
}

fn remove_negatives(v: &mut Vec<i32>) {
    v.retain(|&x| x >= 0);
}

fn max(v: &Vec<i32>) -> Option<i32> {
    v.iter().cloned().max()
}

fn min(v: &Vec<i32>) -> Option<i32> {
    v.iter().cloned().min()
}


fn main() {
    let mut scores = vec![10, -5, 20, 15, -2, 30];
    println!("Original scores: {:?}", scores);
    println!("Average score: {}", average(&scores));
    println!("Max score: {:?}", max(&scores));
    println!("Min score: {:?}", min(&scores));
    remove_negatives(&mut scores);
    println!("Scores after removing negatives: {:?}", scores);
}
