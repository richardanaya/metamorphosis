# metamorphosis 🌟🐛🦋

<a href="https://docs.rs/metamorphosis"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A GPGPU computation graph executor for web assembly.

```rust
use js_ffi::*;
use metamorphosis::*;

#[no_mangle]
pub fn main() -> () {
    let mut kernel = GPUKernel::new();
    kernel.set_compute_graph(ComputationGraphNode::Value(42.0));
    let output = kernel.compute_2d(3, 3);
    js!(console.log).invoke_1(TYPE_OBJECT, output.as_js_value());
}
```
