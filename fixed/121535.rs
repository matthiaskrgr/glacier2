macro_rules! the_macro {
    ( $foo:stmt ; $bar:stmt ; ) => {
        #[cfg(foo)]
        $foo
        
        #[cfg(bar)]
        $bar
    };
}

fn the_function() {
    the_macro!( (); (); );
}
