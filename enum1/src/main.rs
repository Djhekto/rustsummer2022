enum IpAddrKind {
    V4,
    V6,
}
struct _IpAddr {
    _kind: IpAddrKind,
    _address: String,
} //hranim cam nomer i tip

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    
    let _home = _IpAddr {
        _kind: IpAddrKind::V4,
        _address: String::from("127.0.0.1"),
    };

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
}

fn route(_ip_kind: IpAddrKind) {} //если ф написать так, то оба вызова сверху работают

//bez struct
/*
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
*/
//chto ugodno pod enum
/*
struct Ipv4Addr {
    // --snip--
}
struct Ipv6Addr {
    // --snip--
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
} 
*/