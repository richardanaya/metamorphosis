use js_ffi::*;
use metamorphosis::*;

#[no_mangle]
pub fn main() -> () {
    let mut kernel = GPUKernel::new();
    kernel.set_compute_graph(ComputationGraphNode::Value(42.0));
    let output = kernel.compute_2d(3, 3);
    js!(console.log).invoke_1(TYPE_OBJECT, output.as_js_value());
}
