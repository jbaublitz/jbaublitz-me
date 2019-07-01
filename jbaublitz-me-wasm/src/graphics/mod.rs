use std::error::Error;

use js_sys::Float32Array;
use wasm_bindgen::{JsCast,JsValue};
use web_sys::{window,HtmlCanvasElement,WebGlBuffer,WebGlProgram,WebGlRenderingContext,WebGlShader};

use super::JbaublitzError;
use super::textset::set_text;

mod matrix;
mod shaders;
mod vertex;
use vertex::*;

enum ShaderType {
    Vertex,
    Fragment,
}

impl Into<u32> for ShaderType {
    fn into(self) -> u32 {
        match self {
            ShaderType::Vertex => WebGlRenderingContext::VERTEX_SHADER,
            ShaderType::Fragment => WebGlRenderingContext::FRAGMENT_SHADER,
        }
    }
}

struct WhiteNoiseCube {
    context: WebGlRenderingContext
}

impl WhiteNoiseCube {
    const RUST_ARR: &'static [f32] = &[
        -1.0, 1.0,
        1.0, 1.0,
        -1.0, -1.0,
        1.0, -1.0,
    ];

    fn setup() -> Result<Self, Box<dyn Error>> {
        let elem_op = window().and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("graphics-canvas"));
        let elem_canvas = match elem_op {
            Some(e) => e.dyn_into::<HtmlCanvasElement>().map_err(|_| {
                Box::new(JbaublitzError("ID #graphics-canvas does not point to \
                                                             a <canvas> tag"))
            })?,
            None => {
                return Err(Box::new(JbaublitzError("No element with id=\"graphics-canvas\" \
                                                           found")));
            }
        };
        let context_object = match elem_canvas.get_context("webgl") {
            Ok(Some(co)) => co,
            _ => return Err(Box::new(JbaublitzError("Could not get canvas webgl context"))),
        };
        let context = context_object.dyn_into::<WebGlRenderingContext>().map_err(|_| {
            Box::new(JbaublitzError("Could not get canvas webgl context as WebGlRenderingContext \
                                    object"))
        })?;
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        Ok(WhiteNoiseCube { context })
    }

    fn new_shader(&self, type_: ShaderType, source: &str) -> Result<WebGlShader, Box<dyn Error>> {
        let shader = self.context.create_shader(type_.into()).ok_or(
            Box::new(JbaublitzError("Failed to create shader"))
        )?;
        self.context.shader_source(&shader, source);
        self.context.compile_shader(&shader);

        let compiled_successfully = self.context.get_shader_parameter(
            &shader,
            WebGlRenderingContext::COMPILE_STATUS
        );
        if compiled_successfully.as_bool() != Some(true) {
            self.context.delete_shader(Some(&shader));
            return Err(Box::new(JbaublitzError("Failed to compile shader")));
        }

        Ok(shader)
    }

    fn render_program(&self) -> Result<WebGlProgram, Box<dyn Error>> {
        let vertex_shader = self.new_shader(ShaderType::Vertex, shaders::VERTEX_SHADER_SOURCE)?;
        let fragment_shader = self.new_shader(ShaderType::Fragment,
                                              shaders::FRAGMENT_SHADER_SOURCE)?;

        let shader_program = self.context.create_program().ok_or(
            Box::new(JbaublitzError("Failed to create WebGL program"))
        )?;
        self.context.attach_shader(&shader_program, &vertex_shader);
        self.context.attach_shader(&shader_program, &fragment_shader);
        self.context.link_program(&shader_program);

        let js_bool = self.context.get_program_parameter(&shader_program,
                                                         WebGlRenderingContext::LINK_STATUS);
        if js_bool.as_bool() != Some(true) {
            return Err(Box::new(JbaublitzError("Failed to link shader program")));
        }

        Ok(shader_program)
    }

	fn init_buffers(&self) -> Result<WebGlBuffer, Box<dyn Error>> {
        let position_buffer = self.context.create_buffer().ok_or(
            Box::new(JbaublitzError("Failed to create buffer"))
        )?;
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));

        let arr = unsafe { Float32Array::view(Self::RUST_ARR) }.buffer();
        self.context.buffer_data_with_opt_array_buffer(WebGlRenderingContext::ARRAY_BUFFER,
                                                       Some(&arr),
                                                       WebGlRenderingContext::STATIC_DRAW);
		Ok(position_buffer)
	}

    fn draw(&self) -> Result<(), Box<dyn Error>> {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear_depth(1.0);
        self.context.enable(WebGlRenderingContext::DEPTH_TEST);
        self.context.depth_func(WebGlRenderingContext::LEQUAL);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT 
                           | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        let mut mat4 = matrix::Mat4::new();
        mat4.translate(None, None, Some(-6.0));

        Ok(())
    }
}

pub fn graphics() -> Result<(), Box<dyn Error>> {
    set_text(r#"
<canvas id="graphics-canvas" width="640" height="640">
</canvas>
"#)?;
    let context = WhiteNoiseCube::setup()?;
	let program = context.render_program()?;
	context.init_buffers()?;
    context.draw()?;
    Ok(())
}
