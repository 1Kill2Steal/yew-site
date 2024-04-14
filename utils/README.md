# Additional Cargo Make Utilities

## Setting up

### Prerequisites

You need to install cargo make.

```sh
cargo install --force cargo-make
```

### Running the utility

These utilities can be set up with the same command in either `/` or `/utils`.

```sh
cargo make NUM # 'NUM' is the util number you want to make.
```

If you want to make the utility while being in the particular utility
subdirectory (i.e. under `/utils/<NUM_UTIL_NAME>`) then you just need to do:

```sh
cargo run
```

Like you usually do.

### Making a new utility

To make a new utility just refer to the structure of:
`/utils/0_general_util_structure/`.

After you have that structure you need to refer to `/Makefile.toml` and
`/utils/Makefile.toml`. There's a big commented section in both files that show
how to add your utility to the list.

If you use Netlify to host it and the utility itself is related to the website
hosting then you also need to add:

```sh
cargo make NUM # 'NUM' being the number for your new utility
```

Under `/build.sh`
