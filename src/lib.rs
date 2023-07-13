use std::error::Error;
use std::io;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use uuid::Uuid;

const PACKAGE_SIZE: usize = 1024;

pub struct Client<A: ToSocketAddrs + Copy> {
    server_addr: A,
    user_id: Uuid,
    socket: UdpSocket,
    message_listener_list: Vec<Box<dyn MessageListener>>,
}

pub struct Message<'a, MT: MessageType> {
    pub data: &'a [u8],
    pub msg_type: MT,
    pub user_id: Uuid,
}

pub trait Serializable {
    fn serialize(&self) -> &[u8];
}

pub trait MessageListener {
    fn message_recived(&self, user_id: Uuid, data: &[u8]);
}

pub trait MessageType {
    fn code(&self) -> &[u8; 1];
}

pub trait Token {
    fn as_bytes(&self) -> &[u8; 500];
}

impl<A: ToSocketAddrs + Copy> Client<A> {
    pub fn new(server_addr: A, user_id: Uuid) -> io::Result<Self> {
        let socket = UdpSocket::bind("[::]:0")?;
        let message_listener_list = Vec::new();
        Ok(Self {
            server_addr,
            user_id,
            socket,
            message_listener_list,
        })
    }

    //client:send
    //data, msg_type, need_ack, user_id
    //Adapter for token sender
    //client:recive
    //data, msg_type, user_id
    //Adapter for token validation

    pub fn send<T: MessageType, S: Serializable>(
        &self,
        data: S,
        msg_type: T,
        need_ack: bool,
    ) -> io::Result<()> {
        let token: &[u8; 500] = &[244; 500];
        let user_id_bytes = self.user_id.as_bytes();
        let need_ack_bytes = &[u8::from(need_ack)];

        let message_bytes = [
            token as &[_],
            user_id_bytes,
            need_ack_bytes,
            msg_type.code(),
            data.serialize(),
        ]
        .concat();

        if message_bytes.len() > PACKAGE_SIZE {
            Err(format!(
                "Message of size {} is longer then allowed size of {}",
                message_bytes.len(),
                PACKAGE_SIZE
            ));
        }

        self.socket.send_to(&message_bytes, self.server_addr)?;
        Ok(())
    }

    pub fn add_message_listener<MT: MessageType, ML: MessageListener>(
        message_type: MT,
        message_listener: ML,
    ) -> io::Result<()> {
        unimplemented!("Read not implementet yet");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let user_id = Uuid::nil();
        let client = Client::new("", user_id);
        assert_eq!(client.is_ok(), true)
    }
}
