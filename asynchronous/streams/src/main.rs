// // tokio
// use tokio_stream::StreamExt;
// use tokio::time::{ sleep, Duration, timeout };
// use tokio::pin;

// fn numbers_stream() -> impl tokio_stream::Stream<Item = i32> {
//     tokio_stream::iter(1..=10).then(|x| async move {
//         sleep(Duration::from_millis(200)).await;
//         x
//     })
// }


// async fn longer_task(durations: i32) -> String {
//   match timeout(Duration::from_millis(durations.try_into().unwrap()), async move {
//     sleep(Duration::from_secs(2)).await;
//     "success"
//   }).await {
//     Ok(s) => s.to_string(),
//     Err(_) =>  "Task was cancelled due to timeout.".to_string(),
//   }
// }

// #[tokio::main]
// async fn main() {
//     let values = vec![1, 2, 3, 4, 5];
//     let mut stream = tokio_stream::iter(values);
//     while let Some(item) = stream.next().await {
//       println!("Tokio stream item: {}", item);
//     }

//     let numbers = numbers_stream();
//     pin!(numbers);
//     while let Some(number) = numbers.next().await {
//       println!("Tokio fn stream item: {}", number);
//     }

//     let task = longer_task(1500).await;
//     println!("{}", task)
// }


// trpl
use trpl::{ Stream, StreamExt, ReceiverStream };
use std::time::Duration;

fn number_stream() -> impl Stream<Item = i32>{
  let (tx, rx) = trpl::channel();

  trpl::spawn_task(async move {
    for i in 1..10 {
      trpl::sleep(Duration::from_millis(200)).await;
      tx.send(i).unwrap();
    }
  });

  ReceiverStream::new(rx)
}

async fn longer_task(durations: i32) -> String {
  let fut = async {
    trpl::sleep(Duration::from_secs(2)).await;
    "success".to_string()
  };
  match trpl::race(fut, trpl::sleep(Duration::from_millis(durations.try_into().unwrap()))).await {
    trpl::Either::Left(output) => output,
    trpl::Either::Right(_) => "Task was cancelled due to timeout.".to_string(),
  }
}

fn main() {
    trpl::run(async {
      let values = vec![1, 2, 3, 4, 5];
      let mut stream = trpl::stream_from_iter(values);
      while let Some(item) = stream.next().await {
        println!("trpl stream item: {}", item);
      }

      let mut numbers = number_stream();
      while let Some(number) = numbers.next().await {
        println!("Trpl fn stream item: {}", number);
      }

      let task = longer_task(2000).await;
      println!("task: {}", task);
    })
}