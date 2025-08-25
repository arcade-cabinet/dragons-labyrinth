use std::env;
use hexroll_transformer::HexrollTransformer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path-to.hbf> [--export-db game.db] [--api-key KEY]", args[0]);
        std::process::exit(1);
    }
    let mut export: Option<String> = None;
    let mut api_key: Option<String> = None;
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--export-db" => { export = args.get(i+1).cloned(); i += 2; }
            "--api-key"   => { api_key = args.get(i+1).cloned(); i += 2; }
            _ => { i += 1; }
        }
    }
    if api_key.is_some() { std::env::set_var("OPENAI_API_KEY", api_key.unwrap()); }

    let hbf = &args[1];
    let tf = HexrollTransformer::open(hbf, "gpt-4o-mini").expect("open hbf");
    let pages = tf.transform();

    println!("Transformed {} pages (non-empty rows)", pages.len());
    for p in pages.iter().take(5) {
        println!("- {} {:?} {:?}", p.uuid, p.page_type, p.title);
    }

    if let Some(path) = export {
        tf.export_to_sea_db(&path, &pages).expect("export to db");
        println!("Exported world database to {}", path);
    }
}
