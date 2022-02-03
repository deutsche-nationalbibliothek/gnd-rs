#[macro_export]
macro_rules! cli_option {
    ($args: expr, $config: expr, $default: expr) => {
        $args.as_deref().or($config.as_deref()).unwrap_or($default)
    };
}

#[macro_export]
macro_rules! cli_flag {
    ($args: expr, $config: expr) => {
        $args || $config
    };
}
