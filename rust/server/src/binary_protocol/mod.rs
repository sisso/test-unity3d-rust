use std::env::current_exe;

#[derive(Debug)]
pub struct ProtocolBuffer {
    size: Option<usize>,
    buffer: Vec<u8>,
}

impl ProtocolBuffer {
    pub fn new() -> Self {
        ProtocolBuffer {
            size: None,
            buffer: vec![],
        }
    }

    pub fn push(&mut self, bytes: Vec<u8>) -> Option<Vec<u8>> {
        // println!("push {:?}", bytes);
        self.buffer.extend(bytes);

        // println!("self {:?}", self);

        let size = match self.size {
            None if self.buffer.len() >= 4 => {
                let bytes = [
                    self.buffer[0],
                    self.buffer[1],
                    self.buffer[2],
                    self.buffer[3],
                ];

                self.buffer.drain(0..4);

                let size = u32::from_be_bytes(bytes) as usize;

                // println!("set size {:?}", size);

                self.size = Some(size);
                size
            }
            None => return None,
            Some(size) => size,
        };

        // println!("check buffer {:?}/{:?}", self.buffer.len(), size);

        if self.buffer.len() >= size {
            let bytes = self.buffer[0..size].to_vec();
            self.buffer.drain(0..size);
            self.size = None;
            Some(bytes)
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::binary_protocol::ProtocolBuffer;

    #[test]
    fn test_protocol() {
        let mut b = ProtocolBuffer::new();

        let msg_1: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let msg_1_len_bytes = (msg_1.len() as u32).to_be_bytes();

        assert!(b.push(msg_1_len_bytes[0..1].to_vec()).is_none());
        assert!(b.push(msg_1_len_bytes[1..4].to_vec()).is_none());
        assert!(b.push(msg_1[0..9].to_vec()).is_none());
        assert!(b.push(msg_1[9..13].to_vec()).is_none());

        let result = b.push(msg_1[13..].to_vec());
        assert!(result.is_some());
        assert_eq!(result.unwrap().to_vec(), msg_1.to_vec());

        let msg_2_len_plus_msg: [u8; 8] = [0, 0, 0, 4, 9, 8, 7, 6];

        let result = b.push(msg_2_len_plus_msg.to_vec());
        assert!(result.is_some());
        assert_eq!(result.unwrap().to_vec(), vec![9, 8, 7, 6]);
    }
}
