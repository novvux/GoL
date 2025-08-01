mod grid;
use std::{env, time::Duration};

#[derive(Debug)]
struct Settings {
    height: usize,
    width: usize,
    speed: Duration,
    random: bool,
}

impl Settings {
    pub fn init() -> Settings {
        //let termsize::Size { rows, cols } = termsize::get().unwrap();
        Settings {
            height: 16, //rows as usize,
            width: 16,  //(cols - 1) as usize,
            speed: Duration::from_millis(250),
            random: true,
        }
    }
}

fn main() {
    let mut settings = Settings::init();
    let args: Vec<String> = env::args().collect();
    let ln = args.len();
    for i in 1..ln {
        match args[i].as_str() {
            "-h" | "--help" => help(),
            "-g" | "--glider" => settings.random = false,
            "-r" | "--height" => {
                if ln > i + 1 {
                    settings.height = args[i + 1]
                        .parse()
                        .expect("Pass a number to height in rows");
                    continue;
                } else {
                    println!("Pass a number to height in rows");
                    std::process::exit(200)
                }
            }
            "-w" | "--width" => {
                if ln > i + 1 {
                    settings.width = args[i + 1]
                        .parse()
                        .expect("Pass a number to width in columns");
                    continue;
                } else {
                    println!("Pass a number to width in columns");
                    std::process::exit(200)
                }
            }
            "-s" | "--speed" => {
                if ln > i + 1 {
                    settings.speed = Duration::from_millis(
                        args[i + 1]
                            .parse()
                            .expect("Pass a number to speed in milliseconds"),
                    );
                    continue;
                } else {
                    println!("Pass a number to speed in milliseconds");
                    std::process::exit(200)
                }
            }
            _ => continue,
        }
    }
    let mut grid = crate::grid::Grid::init(settings.width, settings.height);

    if settings.random {
        grid.randomize();
    } else {
        grid.toggle(2, 2);
        grid.toggle(2, 3);
        grid.toggle(2, 4);
        grid.toggle(1, 4);
        grid.toggle(0, 3);
    }

    loop {
        for y in 0..settings.height {
            for x in 0..settings.width {
                print!(
                    "{}",
                    if grid.get(x as usize, y as usize) {
                        "■"
                    } else {
                        "◦"
                    }
                );
            }
            println!();
        }
        grid.switch();
        std::thread::sleep(settings.speed);
    }
}

/*
 * Very helpful function
 */
fn help() {
    println!(
        "A Game Of Life implemented in rust

        Usage: gol [OPTIONS]

        Options:
          -h, --help            Show this message
          -g, --glider          Show glider instead of random
          -r, --height <number> Change height of the field
          -w, --width <number>  Change width of the field
          -s, --speed <number>  Set time between frames"
    );
    std::process::exit(0)
}
