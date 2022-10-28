// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;
// use std::sync::Mutex;

// use std::sync::{Arc, Mutex};

use std::sync::RwLock;

fn main() {
    // let v = vec![1, 3, 4];

    // let handle = thread::spawn(move || {
    // for i in 1..5 {
    // println!("hi number {} from the spawned thread!", i);
    // println!("move {:?}", v);
    // thread::sleep(Duration::from_millis(1));
    // }
    // });

    // 阻塞线程执行
    // handle.join().unwrap();

    // for i in 1..5 {
    // println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // 消息通道
    // 多发送者 单接收者
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     tx.send(1).unwrap();
    // });

    // 不阻塞方法、会直接执行、导致没有接受到子线程发送的数据
    // println!("receive try {:?}", rx.try_recv().unwrap());

    // 阻塞方法、等待子线程发送完毕
    // println!("receive {}", rx.recv().unwrap());

    // 使用 for 进行循环接收
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let v = vec![
    //         String::from("h1"),
    //         String::from("hello"),
    //         String::from("thread"),
    //     ];

    //     for item in v {
    //         tx.send(item).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for r in rx {
    //     println!("receive from thread:{:?}", r)
    // }

    // 使用多发送者

    // let (tx, rx) = mpsc::channel();

    // 需要发送者进行克隆
    // let t1 = tx.clone();

    // thread::spawn(move || {
    //     tx.send(String::from("message from one thread")).unwrap();
    // });

    // thread::spawn(move || {
    //     t1.send(String::from("message from two thread")).unwrap();
    // });

    // for receive in rx {
    //     println!("receive message is: {:?}", receive);
    // }

    // mutex -  mutual exclision 互斥锁
    // 做好锁的作用域管理

    // let m = Mutex::new(5);

    // {
    // 作用域 1
    // let mut num1 = m.lock().unwrap();
    // *num1 = 6;
    // 自动 drop 掉 num1
    // }

    // {
    // 作用域 2
    // let mut num2 = m.lock().unwrap();
    // *num2 = 7
    // 自动 drop 掉 num2
    // }

    // println!("num is {:?}", m)

    // 多线程 互斥锁

    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let clone_counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = clone_counter.lock().unwrap();
    //         *num += 1
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("main thread counter is {:?}", *counter.lock().unwrap());

    // 读写锁 RwLock
    // 同一时间可以允许多个读、但是只允许一个写
    // 读和写不允许同时存在

    let lock = RwLock::new(5);

    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut x = lock.write().unwrap();
        *x += 1;

        println!("x is {:?}", x)
    }
}
