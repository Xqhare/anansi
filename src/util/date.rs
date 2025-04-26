// The rationale behind using a struct is memory efficiency.
// Storing the same data as the string it is read as takes up 10 bytes.
// This struct takes up 4 bytes!

use std::str::FromStr;

use super::deserialise_date;

/// Represents a date in the format `YYYY-MM-DD`.
///
/// While probably never constructed directly, it can be by using the `Date::new` function.
/// Consider using the `Date::from` function instead. It takes any string (correctly formatted) as an argument.
///
/// Date considers the date '0000-00-00' to be invalid.
#[derive(Debug, Clone, Copy, PartialEq)]
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
//                        Parse implementation
// ---------------------------------------------------------------
impl FromStr for Date {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(deserialise_date(s))
    }
}

// ---------------------------------------------------------------
//                        General implementation
// ---------------------------------------------------------------
impl Date {
    /// Creates a new date.
    ///
    /// Consider using the `Date::from<String>` function instead.
    ///
    /// # Arguments
    /// 
    /// * `year` - The year of the date.
    /// * `month` - The month of the date.
    /// * `day` - The day of the date.
    ///
    /// # Example
    /// ```
    /// use anansi::Date;
    /// 
    /// let date = anansi::Date::new(2022, 1, 1);
    /// assert_eq!(date, Date::from("2022-01-01"));
    /// ```
    pub fn new(year: u16, month: u8, day: u8) -> Date {
        Date {
            year,
            month,
            day,
        }
    }

    /// Formats a date into the format `YYYY-MM-DD`
    ///
    /// # Example
    /// ```
    /// use anansi::Date;
    /// 
    /// let date = anansi::Date::new(2022, 1, 1);
    /// assert_eq!(date.format_date(), "2022-01-01");
    /// let date = anansi::Date::default();
    /// assert_eq!(date.format_date(), "");
    /// ```
    pub fn format_date(&self) -> String {
        if self.year != 0 {
            format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
        } else {
            String::new()
        }
    }

    /// Returns `true` if the date is set.
    ///
    /// # Example
    /// ```
    /// use anansi::Date;
    /// 
    /// let date = anansi::Date::new(2022, 1, 1);
    /// assert_eq!(date.is_set(), true);
    /// let date = anansi::Date::default();
    /// assert_eq!(date.is_set(), false);
    /// ```
    pub fn is_set(&self) -> bool {
        self.year != 0 && self.month != 0 && self.day != 0
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

impl<S: AsRef<str>> From<S> for Date {
    fn from(input: S) -> Self {
        deserialise_date(input)
    }
}
