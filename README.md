A demo of writing riscv baremetal code in Rust. For more details, see <http://accu.cc/content/ckb/rust_riscv_bytes/>.

For quick run:

```sh
$ cargo build

$ git clone https://github.com/nervosnetwork/ckb-vm
$ cd ckb-vm
$ cargo run --example ckb-vm-runner rust-riscv64imac-unknown-none-elf/target/riscv64imac-unknown-none-elf/debug/rust-riscv64imac-unknown-none-elf -- Hello World!

# Hello World!
```
