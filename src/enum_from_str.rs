/// Creates an enum with the given name and empty variants.
/// Automatically implements FromStr to parse it easily and Display to print it easily.
/// Also derives Clone, Debug, PartialEq, Eq and Hash for this enum.
macro_rules! enum_from_str {
    ($(#[$meta:meta])* $name:ident => $( $x:ident ),+ ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $x,
            )+
        }

        impl std::str::FromStr for $name {
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

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match &self {
                    $(
                        $name::$x => write!(f, stringify!($x)),
                    )+
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn test_enum_from_str() {
        enum_from_str!{TestEnum => One, Two, Three};

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
