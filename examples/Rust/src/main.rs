use std::ops::Mul;

fn encode(s: &str) -> String {
    let mut ret: String = String::new();

    for c in s.chars() {
        let mut char_code: u32 = decimal_to_octal(c as u32);

        if char_code == 0 {
            ret.push_str("\u{16D7}")
        } else {
            while char_code > 0 {
                if char_code >= 200 {
                    ret.push_str("\u{16D4}");
                    char_code -= 200;
                } else if char_code >= 50 {
                    ret.push_str("\u{16E4}");
                    char_code -= 50;
                } else if char_code >= 10 {
                    ret.push_str("\u{16B8}");
                    char_code -= 10;
                } else if char_code >= 5 {
                    ret.push_str("\u{16BB}");
                    char_code -= 5;
                } else if char_code >= 1 {
                    ret.push_str("\u{16A4}");
                    char_code -= 1;
                }
            }
        }

        ret.push_str("\u{26B8}");
    }

    return ret;
}

fn decode(s: &str) -> String {
    let mut ret: String = String::new();
    let mut g_char_code: u32 = 0;

    for c in s.chars() {
        
        let mut l_char_code: u32 = g_char_code;

        if c.to_string() == "\u{16D4}" { l_char_code += 200 };
        if c.to_string() == "\u{16E4}" { l_char_code += 50 };
        if c.to_string() == "\u{16B8}" { l_char_code += 10 };
        if c.to_string() == "\u{16BB}" { l_char_code += 5 };
        if c.to_string() == "\u{16A4}" { l_char_code += 1 };

        if c.to_string() == "\u{26B8}" {
            ret.push(char::from_u32(octal_to_decimal(g_char_code)).unwrap());
            l_char_code = 0;
        }

        g_char_code = l_char_code;
    }

    return ret;
}

fn octal_to_decimal(oct: u32) -> u32 {
    let v = (oct.to_string()).chars().collect::<Vec<char>>();
    let mut v2: Vec<u32> = Vec::new();
    let mut ret: u32 = 0;

    for i in 0..v.len() {
        let n: u32 = parse_int(v[i].to_string().as_str()).mul((8 as u32).pow((v.len()-1-i).try_into().unwrap()));
        v2.insert(i, n);
    }

    for i in v2.iter() {
        ret += i;
    }

    return ret;
}

fn decimal_to_octal(dec: u32) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    let mut quot: u32 = dec;

    while quot > 0 {
        let n: u32 = quot.to_owned() % 8;
        v.insert(v.len(), n);
        quot = quot/8;
    }

    v = vec_reverse(v);

    let mut ret: String = String::new();

    for i in v.iter() {
        ret.push_str(i.to_string().as_str());
    }

    return parse_int(ret.as_str());
}

fn parse_int(s: &str) -> u32 {
    return s.parse::<u32>().unwrap();
}

fn vec_reverse(v: Vec<u32>) -> Vec<u32> {
    let mut new_vec: Vec<u32> = Vec::new();
    for i in 0..v.len() {
        new_vec.insert(i, v[v.len()-i-1].clone());
    }
    return new_vec;
}

fn main() {

    let encoded: String = encode("Hello World!");
    let decoded: String = decode(&encoded);
    
    println!("{}", encoded);
    println!("{}", decoded);
}
