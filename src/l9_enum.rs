#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

pub fn exmpl_1() {
    let ip_v4 = IpAddKind::V4;
    let ip_v6 = IpAddKind::V6;

    println!("{ip_v4:?}, {ip_v6:?}");
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

pub fn exmpl_2() {
    let ip_v4 = IpAddr::V4(String::from("127.0.0.1"));
    let ip_v6 = IpAddr::V6(String::from("::1"));

    println!("{ip_v4:?}, {ip_v6:?}");
}