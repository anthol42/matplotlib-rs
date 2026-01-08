# Matplotlib Rust Wrapper
It contains two matplotlibs API: `plt.plot` and `fig, axes = plt.subplots`

## Example
The `matplotlib_rs` crate is design to match the python api of matplotlib. This means that if you are familiar with 
the official matplotlib api, you will feel right at home! If you aren't familiar with the api, you can check the 
[official api reference](https://matplotlib.org/stable/api/), check on forums, or ask your favorite llm to help you!

Only one thing change from the python api: default parameters. Rust currently does not support default parameters 
while python does, and matplotlib use default parameters extensively. To reach api parity, we implemented a builder 
structure to set optional parameters. If an optional parameter is not set, it will default to matplotlib's default 
value. Once all the parameters are set, you can call the `set()` method to run the command. Here is an example:
```rust
use matplotlib::pyplot as plt;

// This will do nothing as the `.set()` method is not called.
plt::plot(arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]), arr1(&[1.0, 4.0, 9.0, 16.0, 25.0]))
        .color("blue".to_string())
        .marker("o".to_string());

// This will work.
plt::plot(arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]), arr1(&[1.0, 4.0, 9.0, 16.0, 25.0]))
        .color("blue".to_string())
        .marker("o".to_string())
        .set().unwrap();

// Then, you can show the plot with
plt::show();
```

Check out the `/matplotlib/examples` for more examples, and check out the documentation to know what parameters are 
implemented. Happy coding!

## Functions supported:
- [X] `plot`:
- [X] `scatter`:
- [X] `hist`:
- [X] `imshow`:
- [X] `fillbetween`
- [X] `bar`
- [X] `axhline`
- [X] `axvline`
- [X] `text`

- [X] `axis`
- [X] `grid`
- [X] `set_xlim`
- [X] `set_ylim`
- [X] `set_xlabel`
- [X] `set_ylabel`
- [X] `set_title`
- [X] `legend`
- [X] `set_xticks`
- [X] `set_yticks`

- [X] `suptitle`
- [X] `savefig`
- [X] `tight_layout`
- [X] `subplots_adjust`