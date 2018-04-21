extern crate luminance;
extern crate luminance_glfw;

use luminance_glfw::{Action, Device, Key, WindowDim, WindowOpt, WindowEvent, GLFWDevice};
use luminance::*;
use luminance::tess::{Mode, Tess, TessVertices};
use luminance::shader::program::Program;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::{entry, pipeline, RenderState};

const SCREEN_WIDTH: u32 = 960;
const SCREEN_HEIGHT: u32 = 540;

type Position = [f32; 2];
type RGB = [f32; 3];
type Vertex = (Position, RGB);

const TRIANGLE_VERTS: [Vertex; 3] = [
    ([-0.5, -0.5], [0.8, 0.5, 0.5]), // red bottom leftmost
    ([-0., 0.5], [0.5, 0.8, 0.5]), // green top
    ([0.5, -0.5], [0.5, 0.5, 0.8]) // blue bottom rightmost
];

const SHADER_VS: &str = include_str!("./shaders/vs.glsl");
const SHADER_FS: &str = include_str!("./shaders/fs.glsl");

fn app(dev: &mut GLFWDevice) {
    let triangle = Tess::new(Mode::Triangle, TessVertices::Fill(&TRIANGLE_VERTS), None);

    let (shader, warnings) = Program::<Vertex, (), ()>::from_strings(None, SHADER_VS, None, SHADER_FS).unwrap();

    for warning in &warnings {
        eprintln!("{:#?}", warning);
    }

    let rdr_state = RenderState::default();

    let fb = Framebuffer::default([SCREEN_WIDTH, SCREEN_HEIGHT]);

    'app: loop {
        for ev in dev.events() {
            match ev {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
                _ => ()
            }
        }

        dev.draw(|| {
            entry(|_| {
                pipeline(&fb, [0., 0., 0., 1.], |shd_gate| {
                    shd_gate.shade(&shader, |rdr_gate, _| {
                        rdr_gate.render(rdr_state, |tess_gate| {
                            let t = (&triangle).into();
                            tess_gate.render(t);
                        });
                    });
                });
            });
        });
    }
}

fn main() {
    let rdev: Result<GLFWDevice, luminance_glfw::GLFWDeviceError> = Device::new(WindowDim::Windowed(SCREEN_WIDTH, SCREEN_HEIGHT), "lumitest", WindowOpt::default());
    match rdev {
        Err(e) => {
            eprintln!("{:#?}", e);
            ::std::process::exit(1);
        }

        Ok(mut dev) => {
            println!("Window created successfully.");
            app(&mut dev)
        }
    }
}