# fakelish

English-like word generator; port of Ryo Ota's fakelish:
 * [NPM](https://github.com/nwtgck/fakelish-npm)
 * [Go](https://github.com/nwtgck/go-fakelish)

## Build CLI:
```bash
git clone https://github.com/shareable-vision/fakelish-rs
cd ./fakelish-rs
cargo build --release --features="cli"
./target/release/fakelish --help
```

## Use Library:
```rs
use fakelish::WordGenerator;
fn main() {
	// Configure the word generator:
	let mut generate = WordGenerator::new(7, 11, 4);
	let word: String = generate.word();
	let words: Vec<String> = generate.phrase(20);
}
```

The output should be like the following:
```
liverge
britingl
espectu
ideative
ineffectomy
oweling
gynously
reloade
econful
oominis
fluctually
inbreechi
ozonitrin
nonflaggie
wolframe
oligophase
illetonico
weightl
outleaves
rigoratini
```
