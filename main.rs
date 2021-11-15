use rand::Rng;
use std::env;
use std::fs;
use std::io::Write;

// Parameters: a b c n_runs n_pops n_in_pop p_cross p_mut
fn main() {
    let args: Vec<String> = env::args().collect();
    let n_pops: u8 = args[5].parse().unwrap();
    let n_in_pop: u8 = args[6].parse().unwrap();

    if n_pops * n_in_pop > 150 {
        panic!("n_pops * n_in_pop must be <= 150");
    }

    let mut file = fs::File::create("logs.txt").unwrap();

    let a: i32 = args[1].parse().unwrap();
    let b: i32 = args[2].parse().unwrap();
    let c: i32 = args[3].parse().unwrap();
    let n_runs: u8 = args[4].parse().unwrap();
    let p_cross: f64 = args[7].parse().unwrap();
    let p_mut: f64 = args[8].parse().unwrap();

    let f = |x: i32| a * x.pow(2) + b * x + c;

    let fbinary = |binary: &String| {
        let x = i32::from_str_radix(&binary, 2).unwrap();
        f(x)
    };

    let mut rng = rand::thread_rng();

    for r in 0..n_runs {
        println!();
        println!("==============");
        println!("{} RUN 🚀", r + 1);
        println!("==============");
        println!();

        let mut last_pop: Vec<String> = Vec::new();

        for _ in 0..n_in_pop {
            last_pop.push(format!("{:08b}", rng.gen::<u8>()));
        }

        for i in 0..n_pops {
            println!("{}. {:?}", i + 1, last_pop);

            let mut pairs: Vec<Vec<String>> = Vec::new();

            {
                let mut pool = last_pop.clone();
                let mut pool_len = pool.len();

                while pool_len > 0 {
                    let mut pair: Vec<String> = Vec::new();
                    let mut rnd = rng.gen_range(0..pool_len);
                    pair.push(pool.remove(rnd));
                    pool_len -= 1;
                    if pool_len > 0 {
                        rnd = rng.gen_range(0..pool_len);
                        pair.push(pool.remove(rnd));
                        pool_len -= 1;
                    }
                    pairs.push(pair);
                }
            }

            let mut mod_pop: Vec<String> = Vec::new();

            // Cross
            for pair in pairs {
                if pair.len() == 2 {
                    let rnd = rng.gen_range(0.0..1.0);
                    let do_cross = p_cross >= rnd;

                    let mut new_first = pair[0].to_string();
                    let mut new_second = pair[1].to_string();

                    if do_cross {
                        let cut_index = rng.gen_range(1..8);
                        let first_cut = new_first[cut_index..8].to_string();
                        let second_cut = new_second[cut_index..8].to_string();
                        new_first = new_first[0..cut_index].to_string() + &second_cut;
                        new_second = new_second[0..cut_index].to_string() + &first_cut;
                    }

                    mod_pop.push(new_first);
                    mod_pop.push(new_second);
                } else {
                    mod_pop.push(pair[0].to_string());
                }
            }

            // Mutate
            let mut values: Vec<i32> = Vec::new();
            let mut minimum: i32 = 2147483647;

            for x in &mut mod_pop {
                let xclone = x.clone();

                for (i, c) in xclone.chars().enumerate() {
                    let rnd = rng.gen_range(0.0..1.0);
                    let do_mutate = p_mut >= rnd;

                    if do_mutate {
                        let mut replace_with = "0";
                        if c == '0' {
                            replace_with = "1"
                        }
                        x.replace_range(i..i + 1, replace_with);
                    }
                }

                let v = fbinary(x);
                if minimum > v {
                    minimum = v
                }
                values.push(v);
            }

            let mut sum: i32 = 0;
            let mut new_values: Vec<i32> = Vec::new();

            for value in &values {
                let new_value = value - minimum + 1;
                sum += new_value;
                new_values.push(new_value)
            }

            // Selection
            let mut ps: Vec<f64> = Vec::new();

            for value in &new_values {
                ps.push(*value as f64 / sum as f64);
            }

            last_pop = Vec::new();
            
            for _ in 0..n_in_pop {
                let rnd = rng.gen_range(0.0..1.0);
                let mut sum = 0.0;
                for i in 0..ps.len() {
                    let p = ps[i];
                    sum += p;
                    if sum >= rnd {
                        last_pop.push(mod_pop[i].to_string());
                        break;
                    }
                }
            }
        }

        // Get best in run
        let mut best_x = 0; 
        let mut best_y = -2147483647;

        for x in last_pop {
            let xint = i32::from_str_radix(&x, 2).unwrap();
            let y = f(xint); 

            if y > best_y {
                best_y = y;
                best_x = xint;
            }
        }

        println!();
        println!("BEST 🏆: {:?}", best_x);

        let output = format!("{} {} \n", best_x, best_y);
        file.write(output.as_bytes()).unwrap();
    }

    println!();
    println!("LOGS FILE CREATED 📄");
}
