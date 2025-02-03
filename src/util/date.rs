// The rationale behind using a struct is memory efficiency.
// Storing the same data as the string it is read as takes up 10 bytes.
// This struct takes up 4 bytes!

use super::deserialise_date;

/// IMPORTANT: This struct assumes that no date will ever be in year 0, and that all dates be
/// between 0 and 65_535.
#[derive(Debug, Clone, Copy)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

// ---------------------------------------------------------------
//                        Default implementation
// ---------------------------------------------------------------
impl Default for Date {
    fn default() -> Self {
        Date {
            year: 0,
            month: 0,
            day: 0,
        }
    }
}

// ---------------------------------------------------------------
//                        General implementation
// ---------------------------------------------------------------
impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Date {
        Date {
            year,
            month,
            day,
        }
    }

    /// Formats a date into the format `YYYY-MM-DD`
    pub fn format_date(&self) -> String {
        if self.year != 0 {
            format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
        } else {
            String::new()
        }
    }

    pub fn is_set(&self) -> bool {
        self.year != 0
    }
}

// ---------------------------------------------------------------
//                       Display implementation
// ---------------------------------------------------------------
impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_date())
    }
}

impl From<&str> for Date {
    fn from(input: &str) -> Self {
        deserialise_date(input)
    }
}
