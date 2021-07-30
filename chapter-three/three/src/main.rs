use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

// cargo run and curl 127.0.0.1:8080 -d hello
fn main() {
    // TcpListener 用于舰艇TCP连接
    // bind()返回Result<T, E>
    // 执行成功时返回一个TcpListener实例，并监听传入的ip:port
    // 有可能执行失败，返回对应的错误，例如监听1024以下端口时，非管理员用户会失败
    // unwrap() 在返回错误时会panic终止程序，正常时返回TcpListener类型实例
    // 这里不再进行自定义的错误处理
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // 返回监听器的连接的迭代器，持续读取客户端发送来的消息
    for stream in listener.incoming() {
        // 获取TcpStream
        let stream = stream.unwrap();
        // 打印客户端请求信息，并返回响应 "Hi~"
        handle_connnection(stream)
    }
}

fn handle_connnection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // 读取信息到固定长度的buffer中
    stream.read(&mut buffer).unwrap();

    // 构造响应
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        "Hi~".len(),
        "Hi~",
    );

    // 打印请求信息
    // from_utf8_lossy() 只转换UTF-8编码，非UTF-8被当作无效序列，统一使用�代替
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 回血请求到流
    stream.write(response.as_bytes()).unwrap();
    // 把所有字节刷新到连接中
    stream.flush().unwrap();
}
