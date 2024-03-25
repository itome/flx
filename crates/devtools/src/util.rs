#[macro_export]
macro_rules! params {
    ( $( $k:expr => $v:expr ),* ) => {
        {
            #[allow(unused_mut)]
            let mut map = Map::new();
            $( map.insert($k, $v); )*
            map
        }
    };
    ( $( $k:expr => $v:expr ),+ , ) => {
        params! { $( $k => $v ),* }
    };
}
