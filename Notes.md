## build base_sbf_addition_checker

cargo build-bpf \
xxd -i ./target/deploy/addition_checker.so > bytecode.rs

## convert dot to svg

dot -Tsvg MY_TEST/out.dot > MY_TEST/t3.svg

## run test

cargo test --package sol-azy --bin sol-azy -- reverse::tests --show-output

## useful options for reverse module

keyword `reverse`

mode:

- disass
- cfg
- both
- rusteq (not supported yet)

params:

--out-dir (String)
--target-file (String)
--is-stripped (optionnal)

example:

`cargo run -- reverse --mode disass --out-dir . --bytecodes-file test_cases/base_sbf_addition_checker/target/sbf-solana-solana/release/addition_checker.so --is-stripped` 

### for future improvment

instead of input file we can use program address and then fetch the bytecode from a solana node
