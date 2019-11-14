# metamorphosis 🌟🐛🦋

<a href="https://docs.rs/metamorphosis"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A GPGPU computation graph executor for web assembly.

# Generate a matrix of 42

```rust
use js_ffi::*;
use metamorphosis::*;

#[no_mangle]
pub fn main() -> () {
    let mut kernel = GPUKernel::new();
    kernel.set_compute_graph(ComputationGraphNode::Value(42.0));
    let output = kernel.compute_2d(512, 512);
    js!(console.log).invoke_1(TYPE_OBJECT, output.as_js_value());
}
```

# A simple physics system
This uses webgl to parallel calculate a simple `position + velocity*time_step` calculation for 10 positions
```rust
use js_ffi::*;
use metamorphosis::*;

#[no_mangle]
pub fn main() -> () {
    let positions = 
    let mut kernel = GPUKernel::new();
    
    // create a matrix of 10 positions 
    let position = kernel.input_2d(vec![
        0.0,0.0,0.0,
        0.0,0.0,0.0,
        0.0,0.0,0.0,
        0.0,0.0,0.0,
        0.0,0.0,0.0,
        0.0,0.0,0.0,
        0.0,0.0,0.0,
        0.0,0.0,0.0,
        0.0,0.0,0.0,
        0.0,0.0,0.0,
    ],3,10);
    // create a matrix of 10 velocities 
    let velocity = kernel.input_2d(vec![
        1.0,0.0,0.0,
        0.0,1.0,0.0,
        0.0,0.0,1.0,
        0.0,1.0,0.0,
        1.0,0.0,0.0,
        0.0,1.0,0.0,
        0.0,0.0,1.0,
        0.0,1.0,0.0,
        1.0,0.0,0.0,
        0.0,1.0,0.0,
    ],3,10);
    let time_step = kernel.input_float32(.1);
    kernel.set_compute_graph(ComputationGraphNode::Value(42.0));
    //calculate the output 10 positions of the physics system
    let output = kernel.compute_2d(3, 10);
    js!(console.log).invoke_1(TYPE_OBJECT, output.as_js_value());
}
```
