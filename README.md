## Picsum Downloader

**Use Case** 
Basically every project where you might need some kind of placeholder images whether to upload while testing or for basically anything else.
You need a bunch of images of different sizes, probably of different resolutions. Well, there is a very cool application called [https://picsum.photos](https://picsum.photos) which does exactly that. This tool just helps you download those images to your local computer and you can test stuff locally.

### Installation

~~Go to the release page and download the executable as per your operating system/cpu architecture.~~
For now, please build from source. Github actions need to be updated for linux and macos builds.

#### Build from source
To build from source, you must have rustc and cargo available on your machine.
If you don't have it installed, check [Rustup Docs](https://rustup.rs/) to get started.

Then, follow the steps below:

- `git clone https://github.com/aashan10/picsum.git`
- `cd picsum && cargo build -r`

After that, you'll have a release binary inside `<project-dir>/target/release/picsum` (`<project-dir>/target/release/picsum.exe` on windows).

On unix based systems, you can move the generated binary to `/usr/local/bin` or set up appropriate `PATH` environment variable in order to run the `picsum` command.

### Usage
```bash 
picsum --help
Usage: picsum [<arg> <value>]
  --count <number>        Number of images to download
                              default: 50
  --width <number>        Width of image to be downloaded
                              default: 1920
  --height <number>       Height of image to be downloaded
                              default: 1080
  --dir <directory>       Directory to be used to save downloaded files
                              default: ~/Downloads
  --threads <number>      Allowed number of threads to be used for parallel downloads
                              default: 4
  --help                  Display this help message
```

### Disclaimer
The downloaded images are not covered by copyright as they are generated by an external service. This is merely a tool for gathering the contents which is already available on the internet.
The author is not legally bound to any of the images downloaded via this tool.
