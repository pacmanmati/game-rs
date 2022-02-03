use std::{borrow::BorrowMut, time::Duration, time::Instant};

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use wgpu::{util::DeviceExt, *};

use std::cell::RefCell;
use std::rc::Rc;

mod cube;
use cube::Cube;

mod camera;
use camera::Camera;

mod renderer;
// use renderer::scene::Scene;
use renderer::Renderer;

use cgmath::*;

/**
 * let's draw a cube the nasty way then make nice helpers.
 * Return a closure we can call to render the cube.
 */
fn draw_rotating_cube(renderer: Rc<RefCell<Renderer>>) -> impl FnMut() {
    let cube = Cube::new();
    // we probably want the shader to take as input:
    // * cube positions
    // * cube colours
    // * cube indices
    // * view matrix
    // * projection matrix
    // * model matrix we use to rotate

    // println!("{:?}", cube.interleaved());

    let cube_model = Matrix4::<f32>::one();
    println!("{:?}", cube_model);
    let aspect = renderer.borrow().config.width as f32 / renderer.borrow().config.height as f32;
    let camera = Camera::new(30.0, aspect, 0.1, 100.0);
    println!("{:?}", camera.vp());

    let cube_model_buffer =
        renderer
            .borrow()
            .device
            .create_buffer_init(&util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&Into::<[[f32; 4]; 4]>::into(cube_model)),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });

    println!("{:?}", cube_model_buffer);

    let camera_buffer = renderer
        .borrow()
        .device
        .create_buffer_init(&util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&Into::<[[f32; 4]; 4]>::into(camera.vp())),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

    let vertex_buffer = renderer
        .borrow()
        .device
        .create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&cube.interleaved()),
            usage: BufferUsages::VERTEX,
        });

    let index_buffer = renderer
        .borrow()
        .device
        .create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&cube.indices),
            usage: BufferUsages::INDEX,
        });

    let camera_bind_group_layout =
        renderer
            .borrow()
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

    let model_bind_group_layout =
        renderer
            .borrow()
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

    let camera_bind_group = renderer
        .borrow()
        .device
        .create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

    let model_bind_group = renderer
        .borrow()
        .device
        .create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &model_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: cube_model_buffer.as_entire_binding(),
            }],
        });

    let shader = renderer
        .borrow()
        .device
        .create_shader_module(&ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

    let layout = renderer
        .borrow()
        .device
        .create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &model_bind_group_layout],
            push_constant_ranges: &[],
        });

    let pipeline = renderer
        .borrow()
        .device
        .create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vert_main",
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<[u32; 3]>() as u64 * 2,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "frag_main",
                targets: &[ColorTargetState {
                    format: renderer.borrow().config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                }],
            }),
            multiview: None,
        });

    let mut rotation_y = 0.0;

    move || {
        let mut encoder =
            renderer
                .borrow()
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        let texture = renderer.borrow().surface.get_current_texture().unwrap();

        let view = texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&pipeline);
            rpass.set_bind_group(0, &camera_bind_group, &[]);
            rpass.set_bind_group(1, &model_bind_group, &[]);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
            rpass.draw_indexed(0..cube.indices.len() as u32, 0, 0..1);
        }

        rotation_y += 1.0;
        let new_model = Matrix4::from_angle_y(Deg(rotation_y)) * cube_model;
        renderer.borrow().queue.write_buffer(
            &cube_model_buffer,
            0,
            bytemuck::cast_slice(&Into::<[[f32; 4]; 4]>::into(new_model)),
        );

        renderer
            .borrow()
            .queue
            .submit(std::iter::once(encoder.finish()));
        texture.present();
    }
}

fn update() {}

/**
 * todo:
 * - a drawable object that contains the geometry and any material / shader related stuff
 * - design something for now, you can always refactor later
 */

fn main() {
    env_logger::init();

    run_app();
}

fn run_app() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("window")
        .build(&event_loop)
        .unwrap();
    let renderer = Rc::new(RefCell::new(Renderer::new(
        &window,
        window.inner_size().height,
        window.inner_size().width,
    )));

    let framerate = 60.0;
    let mut last_frametime = Instant::now();
    let (mut frame_count, mut accum_time) = (0, 0.0);

    let mut ctx = egui::CtxRef::default();
    let mut text = "Hello world!";

    {
        let mut render = draw_rotating_cube(Rc::clone(&renderer));

        event_loop.run(move |event, _, control_flow| {
            // *control_flow = ControlFlow::Wait;
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(PhysicalSize { width, height }) => {
                        (*renderer).borrow_mut().resize_surface(height, width)
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size: PhysicalSize { width, height },
                    } => (*renderer).borrow_mut().resize_surface(*height, *width),
                    _ => (),
                },
                // WindowEvent::Resized(PhysicalSize { width, height }) => renderer.resize(),
                Event::RedrawRequested(_) => {
                    accum_time += last_frametime.elapsed().as_secs_f64();
                    last_frametime = Instant::now();
                    frame_count += 1;

                    if frame_count == 100 {
                        println!("FPS: {}", frame_count as f64 / accum_time);
                        frame_count = 0;
                        accum_time = 0.0;
                    }
                    update();

                    let (output, shapes) = ctx.run(egui::RawInput::default(), |ctx| {
                        egui::CentralPanel::default().show(&ctx, |ui| {
                            ui.label(text);
                            if ui.button("Click me").clicked() {
                                text = "Goodbye world!";
                            }
                        });
                    });

                    let clipped_meshes = ctx.tessellate(shapes);
                    // renderer.borrow().render_ui(clipped_meshes);
                    render();
                }
                Event::RedrawEventsCleared => {
                    let time_elapsed = last_frametime.elapsed();
                    let wait_time = Duration::from_secs_f64(1.0 / framerate);

                    if time_elapsed >= wait_time {
                        window.request_redraw();
                        // println!("Drawing after {} ms", time_elapsed.as_millis());
                    } else {
                        *control_flow =
                            ControlFlow::WaitUntil(Instant::now() + wait_time - time_elapsed);
                        // println!("Waiting {}ms", (wait_time - time_elapsed).as_millis());
                    }
                }
                _ => (),
            }
        });
    }
}
