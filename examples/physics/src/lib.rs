use js_ffi::*;
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
    
    /*// create a matrix representing 10 velocities 
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
    let time_step = kernel.input_f32(0.1);
    
    // specify `position + velocity*time_step` as a graph of computation
    kernel.set_compute_graph({
            // for each position in output matrix, find the position component 
            // and its corresponding velocity component from our input matricies
            let p = get_2d(position,output_x(),output_y()); 
            let v = get_2d(velocity,output_x(),output_y()); 
            // calculate a new position component
            add(p,mul(v,time_step))
        });
    
    // calculate the output 10 positions of the physics system
    let output = kernel.compute_2d(3, 10);
    js!(console.log).invoke_1(TYPE_OBJECT, output);*/
}