use js_ffi::*;

pub enum ComputationGraphNode {
    Value(f32),
}

impl ComputationGraphNode {
    fn to_shader(&self) -> String {
        "function(){return 42;}".to_string()
    }
}

pub struct GPUKernel {
    gpu: JSObject,
    kernel: Option<JSObject>,
}

impl GPUKernel {
    pub fn new() -> Self {
        let api = globals::get::<GPU>();
        GPUKernel {
            gpu: api.create_gpu(),
            kernel: None,
        }
    }

    pub fn set_compute_graph(&mut self, node: ComputationGraphNode) {
        let api = globals::get::<GPU>();
        self.kernel = Some(api.create_kernel(&self.gpu, &node.to_shader()));
    }

    pub fn compute_2d(&mut self, width: u32, height: u32) -> JSObject {
        let api = globals::get::<GPU>();
        api.compute_2d(self.kernel.as_ref().unwrap(), width, height)
    }
}

struct GPU {
    fn_create_gpu: JSFunction,
    fn_create_kernel: JSFunction,
    fn_compute_2d: JSFunction,
}

impl Default for GPU {
    fn default() -> Self {
        GPU {
            fn_create_gpu: js!(
                () => {
                    debugger;
                    return new GPU();
                }
            ),
            fn_create_kernel: js!(
                (gpu,shader) => {
                    debugger;
                    return gpu.createKernel(function() {
                        return 42
                    }, { output: [3,3] });
                }
            ),
            fn_compute_2d: js!(
                (kernel,width,height) => {
                    debugger;
                    return kernel()
                }
            ),
        }
    }
}

impl GPU {
    fn create_gpu(&self) -> JSObject {
        JSObject(self.fn_create_gpu.invoke_0())
    }

    fn create_kernel(&self, gpu: &JSObject, shader: &str) -> JSObject {
        JSObject(self.fn_create_kernel.invoke_2(
            TYPE_OBJECT,
            gpu.as_js_value(),
            TYPE_STRING,
            to_js_string(shader),
        ))
    }

    fn compute_2d(&self, kernel: &JSObject, width: u32, height: u32) -> JSObject {
        JSObject(self.fn_compute_2d.invoke_3(
            TYPE_OBJECT,
            kernel.as_js_value(),
            TYPE_NUM,
            width.into(),
            TYPE_NUM,
            height.into(),
        ))
    }
}
