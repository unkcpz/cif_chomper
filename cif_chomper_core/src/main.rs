use cif_chomper_core::parser::cif2_file;

fn main() {
    let mil_101 = include_str!("../../cif_chomper/example_data/mil-101.cif");
    let model = cif2_file(mil_101).unwrap();
    // dbg!(model);
}
