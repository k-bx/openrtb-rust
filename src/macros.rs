macro_rules! enum_list_module {
    ( $enum_name:ident $variant_type:ty :
      $(
          $variant_name:ident $variant_value:expr
       ),*
    ) => {
        use std::fmt::{self, Display};
        use std::convert::TryInto;

        use serde;

        #[derive(Clone, Debug, PartialEq)]
        pub enum $enum_name {
            $($variant_name,)*
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct IntoEnumError {
            value: $variant_type,
        }

        impl Display for IntoEnumError {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(format!(stringify!($enum_name has no variant with value {}), self.value).as_str())
            }
        }

        impl TryInto<$enum_name>for $variant_type {
            type Error = IntoEnumError;

            fn try_into(self) -> Result<$enum_name, Self::Error> {
                match self {
                    $( $variant_value => Ok($enum_name::$variant_name), )*
                    _ => Err(IntoEnumError{value: self}),
                }
            }
        }

        impl Into<$variant_type> for $enum_name {
            fn into(self) -> $variant_type {
                match self {
                    $( $enum_name::$variant_name => $variant_value, )*
                }
            }
        }

        impl serde::Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let x: $variant_type = self.clone().into();
                serde::Serialize::serialize(&x, serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<$enum_name, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let result: Result<$variant_type, D::Error> = serde::Deserialize::deserialize(deserializer);
                match result {
                    Ok(x) => x.try_into().map_err(serde::de::Error::custom),
                    Err(e) => Err(e),
                }
            }
        }
    }
}
