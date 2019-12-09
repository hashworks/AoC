use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let s1 = Instant::now();

    let buf: &mut Vec<u8> = &mut vec![];
    File::open("./input").unwrap().read_to_end(buf).unwrap();

    let mut layers = [([[b'0'; 25]; 6], 0u8, 0u8, 0u8); 1000];

    let mut fewest_0_layer = 0;

    let mut layer = 0;
    let mut x = 0;
    let mut y = 0;

    for character in buf {
        match character {
            b'0' => layers[layer].1 += 1,
            b'1' => layers[layer].2 += 1,
            b'2' => layers[layer].3 += 1,
            _ => continue,
        }
        layers[layer].0[x][y] = *character;
        if y == 24 {
            if x == 5 {
                if layers[layer].1 < layers[fewest_0_layer].1 {
                    fewest_0_layer = layer;
                }
                y = 0;
                x = 0;
                layer += 1;
            } else {
                y = 0;
                x += 1;
            }
        } else {
            y += 1;
        }
    }

    println!(
        "part1: {} ({}µs)",
        layers[fewest_0_layer].2 as i32 * layers[fewest_0_layer].3 as i32,
        s1.elapsed().as_micros()
    );

    let s2 = Instant::now();

    for x in 0..6 {
        for y in 0..25 {
            for l in 0..1000 {
                match layers[l].0[x][y] {
                    b'0' => {
                        print!(" ");
                        break;
                    }
                    b'1' => {
                        print!("█");
                        break;
                    }
                    _ => {}
                }
            }
        }
        println!();
    }

    println!("part2: Read it! ({}µs)", s2.elapsed().as_micros());

    println!("Time: {}µs", s1.elapsed().as_micros());
}
