// -- Header
// 3 -> version (numbers)
// 3 -> typeID (numbers)
// ----
// 1 -> lengthTypeId

//D2FE28
//TODO: Padding
//11010010111111100010100
//VVVTTTAAAAABBBBBCCCCC

fn main() {
    let code = hex_to_binary("220D700071F39F9C6BC92D4A6713C737B3E98783004AC0169B4B99F93CFC31AC4D8A4BB89E9D654D216B80131DC0050B20043E27C1F83240086C468A311CC0188DB0BA12B00719221D3F7AF776DC5DE635094A7D2370082795A52911791ECB7EDA9CFD634BDED14030047C01498EE203931BF7256189A593005E116802D34673999A3A805126EB2B5BEEBB823CB561E9F2165492CE00E6918C011926CA005465B0BB2D85D700B675DA72DD7E9DBE377D62B27698F0D4BAD100735276B4B93C0FF002FF359F3BCFF0DC802ACC002CE3546B92FCB7590C380210523E180233FD21D0040001098ED076108002110960D45F988EB14D9D9802F232A32E802F2FDBEBA7D3B3B7FB06320132B0037700043224C5D8F2000844558C704A6FEAA800D2CFE27B921CA872003A90C6214D62DA8AA9009CF600B8803B10E144741006A1C47F85D29DCF7C9C40132680213037284B3D488640A1008A314BC3D86D9AB6492637D331003E79300012F9BDE8560F1009B32B09EC7FC0151006A0EC6082A0008744287511CC0269810987789132AC600BD802C00087C1D88D05C001088BF1BE284D298005FB1366B353798689D8A84D5194C017D005647181A931895D588E7736C6A5008200F0B802909F97B35897CFCBD9AC4A26DD880259A0037E49861F4E4349A6005CFAD180333E95281338A930EA400824981CC8A2804523AA6F5B3691CF5425B05B3D9AF8DD400F9EDA1100789800D2CBD30E32F4C3ACF52F9FF64326009D802733197392438BF22C52D5AD2D8524034E800C8B202F604008602A6CC00940256C008A9601FF8400D100240062F50038400970034003CE600C70C00F600760C00B98C563FB37CE4BD1BFA769839802F400F8C9CA79429B96E0A93FAE4A5F32201428401A8F508A1B0002131723B43400043618C2089E40143CBA748B3CE01C893C8904F4E1B2D300527AB63DA0091253929E42A53929E420");
    let (_, count) = decode(&code, 0);
    println!("count: {count}");
}

fn hex_to_binary(binary: &str) -> String {
    let mut hex = String::new();
    for char in binary.chars() {
        match char {
            '0' => hex.push_str("0000"),
            '1' => hex.push_str("0001"),
            '2' => hex.push_str("0010"),
            '3' => hex.push_str("0011"),
            '4' => hex.push_str("0100"),
            '5' => hex.push_str("0101"),
            '6' => hex.push_str("0110"),
            '7' => hex.push_str("0111"),
            '8' => hex.push_str("1000"),
            '9' => hex.push_str("1001"),
            'A' => hex.push_str("1010"),
            'B' => hex.push_str("1011"),
            'C' => hex.push_str("1100"),
            'D' => hex.push_str("1101"),
            'E' => hex.push_str("1110"),
            'F' => hex.push_str("1111"),
            _ => panic!("wrong character"),
        }
    }
    hex
}

fn decode(code: &str, start: usize) -> (usize, u64) {
    println!("code: {code}");

    let mut counter = u64::from_str_radix(&code[0..3], 2).unwrap();
    println!("counter: {counter}");

    let mut mode = String::new();

    match &code[3..6] {
        "100" => {
            let mut start = 6;
            let mut literal = String::new();
            println!("code: {}", &code[start..]);

            loop {
                literal.push_str(&code[start + 1..start + 5]);
                println!("pushed: {}", &code[start..start + 5]);

                if &code[start..start + 1] == "0" {
                    break;
                }
                start += 5;
            }
            start += 5;
            let literal = u64::from_str_radix(&literal, 2).unwrap();
            println!("literal_dec: {}", literal);
            println!("start: {}", start);

            (start, literal)
        }

        _ => match &code[6..7] {
            "0" => {
                println!("15 bits");

                let size = usize::from_str_radix(&code[7..22], 2).unwrap();
                println!("size: {}", size);

                let mut start = 21;
                let end = start + size;
                println!("end: {}", end);

                while start < end {
                    if !&code[start + 1..].contains('1') {
                        break;
                    }
                    let (start_decoded, counter_decoded) = decode(&code[start + 1..], start);
                    start += start_decoded;
                    counter += counter_decoded;
                    println!("start: {}", start);
                }
                (start + 1, counter)
            }
            "1" => {
                println!("11 bits");

                let size = usize::from_str_radix(&code[7..18], 2).unwrap();
                println!("size: {}", size);

                let mut start = 17;

                for _i in 0..size {
                    if !&code[start + 1..].contains('1') {
                        break;
                    }
                    let (start_decoded, counter_decoded) = decode(&code[start + 1..], start);
                    start += start_decoded;
                    counter += counter_decoded;
                    println!("start: {}", start);
                }
                (start + 1, counter)
            }
            _ => panic!("wrong character"),
        },
    }
}

fn decode_sub(code: &str, mut start: usize, mut counter: u64) -> (usize, u64) {
    match &code[6..7] {
        "0" => {
            println!("15 bits");

            let size = usize::from_str_radix(&code[7..22], 2).unwrap();
            println!("size: {}", size);

            let mut start = 21;
            let end = start + size;
            println!("end: {}", end);

            while start < end {
                if !&code[start + 1..].contains('1') {
                    break;
                }
                let (start_decoded, counter_decoded) = decode(&code[start + 1..], start);
                start += start_decoded;
                counter += counter_decoded;
                println!("start: {}", start);
            }
            return (start + 1, counter);
        }
        "1" => {
            println!("11 bits");

            let size = usize::from_str_radix(&code[7..18], 2).unwrap();
            println!("size: {}", size);

            let mut start = 17;

            for _i in 0..size {
                if !&code[start + 1..].contains('1') {
                    break;
                }
                let (start_decoded, counter_decoded) = decode(&code[start + 1..], start);
                start += start_decoded;
                counter += counter_decoded;
                println!("start: {}", start);
            }
            return (start + 1, counter);
        }
        _ => panic!("wrong character"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, hex_to_binary};

    #[test]
    fn get_binary() {
        assert_eq!("110100101111111000101000", hex_to_binary("D2FE28"));
        assert_eq!(
            "00111000000000000110111101000101001010010001001000000000",
            hex_to_binary("38006F45291200")
        );
    }

    #[test]
    fn decode_examples() {
        let code = hex_to_binary("EE00D40C823060");
        let (_, counter) = decode(&code, 0);
        assert_eq!(counter, 14);
        println!("------");
        let code = hex_to_binary("38006F45291200");
        let (_, counter) = decode(&code, 0);
        assert_eq!(counter, 9);
        println!("------");
        let code = hex_to_binary("8A004A801A8002F478");
        let (_, counter) = decode(&code, 0);
        assert_eq!(counter, 16);
        println!("------");

        let code = hex_to_binary("620080001611562C8802118E34");
        let (_, counter) = decode(&code, 0);
        assert_eq!(counter, 12);
        println!("------");

        let code = hex_to_binary("C0015000016115A2E0802F182340");
        let (_, counter) = decode(&code, 0);
        assert_eq!(counter, 23);
        println!("------");

        let code = hex_to_binary("A0016C880162017C3686B18A3D4780");
        let (_, counter) = decode(&code, 0);
        assert_eq!(counter, 31);
        println!("------");
    }
}
