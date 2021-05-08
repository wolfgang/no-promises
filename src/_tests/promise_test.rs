use std::{thread, time};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::promise::Promise;
use crate::promise;

#[async_std::test]
async fn wait_for_value() {
    assert_eq!(1, 1);

    let p: Promise<i32> = Promise::new(|resolve, _| {
        thread::sleep(time::Duration::from_millis(10));
        resolve(1234)
    });

    assert_eq!(p.await, Ok(1234));
}

#[async_std::test]
async fn wait_for_string() {
    let p: Promise<String> = Promise::new(|resolve, _| {
        resolve(String::from("HELLO"))
    });

    assert_eq!(p.await, Ok("HELLO".into()));
}

#[async_std::test]
async fn reject_promise() {
    assert_eq!(1, 1);

    let p: Promise<i32> = Promise::new(|_, reject| {
        reject(promise::Error::from("promise rejected"))
    });

    assert_eq!(p.await, Err(promise::Error::from("promise rejected")));
}


#[async_std::test]
async fn read_from_socket() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    thread::spawn(move || {
        for stream in listener.incoming() {
            stream.unwrap().write(b"HELLO").unwrap();
        }
    });

    let promise: Promise<[u8; 10]> = Promise::new(|resolve, _| {
        let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        let mut buffer = [0; 10];
        stream.read(&mut buffer).unwrap();
        resolve(buffer)
    });

    let result = promise.await;
    assert!(result.is_ok());
    assert_eq!(&result.unwrap()[0..5], b"HELLO");
}

#[async_std::test]
async fn fail_to_read_from_socket() {
    let promise: Promise<()> = Promise::new(|resolve, reject| {
        let stream = TcpStream::connect("127.0.0.1:9090");
        match stream {
            Ok(_) => { resolve(()) }
            Err(e) => { reject(promise::Error::from(e)) }
        }
    });

    let result = promise.await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().starts_with("Connection refused"));
}

#[async_std::test]
async fn resolve_or_fail() {
    assert_eq!(promise_div(10.0, 0.0).await, Err(promise::Error::from("division by zero")));
    assert_eq!(promise_div(10.0, 2.0).await, Ok(5.0));
}

#[async_std::test]
async fn promised_uppercase_string() {
    assert_eq!(promise_uppercase(String::from("hello")).await,
               Ok(String::from("HELLO")));
}

fn promise_div(a: f32, b: f32) -> Promise<f32> {
    Promise::new(move |resolve, reject| {
        if b == 0.0 { reject(promise::Error::from("division by zero")) } else { resolve(a / b) }
    })
}

fn promise_uppercase(s: String) -> Promise<String> {
    Promise::new(move |resolve, _| {
        resolve(s.to_ascii_uppercase())
    })
}
