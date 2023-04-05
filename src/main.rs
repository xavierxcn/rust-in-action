// #[allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    let status = StatusMessage::Ok;
    println!("Status of sat {:?}: {:?}", sat_id.id, status);
    sat_id
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}

type Message = String;

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: MailBox,
}

struct GroundStation;

fn main() {

}