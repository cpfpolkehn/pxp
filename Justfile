test:
    cargo nextest run

tokenise +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin tokenise --release -- {{args}}

parse +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-parser --bin parse --release -- {{args}}

parse-doc +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-docblock --bin parse-doc --release -- {{args}}

index +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-indexer --bin index --release -- {{args}}

typemap +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-typemap --bin typemap --release -- {{args}}