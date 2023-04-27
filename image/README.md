## Compile

```bash
$ cd xd

// build release
$ cargo build --release
```

If you are using Rust for the first time and the progress bar stucks at the first step like this: 

```bash
    Updating crates.io index
       Fetch [==================>                                    ]  34.94%
```


and then run `cargo build --release`. The Rust compile process will take pretty much time, which involves downloading and compiling the depended packages. It took me `4m 38s` to finish.

```bash
 Finished release [optimized] target(s) in 4m 38s
```

## Available features

**Colored** output with **length**-specificed limitation.

## Usage

the binary is in the `target/release`.

![help](/image/usage-help.png)

![xd](/image/usage-xd.png)
