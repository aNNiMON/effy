macro_rules! struct_option {
    ($type:ident) => {
        ::lazy_static::lazy_static! {
            static ref INSTANCES: Vec<$type> = {
                let mut result = Vec::new();
                for &v in $type::VARIANTS.iter() {
                    result.push($type::new(v));
                }
                result
            };
        }

        impl crate::params::SelectableOption for $type {
            fn variants() -> &'static [Self] {
                &INSTANCES
            }

            fn as_str(&self) -> &'static str {
                self.value
            }

            fn describe_self(&self) -> &'static str {
                $type::NAME
            }
        }
    };
}

pub(crate) use struct_option;
