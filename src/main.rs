#![allow(warnings)]

use std::net::IpAddr;

// enum type variables
// definition of the enum type
enum IpAddressKind {
    V4(String), 
    V6(String)
}

struct ipAddress {
    kind : IpAddressKind,
    address : String,
}
fn main() {
  // enum examples
  let four : IpAddressKind = IpAddressKind::V4(String::from("127.0.0.1"));
  let six : IpAddressKind = IpAddressKind::V6(String::from("::1"));

  route(four);
  route(six);

  let home : ipAddress = ipAddress {
    kind : IpAddressKind::V4(String::from("127.0.0.1")),
    address : String::from("127.0.0.1"),
  };

  let loopBack : ipAddress = ipAddress {
    kind : IpAddressKind::V6(String::from("::1")),
    address : String::from("::1"),
  };
}

fn route(ip_kind: IpAddressKind) {
  // function body  
}
