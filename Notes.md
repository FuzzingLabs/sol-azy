## build base_sbf_addition_checker

cargo build-bpf
xxd -i ./target/deploy/addition_checker.so > bytecode.rs

## convert dot to svg

dot -Tsvg MY_TEST/out.dot > MY_TEST/t3.svg

## use the anza cfg graphically

cargo test --package sol-azy --bin sol-azy -- reverse::disass::test --exact --show-output > MY_TEST/out.dot