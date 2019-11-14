# metamorphosis 🌟🐛🦋

<a href="https://docs.rs/metamorphosis"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A GPGPU computation graph executor for web assembly.

- [x] 2d float matrix inputs
- [x] float scalar inputs
- [x] 2d float matrix outputs

# Generate a 512x515 matrix of 42

```rust
use metamorphosis::*;

#[no_mangle]
pub fn main() -> () {
    let mut kernel = GPUKernel::new();
    // for each position in our output matrix we will execute this 
    // compute graph node that just returns 42
    kernel.set_compute_graph(ComputationGraphNode::Value(42.0));
    // run it
    let output = kernel.compute_2d(512, 512);
}
```

# Physics System

This calculate a `position + velocity*time_step` calculation for 10 positions in parallel

```rust
use metamorphosis::*;

#[no_mangle]
pub fn main() -> () {
    let mut kernel = GPUKernel::new();
    
    // create a matrix representing 10 positions 
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
    
    // create a matrix representing 10 velocities 
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
    
    // create a float that will represent a time step
    let time_step = kernel.input_float32(.1);
    
    // specify `position + velocity*time_step` as a graph of computation
    kernel.set_compute_graph({
            // for each position in output matrix, find the position component 
            // and its corresponding velocity  component from our input matricies
            let p = get_2d(position,OUTPUT_X,OUTPUT_Y); 
            let v = get_2d(velocity,OUTPUT_X,OUTPUT_Y); 
            // calculate a new position component
            add(p,mul(v,time_step))
        });
    
    // calculate the output 10 positions of the physics system
    let output = kernel.compute_2d(3, 10);
}
```
