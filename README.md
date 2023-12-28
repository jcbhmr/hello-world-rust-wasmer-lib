# "Hello world!" Rust Wasmer library

🟪 A Rust library compiled to WASM for use with Wasmer

<table align=center><td>

```js
console.log(hello_world_rust_wasmer_lib.add(1, 2));
//=> 3
console.log(hello_world_rust_wasmer_lib.greet("Alan Turing"));
//=> 'Hello Alan Turing!'

const calc = Calculator.new(hello_world_rust_wasmer_lib, 15);
calc.add(5);
console.log(calc.currentValue());
//=> 20
```

<tr><td>

```py
print(hello_world_rust_wasmer_lib.add(1, 2))
#=> 3
print(hello_world_rust_wasmer_lib.greet("Alan Turing"))
#=> 'Hello Alan Turing!'

calc = Calculator.new(hello_world_rust_wasmer_lib, 15)
calc.add(5)
print(calc.current_value())
#=> 20
```

</table>

🚀 Quickstart template for Wasmer-based WASM libraries \
🦀 Written in Rust \
📖 Follows the [Wasmer Pack tutorial series] \
📥 Imports some host functions \
🟨 Can be used with JavaScript \
🐍 Can be used with Python \
👩‍⚖️ [0BSD licensed] template

## Installation

![Wasmer](https://img.shields.io/static/v1?style=for-the-badge&message=Wasmer&color=4946DD&logo=Wasmer&logoColor=FFFFFF&label=)
![npm](https://img.shields.io/static/v1?style=for-the-badge&message=npm&color=CB3837&logo=npm&logoColor=FFFFFF&label=)
![Yarn](https://img.shields.io/static/v1?style=for-the-badge&message=Yarn&color=2C8EBB&logo=Yarn&logoColor=FFFFFF&label=)
![pnpm](https://img.shields.io/static/v1?style=for-the-badge&message=pnpm&color=222222&logo=pnpm&logoColor=F69220&label=)
![pip](https://img.shields.io/static/v1?style=for-the-badge&message=pip&color=3776AB&logo=Python&logoColor=FFFFFF&label=)

You can install this package using npm through [Wasmer]'s npm redistribution CDN
using the [Wasmer CLI].

```sh
wasmer add jcbhmr/hello-world-rust-wasmer-lib --npm
```

You can also use `--yarn` or `--pnpm` to install the package using [Yarn] or
[pnpm] respectively. See `wasmer add --help` for more info.

⚠️ `wasmer add` doesn't support targeting browser runtimes. It uses
`node:fs/promises` `readFile()`.

If you're using Python you can install it using pip:

```sh
wasmer add jcbhmr/hello-world-rust-wasmer-lib --pip
```

## Usage

![Node.js](https://img.shields.io/static/v1?style=for-the-badge&message=Node.js&color=339933&logo=Node.js&logoColor=FFFFFF&label=)
![Deno](https://img.shields.io/static/v1?style=for-the-badge&message=Deno&color=000000&logo=Deno&logoColor=FFFFFF&label=)
![Bun](https://img.shields.io/static/v1?style=for-the-badge&message=Bun&color=000000&logo=Bun&logoColor=FFFFFF&label=)
![Python](https://img.shields.io/static/v1?style=for-the-badge&message=Python&color=3776AB&logo=Python&logoColor=FFFFFF&label=)

<!-- prettier-ignore -->
Imports (in order): [`fib`](fib.wai), [`io`](io.wai) \
Exports: [`hello-world-rust-wasmer-lib`](hello-world-rust-wasmer-lib.wai)

Here's an example of how you would import and use this package using Node.js:

```js
import { bindings } from "@jcbhmr/hello-world-rust-wasmer-lib";
import { Calculator } from "@jcbhmr/hello-world-rust-wasmer-lib/src/bindings/hello_world_rust_wasmer_lib/hello_world_rust_wasmer_lib.js";
import * as io from "./io.js";
import * as fib from "./fib.js";
const hello_world_rust_wasmer_lib = await bindings.hello_world_rust_wasmer_lib(
  fib,
  io
);

console.log(hello_world_rust_wasmer_lib.add(1, 2));
//=> 3
console.log(hello_world_rust_wasmer_lib.greet("Alan Turing"));
//=> 'Hello Alan Turing!'

const calc = Calculator.new(hello_world_rust_wasmer_lib, 15);
calc.add(5);
console.log(calc.currentValue());
//=> 20
```

ℹ To use the `Calculator` class you need to construct it using
`Calculator.new()`, **not `new Calculator()`**. Note that we also need to pass
it the `exports` bindings context.

To use this package with Python this is what you would do:

```py
from hello_world_rust_wasmer_lib import bindings
from hello_world_rust_wasmer_lib.bindings.hello_world_rust_wasmer_lib import Calculator
import .fib
import .io
hello_world_rust_wasmer_lib = bindings.hello_world_rust_wasmer_lib(fib, io)

print(hello_world_rust_wasmer_lib.add(1, 2))
#=> 3
print(hello_world_rust_wasmer_lib.greet("Alan Turing"))
#=> 'Hello Alan Turing!'

calc = Calculator.new(hello_world_rust_wasmer_lib, 15)
calc.add(5)
print(calc.current_value())
#=> 20
```

## Development

![Wasmer](https://img.shields.io/static/v1?style=for-the-badge&message=Wasmer&color=4946DD&logo=Wasmer&logoColor=FFFFFF&label=)
![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)

You can make sure everything looks good like this:

```sh
cargo wasmer --dry-run --debug --out-dir=./out
mv ./out/wasmer.toml ./out/wapm.toml
wasmer-pack js ./out --out-dir=./out/js
wasmer-pack py ./out --out-dir=./out/py
```

You can get [the wasmer-pack CLI] via `cargo install wasmer-pack-cli`. You can
install [cargo-wasmer] via `cargo install cargo-wasmer`.

<!-- prettier-ignore-start -->
[cargo-wasmer]: https://github.com/wasmerio/cargo-wasmer
[wasmer]: https://wasmer.io/
[wasmer cli]: https://github.com/wasmerio/wasmer
[yarn]: https://yarnpkg.com/
[pnpm]: https://pnpm.io/
[the wasmer-pack CLI]: https://github.com/wasmerio/wasmer-pack
[0bsd licensed]: https://github.com/jcbhmr/hello-world-rust-wasmer-lib/blob/main/LICENSE
[wasmer pack tutorial series]: https://wasmerio.github.io/wasmer-pack/user-docs/tutorial/01-hello-world.html
<!-- prettier-ignore-end -->
