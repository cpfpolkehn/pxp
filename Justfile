test:
    cargo nextest run

tokenise +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin tokenise --release -- {{args}}

parse +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin parse --release -- {{args}}

parse-doc +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin parse-doc --release -- {{args}}

index +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin index --release -- {{args}}

@konsole example *args:
    RUSTFLAGS=-Awarnings cargo run -q --package konsole --example {{example}} --release -- {{args}}

pretty-print +args:
    RUSTFLAGS=-Awarnings cargo run -q --package pxp-tools --bin pretty-print --release -- {{args}}