extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::motors::*;
use ev3dev_lang_rust::sensors::*;
use line_follower::pid::Pid;
use std::io::BufReader;
use std::thread::sleep;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::time::Duration;

fn main() -> Ev3Result<()> {
    println!("Hello, world!");

    let listener = TcpListener::bind("192.168.0.9:7878").unwrap();
    listener.set_nonblocking(true).unwrap();

    let left = MediumMotor::get(MotorPort::OutA)?;
    left.reset()?;
    left.set_polarity(MediumMotor::POLARITY_INVERSED)?;
    left.run_direct()?;
    
    let right = MediumMotor::get(MotorPort::OutD)?;
    right.reset()?;
    right.run_direct()?;

    let color_sensor = ColorSensor::get(SensorPort::In1)?;
    color_sensor.set_mode_col_reflect()?;

    let mut pid = Pid::new(30.0, 1.0, 0.0, 0.02);

    loop {
        if let Ok((stream, _)) = listener.accept() {
            match handle_connection(stream) {
                Request::Pid(p) => pid = p,
                Request::Stop => break,
                Request::Other => (),
            }
        };

        let color = color_sensor.get_color()?;
        let u = pid.update(color as f32);
        dbg!(u);
        
        left.set_duty_cycle_sp((100 + u as i32).clamp(0, 100))?;
        right.set_duty_cycle_sp((100 - u as i32).clamp(0, 100))?;

        sleep(Duration::from_millis(3));
    }

    left.reset()?;
    right.reset()?;
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Request {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\nuwu";
    stream.write(response.as_bytes()).unwrap();

    let path = &http_request[0].split_whitespace().nth(1).unwrap()[1..];
    if path.contains("stop") {
        return Request::Stop;
    }
    if path.contains("favicon") {
        return Request::Other;
    }
        
    let path = path
        .split(",")
        .map(|x| x.parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    Request::Pid(Pid::new(path[0], path[1], path[2], path[3]))
}

enum Request {
    Pid(Pid),
    Stop,
    Other,
}
