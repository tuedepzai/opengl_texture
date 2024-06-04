extern crate glfw;
extern crate gl;
extern crate stb_image;
use std::os::raw::c_char;
use std::ffi::{c_void, CStr, CString};
use gl::types::*;
use std::io;
use std::fs;
use std::mem::{size_of, size_of_val};
use glfw::{Action, Context, fail_on_errors};
use glm::GenInt;
use stb_image::stb_image::{stbi_image_free, stbi_load};


fn main() {
    let mut _glfw = glfw::init(fail_on_errors).unwrap();
    _glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    _glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    _glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = _glfw.create_window(600, 400, "untiled window", glfw::WindowMode::Windowed).unwrap();
   // let (mut window, events) = _glfw.with_primary_monitor(|glfw, m|{
     //   glfw.create_window(600, 400, "untiled window",
      //                      m.map_or(glfw::WindowMode::Windowed, |m| glfw::WindowMode::FullScreen(m)))
    //}).unwrap();
    //
    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s));

    let mut SourceVertexShader : &str= r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTex;
out vec3 ourColor;
out vec2 texCoord;
void main(){
    gl_Position = vec4(aPos, 1.0);
    ourColor = aColor;
    texCoord = aTex;
}
    "#;

    let mut SourceFragmentShader : &str = r#"
#version 330 core
in vec3 ourColor;
in vec2 texCoord;
out vec4 FragColor;

uniform sampler2D ourTexture;

void main(){
     FragColor = texture(ourTexture, texCoord); //* vec4(ourColor, 1.0);
}
    "#;
    let mut VAO : GLuint = 0;
    let mut program : GLuint = 0;
    unsafe {
        let mut VertexShader : GLuint = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(VertexShader, 1, &CString::new(SourceVertexShader).unwrap().as_ptr(), std::ptr::null());
        gl::CompileShader(VertexShader);

        let mut VertexRenderState : GLint = 0;
        gl::GetShaderiv(VertexShader, gl::COMPILE_STATUS, &mut VertexRenderState);

        let mut FragmentShader : GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(FragmentShader, 1, &CString::new(SourceFragmentShader).unwrap().as_ptr(), std::ptr::null());
        gl::CompileShader(FragmentShader);
//bruh
        let mut FragmentRenderState : GLint = 0;
        gl::GetShaderiv(FragmentShader, gl::COMPILE_STATUS, &mut FragmentRenderState);

        if(VertexRenderState == gl::TRUE.into() && FragmentRenderState == gl::TRUE.into()){
            println!("shaders compile successfully");
        }

        program = gl::CreateProgram();
        gl::AttachShader(program, VertexShader);
        gl::AttachShader(program, FragmentShader);
        gl::LinkProgram(program);



        let mut vertices : [f32 ; 32] = [
            //pos         | colors      |     texture
            -0.5, 0.0, 0.0, 1.0, 0.0, 0.0,  -1.0, 1.0,
            -0.5, 0.5, 0.0, 0.0, 1.0, 0.0,  -1.0, 0.0,
            0.5, 0.0, 0.0,  0.0, 0.0, 1.0,  1.0, 0.0,
            0.5, 0.5, 0.0, 0.0, 1.0, 0.0 ,  1.0, 1.0
        ];

        let mut indicies : [i32; 6] = [
          0, 1, 2,
          1, 2, 3
        ];



        gl::GenVertexArrays(1, &mut VAO);
        gl::BindVertexArray(VAO);

        let mut VBO : GLuint = 0;
        gl::GenBuffers(1, &mut VBO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&mut vertices) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW
        );

        let mut EBO : GLuint = 0;
        gl::GenBuffers(1, &mut EBO);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       size_of_val(&mut indicies) as GLsizeiptr,
                       indicies.as_ptr() as *const c_void,
                       gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(0,
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                (8 * std::mem::size_of::<f32>()).try_into().unwrap(),
                                std::ptr::null()

        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1,
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                (8 * std::mem::size_of::<f32>()).try_into().unwrap(),
                                (3 * std::mem::size_of::<f32>()) as *const c_void

        );
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(2,
                                2,
                                gl::FLOAT,
                                gl::FALSE,
                                (8 * std::mem::size_of::<f32>()).try_into().unwrap(),
                                (6 * std::mem::size_of::<f32>()) as *const c_void

        );
        gl::EnableVertexAttribArray(2);

        // setup texture
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);


        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut nrChannels: i32 = 0;
        let p = CString::new("./wall.jpg").unwrap();
        let path: *const c_char = p.as_ptr() as *const c_char;

        let _data : *mut u8 = stbi_load(path, &mut width, &mut height, &mut nrChannels, 0);

        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            width,
            height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            _data as *const u8 as *const c_void
        );
        stbi_image_free(_data as *mut c_void);

        gl::GenerateMipmap(gl::TEXTURE_2D);



    }

    while(!window.should_close()){


        unsafe {
            gl::UseProgram(program);

            gl::BindVertexArray(VAO);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }

        _glfw.poll_events();
        window.swap_buffers();
        for (_, event) in glfw::flush_messages(&events){
            match event {
                glfw::WindowEvent::Key(glfw::Key::Space, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                _ => {}
            }
        }
    }

}
