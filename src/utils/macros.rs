#[macro_export]
macro_rules! map_struct {
    ($src:expr => $dst:ident { $($field:ident),+ $(,)? }) => {
        $dst {
            $(
                $field: $src.$field,
            )+
        }
    };
}
