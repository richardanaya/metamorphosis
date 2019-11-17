use js_ffi::*;

pub enum ComputationGraphNode {
    Get2D(Box<dyn ToShader>, Box<dyn ToShader>, Box<dyn ToShader>),
    Add(Box<dyn ToShader>, Box<dyn ToShader>),
    Mul(Box<dyn ToShader>, Box<dyn ToShader>),
    Value(f32),
    Variable(String),
}

pub fn output_x() -> ComputationGraphNode {
    ComputationGraphNode::Variable("this.threads.x".to_string())
}

pub fn output_y() -> ComputationGraphNode {
    ComputationGraphNode::Variable("this.threads.y".to_string())
}

pub fn add(a: impl ToShader + 'static, b: impl ToShader + 'static) -> ComputationGraphNode {
    ComputationGraphNode::Add(Box::new(a), Box::new(b))
}

pub fn mul(a: impl ToShader + 'static, b: impl ToShader + 'static) -> ComputationGraphNode {
    ComputationGraphNode::Mul(Box::new(a), Box::new(b))
}

pub fn get_2d(
    a: impl ToShader + 'static,
    x: impl ToShader + 'static,
    y: impl ToShader + 'static,
) -> ComputationGraphNode {
    ComputationGraphNode::Get2D(Box::new(a), Box::new(x), Box::new(y))
}

impl ToShader for ComputationGraphNode {
    fn to_shader(&self) -> String {
        match self {
            ComputationGraphNode::Add(a, b) => format!(" {} + {} ", a.to_shader(), b.to_shader()),
            ComputationGraphNode::Mul(a, b) => format!(" {} * {} ", a.to_shader(), b.to_shader()),
            ComputationGraphNode::Get2D(source, x, y) => format!(
                "{}[{}][{}]",
                source.to_shader(),
                x.to_shader(),
                y.to_shader()
            ),
            ComputationGraphNode::Value(v) => format!("{}", v),
            ComputationGraphNode::Variable(v) => v.clone(),
        }
    }
}

pub struct GPUKernel {
    gpu: JSObject,
    kernel: Option<JSObject>,
}

pub struct ComputationResult(JSObject);

impl ToJSValue for ComputationResult {
    fn to_js_value(&mut self) -> JSValue {
        (self.0).to_js_value()
    }
    fn to_js_type(&mut self) -> JSType {
        (self.0).to_js_type()
    }
}

impl ToJSValue for &ComputationResult {
    fn to_js_value(&mut self) -> JSValue {
        (&self.0).0
    }
    fn to_js_type(&mut self) -> JSType {
        (&self.0).to_js_type()
    }
}

pub trait ToShader {
    fn to_shader(&self) -> String;
}

impl GPUKernel {
    pub fn new() -> Self {
        let api = globals::get::<GPU>();
        GPUKernel {
            gpu: api.create_gpu(),
            kernel: None,
        }
    }

    pub fn set_compute_graph(&mut self, node: impl ToShader) {
        let api = globals::get::<GPU>();
        self.kernel = Some(api.create_kernel(&self.gpu, &node.to_shader()));
    }

    pub fn compute_2d(&mut self, width: u32, height: u32) -> ComputationResult {
        let api = globals::get::<GPU>();
        ComputationResult(api.compute_2d(self.kernel.as_ref().unwrap(), width, height))
    }

    pub fn input_2d(&mut self, data: Vec<f32>, width: u32, height: u32) -> ComputationGraphNode {
        let api = globals::get::<GPU>();
        ComputationGraphNode::Variable(api.add_input_2d(
            self.kernel.as_ref().unwrap(),
            data,
            width,
            height,
        ))
    }

    pub fn input_f32(&mut self, value: f32) -> ComputationGraphNode {
        let api = globals::get::<GPU>();
        ComputationGraphNode::Variable(api.add_input_f32(self.kernel.as_ref().unwrap(), value))
    }
}

struct GPU {
    fn_create_gpu: JSInvoker,
    fn_create_kernel: JSInvoker,
    fn_compute_2d: JSInvoker,
    fn_add_input_2d: JSInvoker,
    fn_add_input_f32: JSInvoker,
}

impl Default for GPU {
    fn default() -> Self {
        GPU {
            fn_create_gpu: js!(
                () => {;
                    return new GPU();
                }
            ),
            fn_create_kernel: js!(
                (gpu,shader) => {
                    return {gpu:gpu,shader:shader,params:[]};
                }
            ),
            fn_add_input_2d: js!(
                (kernel,name,arr) => {
                    debugger;
                    let var_name = "var"+kernel.params.length;
                    kernel.params.push({name:name,type:"2d",value:arr});
                    return var_name;
                }
            ),
            fn_add_input_f32: js!(
                (kernel,name,val) => {
                    debugger;
                    let var_name = "var"+kernel.params.length;
                    kernel.params.push({name:name,type:"f32",value:val});
                    return var_name;
                }
            ),
            fn_compute_2d: js!(
                (kernel,width,height) => {
                    debugger;
                    /*gpu.createKernel(eval("(function(){ return "+kernel.shader+"; })"));
                    kernel.setOutput([width, height]);
                    return kernel();*/
                    {}
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
        JSObject(self.fn_create_kernel.invoke_2(gpu, JSString::from(shader)))
    }

    fn compute_2d(&self, kernel: &JSObject, width: u32, height: u32) -> JSObject {
        JSObject(
            self.fn_compute_2d
                .invoke_3(kernel, JSNumber::from(width), JSNumber::from(height)),
        )
    }

    fn add_input_2d(&self, kernel: &JSObject, data: Vec<f32>, width: u32, height: u32) -> String {
        JSString::to_string(self.fn_add_input_2d.invoke_4(
            kernel,
            JSTypedArray::from(&data),
            JSNumber::from(width),
            JSNumber::from(height),
        ))
    }

    fn add_input_f32(&self, kernel: &JSObject, val: f32) -> String {
        JSString::to_string(self.fn_add_input_f32.invoke_2(kernel, JSNumber::from(val)))
    }
}
