# Yew Site

## Short Description & Instructions

It uses WASM bindgen with the Yew framework and the yew router to serve the
website. The website itself is straight-forward as it features the following
content:

- Sticky navigation bar
- Fixed at the bottom Footer
- 3 pages: Homepage, Details, Gallery

The Gallery images are locally saved and the instructions to set the gallery images are the following:

- Change the data trunks from `/index.html` to the appropriate folders.
- Change the image format in the code in `/src/sites/gallery.rs` at the start of the function.
- Upload your images based on your set format.

Tips: If you use Thunar as your file manager there's a very intuitive way to
bulk rename your files. If not then, I'm sure there's other utilities to get
that job done.

The site is fully responsive and the gallery (once set up) has a similar
structure to the way Pinterest images are displayed (just a lot more simple).

### Setting up

[Rust w/ rustup](https://www.rust-lang.org/tools/install) (rust-lang.org)
Then run the following commands:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

Then you can build your application.
For development:

```bash
trunk serve
```

For production:

```bash
trunk build --release
trunk serve --release
```
