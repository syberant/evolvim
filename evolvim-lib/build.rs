use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// This is how it should look like:

const SINE_APPROX_AMOUNT: usize = 10000;
// const SINE_APPROX: [f64; SINE_APPROX_AMOUNT] = [
//     (0.0 / (SINE_APPROX_AMOUNT as f64) * 2.0 * PI),
//     (1.0 / (SINE_APPROX_AMOUNT as f64) * 2.0 * PI),
//     ...
//     ((SINE_APPROX_AMOUNT as f64) / (SINE_APPROX_AMOUNT as f64) * 2.0 * PI),
// ];

// fn lookup_sine_approx(n: f64) -> f64 {
//     use std::f64::consts::PI;

//     let n_reduced = n % (2.0 * PI);
//     let index = n_reduced / (2.0 * PI) * (SINE_APPROX_AMOUNT as f64);
//     return SINE_APPROX[index as usize];
// }

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sine_approx.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Now make the lookup table
    f.write(
        format!(
            "
        // amount of entries in the lookup table, the more entries the more precise
        const SINE_APPROX_AMOUNT: usize = {};
        const SINE_APPROX: [f64; SINE_APPROX_AMOUNT] = [
    ",
            SINE_APPROX_AMOUNT
        )
        .as_bytes(),
    )
    .unwrap();

    for i in 0..SINE_APPROX_AMOUNT {
        // let entry = format!("\t({}. / (SINE_APPROX_AMOUNT as f64) * 2.0 * std::f64::consts::PI).sin(),\n", i);
        let entry = format!(
            "\t{:?},\n",
            (i as f64 / (SINE_APPROX_AMOUNT as f64) * 2.0 * std::f64::consts::PI).sin()
        );
        f.write(entry.as_bytes()).unwrap();
    }
    f.write(b"];
    
    pub fn lookup_sine_approx(n: f64) -> f64 {
        use std::f64::consts::PI;

        let mut n_reduced = n % (2.0 * PI);
        if n_reduced < 0.0 {
            // it's negative
            n_reduced += 2.0 * PI;
        }

        //assert!(n_reduced >= 0.0);

        let index = n_reduced * (SINE_APPROX_AMOUNT as f64 / (2.0 * PI));
        let index = index.floor() as usize;

        //assert!(index < SINE_APPROX_AMOUNT, \"{} was not smaller than {}, got argument {}\", index, SINE_APPROX_AMOUNT, n);

        return SINE_APPROX[index];
    }").unwrap();
}
