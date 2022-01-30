pub mod load;

fn main() {
    let redirects = load::read_csv("./data/sample.csv".to_string());
    for r in &redirects {
        println!("Mapping: {}\t{}", r.source, r.dest)
    }
}