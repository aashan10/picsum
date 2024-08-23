use std::path::{Path, PathBuf};

use tokio;

#[derive(Debug)]
struct Arguments {
    width: u16,
    height: u16,
    dir: Option<String>,
    threads: u8,
    count: u16,
}

#[derive(Clone)]
struct Dimension {
    width: u16,
    height: u16,
}

#[derive(Clone)]
struct Image {
    url: String,
    dimension: Dimension,
    path: Option<PathBuf>,
    name: String,
}

fn parse_args() -> Arguments {
    let args = std::env::args();
    let mut arguments = Arguments {
        width: 1920,
        height: 1080,
        count: 50,
        dir: Some(
            homedir::my_home()
                .unwrap()
                .unwrap()
                .join("Downloads")
                .to_str()
                .unwrap()
                .to_string(),
        ),
        threads: 4,
    };
    let mut peekable = args.peekable();
    peekable.next();
    while let Some(arg) = peekable.peek() {
        match arg.as_str() {
            "--count" => {
                arguments.count = extract_from_peekable::<u16>(&mut peekable, "count");
            }
            "--width" => {
                arguments.width = extract_from_peekable::<u16>(&mut peekable, "width");
            }
            "--height" => {
                arguments.height = extract_from_peekable::<u16>(&mut peekable, "height");
            }
            "--dir" => {
                arguments.dir = Some(extract_from_peekable::<String>(&mut peekable, "dir"));
            }
            "--threads" => {
                arguments.threads = extract_from_peekable::<u8>(&mut peekable, "threads");
            }
            "--help" => {
                display_help();
                std::process::exit(0);
            }
            text => {
                println!("Invalid argument {}", text);
                display_help();
                std::process::exit(1);
            }
        };
    }

    arguments
}

fn extract_from_peekable<T: std::str::FromStr>(
    peekable: &mut std::iter::Peekable<std::env::Args>,
    field_name: &str,
) -> T {
    peekable.next();
    let v: T = peekable
        .next()
        .unwrap_or_else(|| {
            println!(
                "Please pass a valid {} after --{} flag",
                field_name, field_name
            );
            display_help();
            panic!();
        })
        .parse::<T>()
        .unwrap_or_else(|_| {
            println!(
                "Could not parse {}. Please make sure it's a numeric value",
                field_name,
            );
            display_help();
            panic!();
        });
    v
}

fn display_help() {
    println!("Usage: picsum <number> --width <width> --height <height> --dir <directory> --threads <number>");
    println!("  --count <number>        Number of images to download");
    println!("  --width <number>        Width of image to be downloaded");
    println!("  --height <number>       Height of image to be downloaded");
    println!("  --dir <directory>       Directory to be used to save downloaded files");
    println!(
        "  --threads <number>      Allowed number of threads to be used for parallel downloads"
    );
    println!("  --help                  Display this help message");
}

impl Image {
    fn new(name: String, dimension: Dimension) -> Self {
        let url = format!(
            "https://picsum.photos/{}/{}",
            dimension.width, dimension.height
        );

        Self {
            url,
            dimension,
            name,
            path: None,
        }
    }

    async fn download(&mut self, client: reqwest::Client, dir: Option<String>) {
        let mut path = self.dimension.prepare_download_dir(dir).await;
        path = path.join(&self.name);

        let response = client.get(&self.url).send().await.unwrap();
        let _ = tokio::fs::write(
            &path,
            response.bytes().await.unwrap_or_else(|e| {
                dbg!(e);
                panic!("Failed to write image to disk");
            }),
        )
        .await;
        self.path = Some(path.clone());
        println!("Downloaded image to {}", path.to_str().unwrap());
    }
}

impl Dimension {
    fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    fn get_download_dir(&self, dir: Option<String>) -> PathBuf {
        return match dir {
            Some(dir) => Path::new(&dir)
                .join(format!("{}x{}", self.width, self.height))
                .to_path_buf(),
            None => {
                let dirname = format!("{}x{}", self.width, self.height);
                let home = homedir::my_home().unwrap();

                return match home {
                    Some(path) => path.join("Downloads").join(dirname),
                    None => {
                        panic!("Could not resolve home directory. Please pass --dir flag with appropriate value for save location");
                    }
                };
            }
        };
    }

    async fn prepare_download_dir(&self, dir: Option<String>) -> PathBuf {
        let path = self.get_download_dir(dir);

        if tokio::fs::read_dir(path.clone()).await.is_err() {
            let _ = tokio::fs::create_dir_all(path.clone()).await;
        }

        path
    }
}

#[tokio::main]
async fn main() {
    let arguments = parse_args();

    let dimension = Dimension::new(arguments.width, arguments.height);
    let mut images = vec![];

    for i in 0..arguments.count {
        let name = format!("image-{}.jpg", i);
        images.push(Image::new(name, dimension.clone()));
    }

    let client = reqwest::Client::builder().build().unwrap();

    let tasks_per_thread = images.len() / arguments.threads as usize;

    for i in 0..arguments.threads {
        let start = i as usize * tasks_per_thread;
        let end = if i == arguments.threads - 1 {
            images.len()
        } else {
            (i as usize + 1) * tasks_per_thread
        };

        let imgs = images[start..end].to_vec();

        let dir = arguments.dir.clone();
        let c = client.clone();
        let mut set: tokio::task::JoinSet<Vec<Image>> = tokio::task::JoinSet::new();

        set.spawn(async move {
            let mut thread_images = vec![];
            for mut img in imgs {
                img.download(c.clone(), dir.clone()).await;
                thread_images.push(img);
            }

            thread_images
        });

        loop {
            if set.is_empty() {
                break;
            }
            set.join_next().await;
        }
    }
}
