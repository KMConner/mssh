mod arguments;

fn main() {
    let cli = match arguments::parse() {
        Ok(c) => c,
        Err(e) => e.exit()
    };
}
