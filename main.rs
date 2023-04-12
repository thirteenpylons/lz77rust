#[derive(Debug)]
struct Match {
    offset: usize,
    length: usize,
}

fn find_longest_match(buffer: &[u8], lookahead: &[u8]) -> Match {
    let mut best_match = Match { offset: 0, length: 0 };

    for i in 0..buffer.len() {
        let mut j = 0;
        while j < lookahead.len() && i + j < buffer.len() && buffer[i + j] == lookahead[j] {
            j += 1;
        }

        if j > best_match.length {
            best_match.offset = i;
            best_match.length = j;
        }
    }

    best_match
}

fn lz77_encode(data: &[u8]) -> Vec<(usize, usize, u8)> {
    let mut result = Vec::new();
    let mut buffer = [0; 4096];
    let mut buffer_pos = 0;

    for i in 0..data.len() {
        let lookahead = &data[i..];
        let Match { offset, length } = find_longest_match(&buffer, lookahead);

        if length == 0 {
            let next = data[i];
            result.push((0, 0, next));
            buffer[buffer_pos] = next;
            buffer_pos = (buffer_pos + 1) % buffer.len();
        } else {
            let next = lookahead[length];
            result.push((offset, length, next));
            for j in 0..length + 1 {
                buffer[buffer_pos] = lookahead[j];
                buffer_pos = (buffer_pos + 1) % buffer.len();
            }
        }
    }

    result
}

fn lz77_decode(data: &[(usize, usize, u8)]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut buffer = [0; 4096];
    let mut buffer_pos = 0;

    for &(offset, length, next) in data {
        if length == 0 {
            result.push(next);
            buffer[buffer_pos] = next;
            buffer_pos = (buffer_pos + 1) % buffer.len();
        } else {
            for j in 0..length + 1 {
                let index = (buffer_pos + offset + j) % buffer.len();
                let c = buffer[index];
                result.push(c);
                buffer[buffer_pos] = c;
                buffer_pos = (buffer_pos + 1) % buffer.len();
            }
        }
    }

    result
}

fn main() {
    let original = b"abracadabra";
    let encoded = lz77_encode(original);
    let decoded = lz77_decode(&encoded);

    println!("Original: {:?}", original);
    println!("Encoded: {:?}", encoded);
    println!("Decoded: {:?}", decoded);
}
