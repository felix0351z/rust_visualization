extern crate core;

use std::thread::sleep;
use std::time::Duration;
use visualization_test::engine::errors::SenderError;
use visualization_test::engine::sender::Sender;

use error_stack::Result;

#[test]
fn test_creation() -> Result<(), SenderError> {
    Sender::new()?;
    Ok(())
}

#[test]
fn test_sync() -> Result<(), SenderError> {
    let white: Vec<u8> = vec![255; 60*3];
    let black: Vec<u8> = vec![0; 60*3];

    let sender = Sender::new()?;


    let mut switch = false;

    // 10 Seconds
    for i in 0..50*10 {
        if switch {
            sender.send(white.as_slice()).unwrap();
        } else {
            sender.send(black.as_slice()).unwrap();
        }
        switch = !switch;
        sleep(Duration::from_millis(20)); //50FPS
    }

    Ok(())

}

#[test]
fn test_async() -> Result<(), SenderError> {
    let sender = Sender::new()?;
    let sender2 = sender.clone()?;
    let sender3 = sender.clone()?;


  std::thread::spawn(move || {
      let white: Vec<u8> = vec![255; 60*3];
      let black: Vec<u8> = vec![0; 60*3];
      let mut switch = false;

      loop {
          if switch {
              sender.send(white.as_slice()).unwrap();
          } else {
              sender.send(black.as_slice()).unwrap();
          }
          switch = !switch;
          sleep(Duration::from_millis(20)); //50FPS
      }
  });

  std::thread::spawn(move || {
      let white: Vec<u8> = vec![255; 60*3];
      let black: Vec<u8> = vec![0; 60*3];
      let mut switch = false;

      loop {
          if switch {
              sender2.send(white.as_slice()).unwrap();
          } else {
              sender2.send(black.as_slice()).unwrap();
          }
          switch = !switch;
          sleep(Duration::from_millis(20)); //50FPS
      }
  });

  std::thread::spawn(move || {
      let white: Vec<u8> = vec![255; 60*3];
      let black: Vec<u8> = vec![0; 60*3];
      let mut switch = false;

      loop {
          if switch {
              sender3.send(white.as_slice()).unwrap();
          } else {
              sender3.send(black.as_slice()).unwrap();
          }
          switch = !switch;
          sleep(Duration::from_millis(20)); //50FPS
      }
  });

  sleep(Duration::from_secs(10));
  Ok(())
}

