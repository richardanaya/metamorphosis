use js_ffi::*;

pub enum ComputationGraphNode {
    Get2D(Box<dyn ToShader>, Box<dyn ToShader>, Box<dyn ToShader>),
    Add(Box<dyn ToShader>, Box<dyn ToShader>),
    Mul(Box<dyn ToShader>, Box<dyn ToShader>),
    Value(f32),
    Variable(String),
}

pub fn output_x() -> ComputationGraphNode {
    ComputationGraphNode::Variable("this.thread.x".to_string())
}

pub fn output_y() -> ComputationGraphNode {
    ComputationGraphNode::Variable("this.thread.y".to_string())
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
                y.to_shader(),
                x.to_shader()
            ),
            ComputationGraphNode::Value(v) => format!("{}", v),
            ComputationGraphNode::Variable(v) => v.clone(),
        }
    }
}

pub struct GPUKernel {
    params: JSObject,
    kernel: Option<JSObject>,
}

pub struct ComputationResult(JSObject);

impl ToJSValue for ComputationResult {
    fn to_js_value(&self) -> JSValue {
        (self.0).to_js_value()
    }
    fn to_js_type(&self) -> JSType {
        (self.0).to_js_type()
    }
}

impl ToJSValue for &ComputationResult {
    fn to_js_value(&self) -> JSValue {
        (&self.0).0
    }
    fn to_js_type(&self) -> JSType {
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
            params: api.create_params(),
            kernel: None,
        }
    }

    pub fn set_compute_graph(&mut self, node: impl ToShader) {
        let api = globals::get::<GPU>();
        self.kernel = Some(api.create_kernel(&self.params, &node.to_shader()));
    }

    pub fn compute_2d(&mut self, width: u32, height: u32) -> ComputationResult {
        let api = globals::get::<GPU>();
        if self.kernel.is_some() {
            ComputationResult(api.compute_2d(
                &self.params,
                self.kernel.as_mut().unwrap(),
                width,
                height,
            ))
        } else {
            js!(console.error).invoke_1("kernel not configured");
            panic!();
        }
    }

    pub fn input_2d(&mut self, data: Vec<f32>, width: u32, height: u32) -> ComputationGraphNode {
        let api = globals::get::<GPU>();
        ComputationGraphNode::Variable(api.add_input_2d(&self.params, data, width, height))
    }

    pub fn input_f32(&mut self, value: f32) -> ComputationGraphNode {
        let api = globals::get::<GPU>();
        ComputationGraphNode::Variable(api.add_input_f32(&self.params, value))
    }
}

struct GPU {
    fn_create_kernel: JSInvoker,
    fn_create_params: JSInvoker,
    fn_compute_2d: JSInvoker,
    fn_add_input_2d: JSInvoker,
    fn_add_input_f32: JSInvoker,
}

impl Default for GPU {
    fn default() -> Self {
        GPU {
            fn_create_kernel: js!(
                (params,shader) => {
                    let gpu = new GPU();
                    let complete_shader = "(function("+params.map(x=>x.name).join(",")+"){ return "+shader+"; })";
                    console.log(complete_shader);
                    return gpu.createKernel(eval(complete_shader));
                }
            ),
            fn_create_params: js!(
                () => {
                    return [];
                }
            ),
            fn_add_input_2d: js!(
                (params,val,w,h) => {
                    let var_name = "var"+params.length;
                    params.push({name:var_name,type:"2d",value:GPU.input(val,[w,h])});
                    return var_name;
                }
            ),
            fn_add_input_f32: js!(
                (params,val) => {
                    let var_name = "var"+params.length;
                    params.push({name:var_name,type:"f32",value:val});
                    return var_name;
                }
            ),
            fn_compute_2d: js!(
                (params,kernel,width,height) => {
                    kernel.setOutput([width, height]);
                    return kernel.apply(kernel,params.map(x=>x.value));
                }
            ),
        }
    }
}

impl GPU {
    fn create_kernel(&self, params: &JSObject, shader: &str) -> JSObject {
        JSObject(self.fn_create_kernel.invoke_2(params, shader))
    }

    fn create_params(&self) -> JSObject {
        JSObject(self.fn_create_params.invoke_0())
    }

    fn compute_2d(
        &self,
        params: &JSObject,
        kernel: &JSObject,
        width: u32,
        height: u32,
    ) -> JSObject {
        JSObject(self.fn_compute_2d.invoke_4(params, kernel, width, height))
    }

    fn add_input_2d(&self, gpu: &JSObject, data: Vec<f32>, width: u32, height: u32) -> String {
        self.fn_add_input_2d
            .invoke_4(gpu, &data, width, height)
            .as_string()
    }

    fn add_input_f32(&self, gpu: &JSObject, val: f32) -> String {
        self.fn_add_input_f32.invoke_2(gpu, val).as_string()
    }
}
