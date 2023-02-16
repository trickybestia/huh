mod simple_shader;

use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub use simple_shader::SimpleShader;

#[macro_export]
macro_rules! declare_shader {
    ($visibility:vis $program_name:ident, $vertex_shader_source:expr, $fragment_shader_source:expr, ($($attribute_name:ident),*), ($($uniform_name:ident),*)) => {
        mod pog {
            use once_cell::unsync::OnceCell;
            use web_sys::{WebGlProgram, WebGlUniformLocation, WebGlRenderingContext};

            use crate::engine::shader::compile_shader_program;

            static mut INSTANCE: OnceCell<$program_name> = OnceCell::new();

            pub struct $program_name {
                shader_program: WebGlProgram,
                $(
                    $attribute_name: u32,
                )*
                $(
                    $uniform_name: WebGlUniformLocation,
                )*
            }

            impl $program_name {
                fn new(gl: &WebGlRenderingContext) -> Self {
                    let shader_program = compile_shader_program(gl, $vertex_shader_source, $fragment_shader_source);

                    Self {
                        $(
                            $attribute_name: gl.get_attrib_location(&shader_program, stringify!($attribute_name)) as u32,
                        )*
                        $(
                            $uniform_name: gl.get_uniform_location(&shader_program, stringify!($uniform_name)).unwrap(),
                        )*
                        shader_program
                    }
                }

                pub fn instance(gl: &WebGlRenderingContext) -> &'static Self {
                    unsafe {
                        if INSTANCE.get().is_none() {
                            _ = INSTANCE.set(Self::new(gl));
                        }

                        INSTANCE.get().unwrap()
                    }
                }

                pub fn activate(gl: &WebGlRenderingContext) -> &'static Self {
                    let instance = Self::instance(gl);

                    gl.use_program(Some(instance.shader_program()));

                    instance
                }

                pub fn shader_program(&self) -> &WebGlProgram {
                    &self.shader_program
                }

                $(
                    pub fn $attribute_name(&self) -> u32 {
                        self.$attribute_name
                    }
                )*

                $(
                    pub fn $uniform_name(&self) -> &WebGlUniformLocation {
                        &self.$uniform_name
                    }
                )*
            }
        }

        $visibility use pog::$program_name;
    };
}

pub fn compile_shader(gl: &WebGlRenderingContext, source: &str, shader_type: u32) -> WebGlShader {
    let shader = gl.create_shader(shader_type).unwrap();

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if !gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap()
    {
        panic!(
            "Failed to compile shader: {}\nSource:\n{}",
            gl.get_shader_info_log(&shader).unwrap(),
            source
        );
    }

    shader
}

pub fn compile_shader_program(
    gl: &WebGlRenderingContext,
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> WebGlProgram {
    let vertex_shader = compile_shader(
        gl,
        vertex_shader_source,
        WebGlRenderingContext::VERTEX_SHADER,
    );
    let fragment_shader = compile_shader(
        gl,
        fragment_shader_source,
        WebGlRenderingContext::FRAGMENT_SHADER,
    );

    let shader_program = gl.create_program().unwrap();

    gl.attach_shader(&shader_program, &vertex_shader);
    gl.attach_shader(&shader_program, &fragment_shader);

    gl.link_program(&shader_program);

    shader_program
}
