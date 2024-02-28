use std::{env::args, fs::read, path::Path, process::exit};

use pxp_parser::parse;
use pxp_symbol::SymbolTable;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: pretty-print <path>");
        exit(1);
    }

    let path = args.first().unwrap();
    let path = Path::new(path);
    let contents = read(&path).unwrap();
    let mut symbol_table = SymbolTable::new();
    let result = parse(&contents, &mut symbol_table);

    if !result.diagnostics.is_empty() {
        for diagnostic in result.diagnostics.iter() {
            print!("{diagnostic}");
        }

        exit(1);
    }
}