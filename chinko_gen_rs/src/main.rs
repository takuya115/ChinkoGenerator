mod chinko_generator;

use chinko_generator::ChinkoGenerator;

fn main() {
    let mut chinko_gen = ChinkoGenerator::new();
    while !chinko_gen.gen_triplet() {
        // ループ
    }

    println!("{}(ﾎﾞﾛﾝｯ)", chinko_gen);
    println!("{}文字目で出ました", chinko_gen.to_string().len());
}
