use std::env::current_exe;

/// Kind and the bytes that represent a single message
pub type Package = (u16, Vec<u8>);
/// Bytes that represent one. multiples or partial messages
pub type RawBytes = Vec<u8>;

/// This class buffer incoming RawBytes and chunk into Package
#[derive(Debug)]
pub struct PackageBuffer {
    kind: Option<u16>,
    size: Option<usize>,
    buffer: Vec<u8>,
}

impl PackageBuffer {
    pub fn new() -> Self {
        PackageBuffer {
            kind: None,
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
            }

            None => buffer,
        }
    }

    fn get_frame(&mut self) -> Option<Package> {
        let kind = match self.kind {
            None if self.buffer.len() >= 2 => {
                let bytes = [self.buffer[0], self.buffer[1]];

                self.buffer.drain(0..2);

                let kind = u16::from_be_bytes(bytes);

                self.kind = Some(kind);
                kind
            }
            None => return None,
            Some(kind) => kind,
        };

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
            self.kind = None;
            self.size = None;
            Some((kind, bytes))
        } else {
            return None;
        }
    }

    pub fn pack((kind, frame): Package) -> RawBytes {
        let mut raw_bytes = vec![];
        raw_bytes.extend(PackageBuffer::encode_kind(kind));
        raw_bytes.extend(PackageBuffer::encode_len(frame.len()));
        raw_bytes.extend(frame);
        raw_bytes
    }

    pub fn encode_len(len: usize) -> RawBytes {
        (len as u32).to_be_bytes().to_vec()
    }

    pub fn encode_kind(kind: u16) -> RawBytes {
        kind.to_be_bytes().to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_protocol() {
        let mut package_buffer = PackageBuffer::new();

        let msg_1: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let msg_1_kind_bytes = PackageBuffer::encode_kind(38);
        let msg_1_len_bytes = PackageBuffer::encode_len(msg_1.len());

        assert!(package_buffer
            .push(msg_1_kind_bytes[0..1].to_vec())
            .is_empty());
        assert!(package_buffer
            .push(msg_1_kind_bytes[1..2].to_vec())
            .is_empty());
        assert!(package_buffer
            .push(msg_1_len_bytes[0..1].to_vec())
            .is_empty());
        assert!(package_buffer
            .push(msg_1_len_bytes[1..4].to_vec())
            .is_empty());
        assert!(package_buffer.push(msg_1[0..9].to_vec()).is_empty());
        assert!(package_buffer.push(msg_1[9..13].to_vec()).is_empty());

        match package_buffer.push(msg_1[13..].to_vec()) {
            value if value.len() == 1 => {
                let messages = msg_1.to_vec();
                assert_eq!(value[0].0, 38);
                assert_eq!(value[0].1, messages);
            }
            other => {
                panic!("unexpected result {:?}", other);
            }
        }

        let msg_2_kind_len_plus_msg: [u8; 10] = [0, 3, 0, 0, 0, 4, 9, 8, 7, 6];

        match package_buffer.push(msg_2_kind_len_plus_msg.to_vec()) {
            value if value.len() == 1 => {
                assert_eq!(value[0].0, 3);
                assert_eq!(value[0].1, vec![9, 8, 7, 6]);
            }
            other => {
                panic!("unexpected result {:?}", other);
            }
        }
    }

    #[test]
    fn test_protocol_upload() {
        let mut pack_buffer = PackageBuffer::new();

        let mut buffer = vec![];

        let expected_frames = vec![
            (32, vec![0, 1, 2, 3]),
            (32, vec![4, 5, 6]),
            (32, vec![7, 8, 9, 10, 11]),
            (32, vec![12]),
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
            frames.extend(pack_buffer.push(bytes));
        }

        assert_eq!(frames, expected_frames);
    }
}
