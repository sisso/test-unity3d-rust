use std::env::current_exe;

/// A bytes that represent a single message
pub type Package = Vec<u8>;
/// Bytes that represent one. multiples or partial messages
pub type RawBytes = Vec<u8>;

/// This class buffer incoming RawBytes and chunk into Package
#[derive(Debug)]
pub struct PackageBuffer {
    size: Option<usize>,
    buffer: Vec<u8>,
}

impl PackageBuffer {
    pub fn new() -> Self {
        PackageBuffer {
            size: None,
            buffer: vec![],
        }
    }

    pub fn push(&mut self, bytes: RawBytes) -> Vec<Package> {
        // println!("push {:?}", bytes);
        self.buffer.extend(bytes);
        // println!("self {:?}", self);
        self.get_frame_to_buffer(vec![])
    }

    fn get_frame_to_buffer(&mut self, mut buffer: Vec<Package>) -> Vec<Package> {
        match self.get_frame() {
            Some(frame) => {
                buffer.push(frame);
                self.get_frame_to_buffer(buffer)
            },

            None => {
                buffer
            }
        }
    }

    fn get_frame(&mut self) -> Option<Package> {

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

    pub fn pack(frame: Package) -> RawBytes {
        let mut raw_bytes = vec![];
        raw_bytes.extend(PackageBuffer::encode_len(frame.len()));
        raw_bytes.extend(frame);
        raw_bytes
    }

    pub fn encode_len(len: usize) -> RawBytes {
        (len as u32).to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::StdRng;
    use rand::{SeedableRng, Rng};

    #[test]
    fn test_protocol() {
        let mut b = PackageBuffer::new();

        let msg_1: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let msg_1_len_bytes = PackageBuffer::encode_len(msg_1.len());

        assert!(b.push(msg_1_len_bytes[0..1].to_vec()).is_empty());
        assert!(b.push(msg_1_len_bytes[1..4].to_vec()).is_empty());
        assert!(b.push(msg_1[0..9].to_vec()).is_empty());
        assert!(b.push(msg_1[9..13].to_vec()).is_empty());

        match b.push(msg_1[13..].to_vec()) {
            value if value.len() == 1 => {
                assert_eq!(value[0], msg_1.to_vec());
            },
            other => {
                panic!("unexpected result {:?}", other);
            }
        }

        let msg_2_len_plus_msg: [u8; 8] = [0, 0, 0, 4, 9, 8, 7, 6];

        let result = b.push(msg_2_len_plus_msg.to_vec());
        match b.push(msg_2_len_plus_msg.to_vec()) {
            value if value.len() == 1 => {
                assert_eq!(value[0], vec![9, 8, 7, 6]);
            },
            other => {
                panic!("unexpected result {:?}", other);
            }
        }
    }

    #[test]
    fn test_protocol_upload() {
        let mut b = PackageBuffer::new();

        let mut buffer = vec![];

        let expected_frames = vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9, 10, 11],
            vec![12],
        ];

        for frame in &expected_frames {
            buffer.extend(PackageBuffer::pack(frame.clone()));
        }

        let mut rng = rand::thread_rng();
        let mut current = 0;
        let mut frames = vec![];

        while current < buffer.len() {
            let next = (current + rng.gen_range(1, buffer.len())).min(buffer.len());
            let bytes = buffer[current..next].to_vec();
            current = next;
            frames.extend(b.push(bytes));
        }

        assert_eq!(frames, expected_frames);
    }
}
