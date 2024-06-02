use std::net::{Ipv4Addr};
use wasm_bindgen::prelude::*;
use rand::prelude::*;


#[wasm_bindgen]
pub fn generate_ipv4(ip_type:&str) -> String {
    match ip_type {
        // "global" => generate_public_ipv4_address(), // todo ip n'est pas encore mature, il faudra retenter plus tard
        "random" => generate_random_ipv4_address(),
        "shared" => generate_shared_ipv4_address(),
        "private" => generate_private_ipv4_address(),
        "reserved" => generate_reserved_ipv4_address(),
        "link-local" => generate_link_local_ipv4_address(),
        "benchmarking" => generate_benchmarking_ipv4_address(),
        "multicast" => generate_multicast_ipv4_address(),
        "documentation" => generate_documentation_ipv4_address(),
        _ => "Invalid type received. Available types : Public, Private or Reserved.".to_string(),
    }
}

/// Generates a completely random shared IPv4 address (from 100.64.0.0/10 ).
/// Check [RFC 6598](https://datatracker.ietf.org/doc/html/rfc6598)
fn generate_shared_ipv4_address() -> String {
    generate_ipv4_in_cidr("100.64.0.0/10")
}


/// Generates a completely random private IPv4 address.
/// Check [RFC 1918](https://datatracker.ietf.org/doc/html/rfc1918)
fn generate_private_ipv4_address() -> String {
    let mut rng = thread_rng();
    let cidr_spaces = ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16"];
    generate_ipv4_in_cidr(cidr_spaces.choose(&mut rng).unwrap())
}

/// Generates a completely random reserved IPv4 address.
    /// Check [RFC 1112](https://datatracker.ietf.org/doc/html/rfc1112)
fn generate_reserved_ipv4_address() -> String {
    generate_ipv4_in_cidr("240.0.0.0/4")
}


/// Generates a completely random link-local IPv4 address.
/// Check [RFC 3927](https://datatracker.ietf.org/doc/html/rfc3927)
fn generate_link_local_ipv4_address() -> String {
    generate_ipv4_in_cidr("169.254.0.0/16")
}

/// Generates a completely random benchmarking IPv4 address.
/// Check [RFC 2544](https://datatracker.ietf.org/doc/html/rfc2544)
fn generate_benchmarking_ipv4_address() -> String {
    generate_ipv4_in_cidr("198.18.0.0/15")
}

/// Generates a completely random multicast IPv4 address.
/// Check [RFC 5771](https://datatracker.ietf.org/doc/html/rfc5771)
fn generate_multicast_ipv4_address() -> String {
    generate_ipv4_in_cidr("224.0.0.0/4")
}

/// Generates a completely random multicast IPv4 address.
/// Check [RFC 5737](https://datatracker.ietf.org/doc/html/rfc5737)
fn generate_documentation_ipv4_address() -> String {
    let mut rng = thread_rng();
    let cidr_spaces = ["192.0.2.0/24", "198.51.100.0/24", "203.0.113.0/24"];
    generate_ipv4_in_cidr(cidr_spaces.choose(&mut rng).unwrap())
}

/// Generates a completely random IPv4 address.
///
/// # Returns
///
/// A string containing the generated IPv4 address.
///
/// # Example
///
/// ```
/// let ipv4_address = generate_random_ipv4_address();
/// ```
fn generate_random_ipv4_address() -> String {
    let mut rng = rand::thread_rng();
    format!("{}.{}.{}.{}", rng.gen_range(1..=255), rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(1..=254))
}


/// Generates a random IPv4 address within the specified CIDR range.
///
/// # Arguments
///
/// * `cidr` - A string slice representing the CIDR notation specifying the range of IP addresses.
///
/// # Returns
///
/// A string containing the generated IPv4 address, or `` if the CIDR notation is invalid.
///
/// # Example
///
/// ```
/// let cidr = "192.168.1.0/24";
/// let ipv4_address = generate_ipv4_in_cidr(cidr);
/// ```
#[wasm_bindgen]
pub fn generate_ipv4_in_cidr(cidr: &str) -> String {
    // Parse CIDR notation
    let cidr_parts: Vec<&str> = cidr.split('/').collect();
    if cidr_parts.len() != 2 {
        return "".to_string(); // Invalid CIDR notation
    }
    let network_addr: Ipv4Addr = cidr_parts[0].parse().unwrap();
    let subnet_mask_length: u32 = cidr_parts[1].parse().unwrap();

    // Calculate subnet mask
    let subnet_mask: u32 = (!0) << (32 - subnet_mask_length);

    // Calculate network range
    let min_ip: u32 = u32::from(network_addr.clone()) & subnet_mask;
    let max_ip: u32 = min_ip | !subnet_mask;

    // Generate random IP address within the range
    let mut rng = thread_rng();
    let random_ip: u32 = rng.gen_range(min_ip..=max_ip);

    Ipv4Addr::from(random_ip).to_string()
}

#[wasm_bindgen]
pub fn generate_ipv6(ip_type:&str) -> String {
    match ip_type {
        "benchmarking" => generate_benchmarking_ipv6_address(),
        "documentation" => generate_documentation_ipv6_address(),
        "multicast" => generate_multicast_ipv6_address(),
        "random" => generate_random_ipv6_address(),
        "unicast-link-local" => generate_unicast_link_local_ipv6_address(),
        "unique-local" => generate_unique_local_ipv6_address(),
        _ => "Invalid type received. Available types : Public, Private or Reserved.".to_string(),
    }
}


/// https://datatracker.ietf.org/doc/html/rfc5180
/// https://www.rfc-editor.org/errata_search.php?eid=1752
fn generate_benchmarking_ipv6_address()-> String{
    generate_ipv6_address("2001:2::/48")
}

/// https://datatracker.ietf.org/doc/html/rfc3849
fn generate_documentation_ipv6_address()-> String{
    generate_ipv6_address("2001:db8::/32")
}

/// https://datatracker.ietf.org/doc/html/rfc4291
fn generate_multicast_ipv6_address()-> String{
    generate_ipv6_address("ff00::/8")
}

/// completely random ipv6
fn generate_random_ipv6_address()-> String{
    generate_ipv6_address("::/0")
}

/// https://datatracker.ietf.org/doc/html/rfc4291#section-2.4
fn generate_unicast_link_local_ipv6_address()-> String{
    generate_ipv6_address("fe80::/10")
}

/// https://datatracker.ietf.org/doc/html/rfc4193
fn generate_unique_local_ipv6_address()-> String{
    generate_ipv6_address("fc00::/7")
}


#[wasm_bindgen]
pub fn generate_ipv6_address(cidr: &str) -> String {
    let parts: Vec<&str> = cidr.split("/").collect();
    let mut result: String = String::new();
    let mut binary_cidr: String = String::new();
    // - Must be a length of exactly 2 : the prefix and the borne.
    // - The borne must be a number.
    if parts.len()==2 && parts[1].parse::<u8>().unwrap_or(0)>=0 && && parts[1].parse::<u8>().unwrap_or(0)<= &&128 {
        let ipv6_parts: Vec<&str> = parts[0].split(':').collect();
        let borne:u8 = parts[1].parse::<u8>().unwrap_or(0);
        for ip_part in ipv6_parts{
            if ip_part!="" {
                for char in ip_part.chars() {
                    binary_cidr.push_str(hex_char_to_binary(char));
                }
            } else {
                binary_cidr.push_str("0000");
            }
        }
        // For a good mesure, let's concatenate 128 more 0 just in case.
        binary_cidr.push_str("000000000000000000000000000000000000000000000000000000000000");
        binary_cidr.push_str("000000000000000000000000000000000000000000000000000000000000");
        binary_cidr.push_str("00000000");

        // At this point, we should have :
        // - a S short string representing the prefix of the given cidr (ex: "1111011111111...." )
        // - a X desired length of this binary prefix (ex: 7 )
        // We just have to slice the proper prefix with the first X characters of the S string
        // In our example, the 7 first characters of "1111011111111...." which is "1111011".
        // Then we must complete it with random bits until we reach 128 characters.
        if borne < binary_cidr.len() as u8 {
            // Let's take the first X bits.
            result.push_str(&binary_cidr[0..borne as usize]);
            // Now let's add the remaining bits randomly.
            let mut rng = thread_rng();
            let binary_chars: String = (0..128-borne)
                .map(|_| {
                    let bit: u8 = rng.gen_range(0..=1);
                    (b'0' + bit) as char
                })
                .collect();
            result.push_str(&*binary_chars);
        }
        //return format!("result : {} et binary_cidr : {}", result, binary_cidr);
        //result

        match binary_to_ipv6(result) {
            Ok(val) => format!("{}", val),
            Err(err) => format!("{}", err),
        }

    } else {
        "Invalid CIDR notation".to_string()
    }
}


///
/// cast
/// 11111100001011111111000000011100001100000001000100111000100111100010011101000100111110110110100101000001001000111111011000001000
/// to ip
///
fn binary_to_ipv6(binary: String) -> Result<String, &'static str> {
    if binary.len() != 128 {
        return Err("The binary string must be exactly 128 bits long");
    }

    let mut ipv6_parts = Vec::new();

    for i in 0..8 {
        let start = i * 16;
        let end = start + 16;
        let part = &binary[start..end];
        let hex_part = u16::from_str_radix(part, 2)
            .map_err(|_| "Invalid binary string")?;
        ipv6_parts.push(format!("{:x}", hex_part));
    }

    Ok(ipv6_parts.join(":"))
}

/************************************* CONVERSION *************************************************/


/// Convert any IP in dotted decimals notation to its binary equivalent.
/// 192.168.1.1 consists of four octets:
/// - 192 (the first 8 bits)
/// - 168 (the next 8 bits)
/// - 1 (the next 8 bits)
/// - 1 (the last 8 bits)
/// Which gives us : 11000000 10101000 00000001 00000001
/// So we return 11000000101010000000000100000001
#[wasm_bindgen]
pub fn convert_dotted_decimals_to_binary(dotted_decimals_ip: &str) -> String {
    let mut result = String::new();
    let ip_parts: Vec<&str> = dotted_decimals_ip.split(".").collect();
    if ip_parts.len() == 4 {
        for ip_part in ip_parts{
            let ip_part_u8 = ip_part.parse::<u8>().unwrap_or(0);
            let string_binary = format!("{ip_part_u8:b}");
            // Fill it with 0s at the beginning (max 8 characters)
            let padded_binary_string = format!("{:0>8}", string_binary);
            result += &*padded_binary_string;
        }
    }
    return result;
}

/// Converts a hexadecimal number to its binary representation.
/// Examples of equivalences
///  0.0.0.0 <=> 0 <=>
///  255.255.255.255 <=> FFFFFFFF <=>
#[wasm_bindgen]
pub fn convert_hexadecimals_to_binary(hexadecimals_ip: &str) -> String {
    // Steps :
    // 1° Split the string in 4 parts of 2 chars.
    // 2° Convert all these parts in their binary equivalents (each one must always be exactly
    //    8 chars long).
    // 3° Concatenate these parts to a new string.
    let mut binary_result = String::new();
    if hexadecimals_ip.len()==8 {
        let parts: Vec<&str> = hexadecimals_ip
            .as_bytes()
            .chunks(2)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect();
        for part in parts {
            let hexadecimal_part = u8::from_str_radix(part, 16).unwrap_or(0);
            let binary_part = format!("{:08b}", hexadecimal_part);
            binary_result.push_str(&*binary_part);
        }
    }
    return binary_result;
}


#[wasm_bindgen]
pub fn convert_decimals_to_binary(decimals_ip: &str) -> String {
    let integer_value = u32::from_str_radix(&*decimals_ip, 10).expect("Invalid binary string");
    format!("{:0width$b}", integer_value, width = 32)
}

#[wasm_bindgen]
pub fn convert_octals_to_binary(octals_ip: &str) -> String {
    let mut ip_binary = String::new();
    let segments: Vec<&str> = octals_ip.as_bytes().chunks(3)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    for (_i, segment) in segments.iter().enumerate() {
        let decimal_value = u8::from_str_radix(segment, 8).unwrap();
        let val = format!("{:08b}", decimal_value);
        ip_binary.push_str(&*val);
    }
    ip_binary
}

/// Converts any 00000000000000000000000000000000 (0.0.0.0) binary to 00000000 hex.
/// Converts any 11000000101010000000000100000001 (192.168.1.1) binary to C0A80101 hex.
/// Converts any 11111111111111111111111111111111 (255.255.255.255) binary to FFFFFFFF hex.
#[wasm_bindgen]
pub fn convert_ip_binary_to_hexadecimals(binary_ip: String) -> String {
    let mut result = String::new();
    // 1° Break the binary string in 4 different parts of 8 chars long each.
    let segments: Vec<&str> = binary_ip.as_bytes().chunks(8)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    for (_i, segment) in segments.iter().enumerate() {
        let decimal_value = u8::from_str_radix(segment, 2).unwrap();
        let val = format!("{:02X}", decimal_value);
        result.push_str(&*val);
    }
    return result
}



#[wasm_bindgen]
pub fn convert_ip_binary_to_decimals(binary_ip: String) -> String {
    return u32::from_str_radix(&*binary_ip, 2).expect("Invalid binary string").to_string()
}

#[wasm_bindgen]
pub fn convert_ip_binary_to_octals(binary_ip: String) -> String {
    let mut result = String::new();
    // First, let's split the binary string in its 4 sub-parts.
    let segments: Vec<&str> = binary_ip.as_bytes().chunks(8)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    for (_i, segment) in segments.iter().enumerate() {
        let decimal_value = u8::from_str_radix(segment, 2).unwrap();
        let val = format!("{:03o}", decimal_value);
        result.push_str(&*val);
    }
    result
}

#[wasm_bindgen]
pub fn convert_ip_binary_to_dotted_decimals(binary_ip: String) -> String {
    let mut result = String::new();
    // First, let's split the binary string in its 4 sub-parts.
    let segments: Vec<&str> = binary_ip.as_bytes().chunks(8)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();

    let mut count:u8 = 0;
    for (_i, segment) in segments.iter().enumerate() {
        let decimal_value = u8::from_str_radix(segment, 2).unwrap();
        let val = format!("{}", decimal_value);
        result.push_str(&*val);
        if count<3 {
            result.push_str(".");
        }
        count+=1;
    }
    result
}

#[wasm_bindgen]
pub fn convert_ip_binary_to_ipv6(binary_ip: String) -> String {
    let mut result = String::from("::ffff:");
    // First, let's split the binary string in its 4 sub-parts.
    let segments: Vec<&str> = binary_ip.as_bytes().chunks(8)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();

    let mut count:u8 = 0;
    for (i, segment) in segments.iter().enumerate() {
        let decimal_value = u8::from_str_radix(segment, 2).unwrap();
        let hex_val = format!("{:02X}", decimal_value);
        result.push_str(&*hex_val);
        if count==1 {
            result.push_str(":");
        }
        count+=1;
    }
    result.to_lowercase()
}


fn hex_char_to_binary(hex_char: char) -> &'static str {
    match hex_char {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => "",
    }
}
