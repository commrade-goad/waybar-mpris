## HOW TO IGNORE PANIC FROM CHATGPT
```
use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        // call a function that might panic
        // if the function panics, the panic will be caught here
        // and the program will continue executing
        call_something_that_might_panic();
    });

    if let Err(_) = result {
        // handle the panic here if necessary
        // or do nothing and continue executing
        println!("Caught a panic but it's not important, continuing...");
    }

    // rest of the program
}

fn call_something_that_might_panic() {
    // code that might panic
    // if it panics, the panic will propagate up to the catch_unwind block
    panic!("This panic is not important");
}
```

## HOW TO IGNORE PANIC FROM DUCKDUCKGO
```
use std::panic;

fn main() {
    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));

    let result = panic::catch_unwind(|| {
        panic!("test panic");
    });

    match result {
        Ok(res) => res,
        Err(_) => println!("caught panic!"),
    }
}```

## LIB TO USE
- <https://docs.rs/mpris/2.0.0/mpris/>

