
// Use trpl

use std::time::Duration;

async fn compute_square(x: i32) -> i32 {
  trpl::sleep(Duration::from_millis(200)).await;
  x * x
}

async fn get_users() -> Vec<String> {
    trpl::sleep(Duration::from_millis(500)).await;
    let users = vec!["A".to_string(), "B".to_string()];
    users
}


async fn get_products() -> Vec<String> {
    trpl::sleep(Duration::from_millis(400)).await;
    let products = vec!["p1".to_string(), "p2".to_string()];
    products
}

async fn get_orders() -> Vec<String> {
  trpl::sleep(Duration::from_millis(699)).await;
  let orders = vec![];
  orders
}


fn main() {
  trpl::run(async {
    let fut = compute_square(5);
    let res = fut.await;
    println!("Result: {}", res);
    let (u, p, o) = trpl::join!(get_users(), get_products(), get_orders());
    println!("Users: {:?}", u);
    println!("Products: {:?}", p);
    println!("Orders: {:?}", o);
  })
}


// Use tokio
// use tokio::time::{ sleep, Duration };

// async fn compute_square(x: i32) -> i32 {
//   sleep(Duration::from_millis(500)).await;
//   x.pow(2)
// }

// async fn get_users() -> Vec<String> {
//     sleep(Duration::from_millis(500)).await;
//     let users = vec!["A".to_string(), "B".to_string()];
//     users
// }


// async fn get_products() -> Vec<String> {
//     sleep(Duration::from_millis(400)).await;
//     let products = vec!["p1".to_string(), "p2".to_string()];
//     products
// }

// async fn get_orders() -> Vec<String> {
//   sleep(Duration::from_millis(699)).await;
//   let orders = vec![];
//   orders
// }

// #[tokio::main]
// async fn main() {
//   let fut = compute_square(5);
//   let res = fut.await;
//   println!("Result: {}", res);

//   let (u, p, o) = tokio::join!(get_users(), get_products(), get_orders());
//   println!("Users: {:?}", u);
//   println!("Products: {:?}", p);
//   println!("Orders: {:?}", o);
// }

