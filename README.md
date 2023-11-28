# hello-bevy-web

## Development

On first check out, install the wasm target first:

```sh
$ rustup target add wasm32-unknown-unknown
```

To compile and run the app, run the following command:

```sh
$ ./compile_and_serve.sh
```

## Conclusion

Development of rust bevy for the web is suuuuper slow :( Takes me 1 minute to
compile and run the example app in Codespaces.

```sh
$ ./compile_and_serve.sh
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on build directory
   Compiling hello-bevy-web v0.1.0 (/workspaces/hello-bevy-web)
    Finished dev [optimized + debuginfo] target(s) in 1m 02s
Serving HTTP on 0.0.0.0 port 8000 (http://0.0.0.0:8000/) ...
```

Not fun. For now :P

**Update**: I tried running it on my company's VM, the compile time at least
looks reasonable.

```sh
$ ./compile_and_serve.sh
   Compiling hello-bevy-web v0.1.0 (/usr/local/<...>/home/ncapule/projects/hello-bevy-web)
    Finished dev [optimized + debuginfo] target(s) in 10.94s
./compile_and_serve.sh: line 6: wasm-bindgen: command not found
Serving HTTP on 0.0.0.0 port 8000 (http://0.0.0.0:8000/) ...
```

Maybe I could make it faster if my compile target is not wasm.
