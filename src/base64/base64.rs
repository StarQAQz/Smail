const BASE64_TABLE: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

pub fn encode(data: String) -> String {
    let data = data.as_bytes();
    let len = data.len();
    let mut index = 0;
    let mut result = String::new();
    loop {
        if index + 2 >= len {
            break;
        }
        let group = &data[index..=index + 2];
        result.push(BASE64_TABLE[(group[0] >> 2) as usize]);
        result.push(BASE64_TABLE[((group[0] & 0x3) << 4 | group[1] >> 4) as usize]);
        result.push(BASE64_TABLE[((group[1] & 0xf) << 2 | group[2] >> 6) as usize]);
        result.push(BASE64_TABLE[(group[2] & 0x3f) as usize]);
        index += 3;
    }
    let group = &data[index..len];
    let len = len % 3;
    if len == 1 {
        result.push(BASE64_TABLE[(group[0] >> 2) as usize]);
        result.push(BASE64_TABLE[((group[0] & 0x3) << 4) as usize]);
        result.push_str("==");
    } else if len == 2 {
        result.push(BASE64_TABLE[(group[0] >> 2) as usize]);
        result.push(BASE64_TABLE[((group[0] & 0x3) << 4 | group[1] >> 4) as usize]);
        result.push(BASE64_TABLE[((group[1] & 0xf) << 2) as usize]);
        result.push_str("=");
    }
    result
}
