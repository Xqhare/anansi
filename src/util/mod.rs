mod date;
mod task_list;
mod test;

pub use date::*;
pub use task_list::*;

pub fn deserialise_date<S: AsRef<str>>(input: S) -> Date {
    let split_check = input.as_ref().split('-').collect::<Vec<&str>>();
    if split_check.len() == 3 && split_check[0].len() == 4 && split_check[1].len() == 2 && split_check[2].len() == 2 {
        let year = split_check[0].parse::<u16>();
        let month = split_check[1].parse::<u8>();
        let day = split_check[2].parse::<u8>();
        if year.is_ok() && month.is_ok() && day.is_ok() {
            if month.as_ref().unwrap() < &13 && day.as_ref().unwrap() < &32 {
                Date::new(year.unwrap(), month.unwrap(), day.unwrap())
            } else {
                Date::default()
            }
        } else {
            Date::default()
        }
    } else {
        Date::default()
    }
}

pub fn is_ascii_whitespace(byte_value: u8) -> bool {
    byte_value == 0x09 || byte_value == 0x0A || byte_value == 0x0B || byte_value == 0x0C || byte_value == 0x0D || byte_value == 0x20
}

pub fn is_ascii_digit(byte_value: u8) -> bool {
    byte_value >= 0x30 && byte_value <= 0x39
}

/// Returns `(`is_newline`, `new_index`)`
/// 
/// - `new_index` is the index after the newline characters OR the current index if it is not a newline
/// - `is_newline` is true if the current index is a newline (`\r\n`, `\r`, `\n`)
pub fn is_newline(index: usize, contents: &Vec<u8>) -> (bool, usize) {
    if index >= contents.len() {
        return (false, index);
    }

    if contents[index] == b'\r' {
        if index + 1 < contents.len() && contents[index + 1] == b'\n' {
            return (true, index + 2);
        } else {
            return (true, index + 1);
        }
    } else {
        if contents[index] == b'\n' {
            return (true, index + 1);
        } else {
            return (false, index);
        }
    } 
}
