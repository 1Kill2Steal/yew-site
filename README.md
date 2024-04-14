# Yew Site

## Hosted on Netlify

<https://1kill2steal.netlify.app>

[![Netlify Status](https://api.netlify.com/api/v1/badges/a7cccd56-43ac-49f4-abd7-38c56ba83f18/deploy-status)](https://app.netlify.com/sites/1kill2steal/deploys)

## Short Description & Instructions

It uses WASM bindgen with the Yew framework and the yew router to serve the
website. The website itself is straight-forward as it features the following
content:

- Sticky navigation bar
- Fixed at the bottom Footer
- 3 pages: Homepage, About Me, Gallery

The Gallery images are locally saved and the instructions to set the gallery images are the following:

- You upload the images in `/hutao/pics` and `/hutao/pics_uncompressed/` (You can change that directory name but you need to bother with changing up the paths in some files - namely the json utility script and the `/index.html` file)
- In the image folders you need to strictly have your image names either start with the number followed by underscore or end in underscore followed by the number followed by the extension (as that's how the regex is implemented in `/utils/x1_file_and_json_utils/`).

Tips: If you use Thunar as your file manager there's a very intuitive way to
bulk rename your files. If not then, I'm sure there's other utilities to get
that job done.

The site is fully responsive and the gallery (once set up) has a similar
structure to the way Pinterest images are displayed (just a lot more simple).

### Setting up

[Rust w/ rustup](https://www.rust-lang.org/tools/install) (rust-lang.org)
From here on you have 2 routes (using the build.sh script if you're on Linux or
running commands manually):

Running the script:

```sh
chmod +x ./build.sh # <- That's one time only.
./build.sh
```

Manual way:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli

cargo make test_1
cargo make 1
```

> [!NOTE]
> There may be more cargo make tests, in the case that there's more and this
> wasn't updated, feel free to PR it adding another command for the test or
> just open an issue.

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
