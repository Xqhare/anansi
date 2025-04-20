mod date;
mod test;
mod sort_by;

pub use sort_by::SortBy;
pub use date::*;

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
