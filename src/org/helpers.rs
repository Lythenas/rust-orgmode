use std::collections::HashMap;

use chrono::prelude::*;
use chrono::Duration;

pub enum OrgState {
    Todo(String),
    Done(String),
}

pub enum OrgDate {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Interval {
        start: NaiveDateTime,
        end: NaiveDateTime
    },
    Repeat(NaiveDateTime, Duration),
}

pub type OrgProperties = HashMap<String, String>;

pub struct OrgContent {
    value: String,
}

/// Creates an enum with the given name and empty variants.
/// Automatically implements FromStr to parse it easily and Display to print it easily.
/// Also derives Clone, Debug, PartialEq, Eq and Hash for this enum.
macro_rules! parseable_simple_enum {
    ($name:ident, $( $x:ident ),+ ) => {
        use std::fmt;
        use std::str::FromStr;
        
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $x,  
            )+
        }

        impl FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($x) => Ok($name::$x),
                    )+
                    _ => Err(())
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match &self {
                    $(
                        $name::$x => write!(f, stringify!($x)),
                    )+
                }
            }
        }
    };
}

parseable_simple_enum!(Priority, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

impl Default for Priority {
    fn default() -> Self {
        Priority::A
    }
}
 
#[cfg(test)]
mod tests {
    
    #[test]
    fn test_parseable_simple_enum_generation() {
        parseable_simple_enum!(TestEnum, One, Two, Three);
        
        let one = TestEnum::from_str("One").unwrap();
        assert_eq!(one, TestEnum::One);
        assert_eq!(format!("{}", one), "One");

        let two = TestEnum::from_str("Two").unwrap();
        assert_eq!(two, TestEnum::Two);
        assert_eq!(format!("{}", two), "Two");

        let three = TestEnum::from_str("Three").unwrap();
        assert_eq!(three, TestEnum::Three);
        assert_eq!(format!("{}", three), "Three");
    }

}
