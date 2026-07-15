
#[macro_export]
macro_rules! assert_ok {
    ( $ctx:ident [ $( $args:literal ),+ ] if $bind:ident @ ( $guard:expr ) ) => {
        assert!(matches!( $ctx::execute_from([ $( $args ),* ]), Ok($bind) if $guard ))
    };

    ( $ctx:ident [ $( $args:literal ),+ ] ) => {
        assert!( $ctx::execute_from([ $( $args ),* ]).is_ok())
    };
}

#[macro_export]
macro_rules! assert_clap_err {
    ( $ctx:ident [ $( $args:literal ),+ ] ) => {
        assert!(
            matches!(
                $ctx::execute_from([ $( $args ),* ]),
                Err(OvationError::ClapError(_))
            )
        )
    };
}

#[macro_export]
macro_rules! assert_terminal {
    ( $ctx:ident [ $( $args:literal ),+ ] ) => {
        assert!(
            matches!(
                $ctx::execute_from([ $( $args ),* ]),
                Err(OvationError::ClapTerminal(_))
            )
        )
    };
}

#[macro_export]
macro_rules! assert_command_err {
    ( $ctx:ident [ $( $args:literal ),+ ] ) => {
        assert!(
            matches!(
                $ctx::execute_from([ $( $args ),* ]),
                Err(OvationError::CommandError(_))
            )
        )
    };
}

