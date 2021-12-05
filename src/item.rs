use num_bigint::BigUint;

pub trait Item {
    fn cost(&self) -> BigUint;
    fn level(&self) -> BigUint;
    fn adjust_cost(&mut self);
    fn increase_level(&mut self);
}

#[macro_export]
macro_rules! item_struct {
    ($name:ident { { $($var:ident: $type:ty,)* } $($body:tt)*}) => {
        #[derive(Clone, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            pub cost: BigUint,
            pub level: BigUint,
            $(pub $var: $type,)*
        }

        impl $crate::item::Item for $name {
            fn cost(&self) -> BigUint {
                self.cost.clone()
            }

            fn level(&self) -> BigUint {
                self.level.clone()
            }

            fn increase_level(&mut self) {
                self.level = &self.level + 1u32;
            }

            $($body)*
        }
    }
}
