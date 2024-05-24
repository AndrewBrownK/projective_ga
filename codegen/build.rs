use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Write;

fn main() {
    generate_multi_bases();
}


/// Code-gen for code-gen for code-gen.... kind of absurd
/// Speaking of absurd
/// What if someone wants 16 different basis elements?
/// Well I thought it would be sooo coooool to make basis element bitflags
/// In an 8 dimensional algebra, there are 256 elements (including scalar)
/// But in a 16 dimensional algebra, there are 65,534 elements
/// So while pre-generating combined bases is certainly cool, maybe it is dubious to call
/// 65k constants "convenient" if it is slowing down your IDE or the compiler.
/// So yet again, this is another great situation for feature flags. But we can't just
/// put the flag on 65k constants, that's still a lot to digest.
/// So instead, we'll make sure the feature completely changes the file, so it is actually
/// smaller and easier to compile, and only gets chunky if you are insane and want to do
/// 9+ dimensional stuff
fn generate_multi_bases() {


    let out_dir = &env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(out_dir).join("generated_multi_bases.rs");
    let mut f = File::create(dest_path).unwrap();

    f.write_all(b"
#[allow(non_upper_case_globals)]
impl BasisSignature {
").unwrap();


    let mut max = u8::MAX as u16;
    if cfg!(feature = "very-large-bases") {
        max = u16::MAX;
    }
    let mut numbers: Vec<u16> = (0..=max).collect();
    numbers.sort_by(|a, b| {
        a.count_ones().cmp(&b.count_ones()).then_with(|| {
            b.reverse_bits().cmp(&a.reverse_bits())
        })
    });
    for num in numbers {
        if num.count_ones() < 2 {
            continue
        }

        let mut combined_basis = String::new();
        for i in 0..16 {
            if num & (1 << i) != 0 {
                combined_basis.push(char::from_digit(i, 16).unwrap());
            }
        }

        let line = format!("    pub const e{combined_basis}: BasisSignature = BasisSignature::from_bits_retain(0b{num:016b});\n");
        f.write(line.as_bytes()).unwrap();
    }

    f.write_all(b"
}
").unwrap();
}
