extern crate glfw;
extern crate num_traits;

pub mod gl;

pub mod kat {
    use std::borrow::BorrowMut;
    use std::ffi::c_void;
    use std::marker::PhantomData;
    use std::ptr::null;
    use glfw;
    use glfw::{Context, Glfw, WindowMode};
    use num_traits::Num;
    use crate::gl;
    use crate::gl::types::*;

    pub struct Window {
        win: glfw::Window,
        events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>
    }

    pub struct Color {
        r: f32,
        g: f32,
        b: f32,
        a: f32
    }

    pub mod colors {
        use crate::kat::Color;

        pub const RED: Color = Color::create(1.0, 0.0, 0.0, 1.0);
        pub const GREEN: Color = Color::create(0.0, 1.0, 0.0, 1.0);
        pub const BLUE: Color = Color::create(0.0, 0.0, 1.0, 1.0);
        pub const YELLOW: Color = Color::create(1.0, 1.0, 0.0, 1.0);
        pub const MAGENTA: Color = Color::create(1.0, 0.0, 1.0, 1.0);
        pub const CYAN: Color = Color::create(0.0, 1.0, 1.0, 1.0);
        pub const WHITE: Color = Color::create(1.0, 1.0, 1.0, 1.0);
        pub const BLACK: Color = Color::create(0.0, 0.0, 0.0, 1.0);
    }

    impl Color {
        pub const fn create(r: f32, g: f32, b: f32, a: f32) -> Color {
            Color{r, g, b, a}
        }
    }

    pub struct Engine {
        glfw_ctx: Glfw
    }

    impl Engine {
        pub fn create() -> Engine {
            Engine {
                glfw_ctx: glfw::init(glfw::FAIL_ON_ERRORS).unwrap()
            }
        }

        pub fn new_window(&mut self, width: u32, height: u32, title: &str) -> Window {
            let (mut win, events) = self.glfw_ctx.create_window(width, height, title, WindowMode::Windowed)
                .expect("Failed to create window.");

            win.set_all_polling(true);

            let mut window: Window = Window {
                win, events
            };

            load_gl(window.borrow_mut());

            return window;
        }

        pub fn clear(&self, color: &Color) {
            unsafe {
                gl::ClearColor(color.r, color.g, color.b, color.a);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            }
        }

        pub fn clear_default(&self) {
            self.clear(&colors::BLACK)
        }
    }

    pub fn load_gl(win: &mut Window) {
        win.win.make_current();
        gl::load_with(|s| win.win.get_proc_address(s) as *const _);
    }

    impl Window {

        pub fn is_open(&self) -> bool {
            !self.win.should_close()
        }

        pub fn update_events<F>(&mut self, engine: &mut Engine, mut f: F) where F: FnMut(&mut glfw::Window, glfw::WindowEvent) {
            engine.glfw_ctx.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                f(self.win.borrow_mut(), event);
            }
        }

        pub fn swap(&mut self) {
            self.win.swap_buffers();
        }
    }

    pub trait Bindable {
        fn bind(&mut self);
    }

    #[derive(Clone,Copy,PartialEq,Eq,Hash)]
    pub enum BufferTarget {
        Array,
        ElementArray
    }

    #[derive(Clone,Copy,PartialEq,Eq,Hash)]
    pub enum DrawMode {
        Triangles,
        TriangleFan,
        Lines,
        TriangleStrip,
        Patches,
        LineStrip,
        LineLoop,
        Points
    }


    pub const fn translate_buffer_target(target: BufferTarget) -> u32 {
        match target {
            BufferTarget::Array => { gl::ARRAY_BUFFER }
            BufferTarget::ElementArray => { gl::ELEMENT_ARRAY_BUFFER }
        }
    }

    pub const fn translate_draw_mode(mode: DrawMode) -> u32 {
        match mode {
            DrawMode::Triangles => { gl::TRIANGLES }
            DrawMode::TriangleFan => { gl::TRIANGLE_FAN }
            DrawMode::Lines => { gl::LINES }
            DrawMode::TriangleStrip => { gl::TRIANGLE_STRIP }
            DrawMode::Patches => { gl::PATCHES }
            DrawMode::LineStrip => { gl::LINE_STRIP }
            DrawMode::LineLoop => { gl::LINE_LOOP }
            DrawMode::Points => { gl::POINTS }
        }
    }


    pub struct Buffer<T: Num> {
        handle: u32,
        size: usize,
        target: BufferTarget,
        _phantom: PhantomData<T>
    }

    impl <T: Num> Buffer<T> {
        pub fn create_null(target: BufferTarget) -> Buffer<T> {
            Self::create_blank_sized(target, 0)
        }

        pub fn create_blank_sized(target: BufferTarget, size: usize) -> Buffer<T> {
            let mut i: u32 = 0;
            unsafe {
                gl::CreateBuffers(1, &mut i);
                gl::NamedBufferData(i, (size * std::mem::size_of::<T>()) as GLsizeiptr, null(), gl::DYNAMIC_DRAW);
            }
            return Buffer::<T>{
                handle: i,
                size,
                target,
                _phantom: Default::default()
            }
        }

        pub fn create(target: BufferTarget, data: Vec<T>) -> Buffer<T> {
            let mut i: u32 = 0;
            unsafe {
                gl::CreateBuffers(1, &mut i);
                let l: GLsizeiptr = (data.len() * std::mem::size_of::<T>()) as GLsizeiptr;
                gl::NamedBufferData(i, l, data.as_ptr() as *const c_void, gl::STATIC_DRAW);
            }

            return Buffer::<T>{
                handle: i,
                size: data.len(),
                target,
                _phantom: Default::default()
            }
        }
    }

    pub struct VertexArray {
        handle: u32,
        nextAttrib: usize,
        nextBinding: usize
    }

    impl VertexArray {
        pub fn new() -> VertexArray {
            let mut i: u32 = 0;
            unsafe { gl::CreateVertexArrays(1, &mut i); }
            return VertexArray {
                handle: i,
                nextAttrib: 0,
                nextBinding: 0
            }
        }

        pub fn vertex_buffer(&mut self, buf: &mut Buffer<f32>, attribs: Vec<usize>) {
            let mut stride: usize = 0;
            buf.bind();

            for a in attribs {
                unsafe {
                    gl::VertexArrayAttribFormat(self.handle, self.nextAttrib as GLuint, a as GLint, gl::FLOAT, gl::FALSE, stride as GLuint);
                    gl::VertexArrayAttribBinding(self.handle, self.nextAttrib as GLuint, self.nextBinding as GLuint);
                }

                stride += a * std::mem::size_of::<f32>();
                self.nextAttrib += 1;
            }

            unsafe { gl::VertexArrayVertexBuffer(self.handle, self.nextBinding as GLuint, buf.handle, 0, stride as GLsizei); }
            self.nextBinding += 1;
        }

        pub fn element_buffer(&mut self, buf: &mut Buffer<u32>) {
            buf.bind();
            unsafe {
                gl::VertexArrayElementBuffer(self.handle, buf.handle);
            }
        }

        pub fn draw_arrays(&mut self, mode: DrawMode, count: usize, start: usize) {
            self.bind();
            unsafe {
                gl::DrawArrays(translate_draw_mode(mode), start as GLint, count as GLint);
            }
        }
    }

    impl <T: Num> Bindable for Buffer<T> {
        fn bind(&mut self) {
            unsafe { gl::BindBuffer(translate_buffer_target(self.target), self.handle as GLuint); }
        }
    }

    impl Bindable for VertexArray {
        fn bind(&mut self) {
            unsafe { gl::BindVertexArray(self.handle); }
        }
    }
}