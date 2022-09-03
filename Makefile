wasm:
	wasm-pack build

node:
	npm run start

bench1:
	cargo bench | tee bench1.txt

bench2:
	cargo bench | tee bench2.txt

release:
	wasm-pack build --release

cmp:
	cargo benchcmp bench1.txt bench2.txt

twiggy:
	twiggy top -n 10 pkg/wasm_lib_bg.wasm

snip:
	wasm-snip -o wasm_lib_snip.wasm pkg/wasm_lib_bg.wasm

check_release:
	wc -c pkg/wasm_lib_bg.wasm

check_snip:
	wc -c wasm_lib_snip.wasm

zip:
	gzip -9 < pkg/wasm_lib_bg.wasm | wc -c

login:
	wasm-pack login

publish:
	wasm-pack publish


.PHONY: wasm, node, bench1, release, bench2, cmp, twiggy, snip, check_release, check_snip, zip, login