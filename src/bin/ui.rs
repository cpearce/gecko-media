/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// Copied from:
// https://github.com/servo/webrender/blob/58ab0f136885b33386ee85878939e3384e2b1fba/webrender/examples/common/boilerplate.rs

use gleam::gl;
use glutin;
use webrender;
use webrender::api::*;

struct Notifier {
    window_proxy: glutin::WindowProxy,
}

impl Notifier {
    fn new(window_proxy: glutin::WindowProxy) -> Notifier {
        Notifier { window_proxy }
    }
}

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<RenderNotifier> {
        Box::new(Notifier {
            window_proxy: self.window_proxy.clone(),
        })
    }

    fn new_frame_ready(&self) {
        #[cfg(not(target_os = "android"))]
        self.window_proxy.wakeup_event_loop();
    }

    fn new_scroll_frame_ready(&self, _composite_needed: bool) {
        #[cfg(not(target_os = "android"))]
        self.window_proxy.wakeup_event_loop();
    }
}

pub trait HandyDandyRectBuilder {
    fn to(&self, x2: i32, y2: i32) -> LayoutRect;
    fn by(&self, w: i32, h: i32) -> LayoutRect;
}
// Allows doing `(x, y).to(x2, y2)` or `(x, y).by(width, height)` with i32
// values to build a f32 LayoutRect
impl HandyDandyRectBuilder for (i32, i32) {
    fn to(&self, x2: i32, y2: i32) -> LayoutRect {
        LayoutRect::new(
            LayoutPoint::new(self.0 as f32, self.1 as f32),
            LayoutSize::new((x2 - self.0) as f32, (y2 - self.1) as f32),
        )
    }

    fn by(&self, w: i32, h: i32) -> LayoutRect {
        LayoutRect::new(
            LayoutPoint::new(self.0 as f32, self.1 as f32),
            LayoutSize::new(w as f32, h as f32),
        )
    }
}

pub trait Example {
    fn render(
        &mut self,
        api: &RenderApi,
        builder: &mut DisplayListBuilder,
        resources: &mut ResourceUpdates,
        layout_size: LayoutSize,
        pipeline_id: PipelineId,
        document_id: DocumentId,
    );
    fn on_event(&mut self, event: glutin::Event, api: &RenderApi, document_id: DocumentId) -> bool;
    fn get_external_image_handler(&mut self) -> Option<Box<webrender::ExternalImageHandler>> {
        None
    }
    fn get_output_image_handler(
        &mut self,
        _gl: &gl::Gl,
    ) -> Option<Box<webrender::OutputImageHandler>> {
        None
    }
    fn init(&mut self, window_proxy: glutin::WindowProxy) {}
}

pub fn main_wrapper(example: &mut Example, options: Option<webrender::RendererOptions>) {
    let res_path = None;

    let window = glutin::WindowBuilder::new()
        .with_title("Gecko Media Player")
        .with_multitouch()
        .with_gl(glutin::GlRequest::GlThenGles {
            opengl_version: (3, 2),
            opengles_version: (3, 0),
        })
        .build()
        .unwrap();

    unsafe {
        window.make_current().ok();
    }

    let gl = match gl::GlType::default() {
        gl::GlType::Gl => unsafe {
            gl::GlFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
        gl::GlType::Gles => unsafe {
            gl::GlesFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
    };

    println!("OpenGL version {}", gl.get_string(gl::VERSION));
    println!("Shader resource path: {:?}", res_path);

    let (width, height) = window.get_inner_size_pixels().unwrap();

    let opts = webrender::RendererOptions {
        resource_override_path: res_path,
        debug: true,
        precache_shaders: true,
        device_pixel_ratio: window.hidpi_factor(),
        ..options.unwrap_or(webrender::RendererOptions::default())
    };

    let size = DeviceUintSize::new(width, height);
    let notifier = Box::new(Notifier::new(window.create_window_proxy()));
    let (mut renderer, sender) = webrender::Renderer::new(gl.clone(), notifier, opts).unwrap();
    let api = sender.create_api();
    let document_id = api.add_document(size);

    example.init(window.create_window_proxy());

    if let Some(external_image_handler) = example.get_external_image_handler() {
        renderer.set_external_image_handler(external_image_handler);
    }
    if let Some(output_image_handler) = example.get_output_image_handler(&*gl) {
        renderer.set_output_image_handler(output_image_handler);
    }

    let epoch = Epoch(0);
    let root_background_color = ColorF::new(0.3, 0.0, 0.0, 1.0);

    let pipeline_id = PipelineId(0, 0);
    let layout_size = LayoutSize::new(width as f32, height as f32);
    let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);
    let mut resources = ResourceUpdates::new();

    example.render(
        &api,
        &mut builder,
        &mut resources,
        layout_size,
        pipeline_id,
        document_id,
    );
    api.set_display_list(
        document_id,
        epoch,
        Some(root_background_color),
        LayoutSize::new(width as f32, height as f32),
        builder.finalize(),
        true,
        resources,
    );
    api.set_root_pipeline(document_id, pipeline_id);
    api.generate_frame(document_id, None);

    'outer: for event in window.wait_events() {
        println!("Event loop!");
        let mut events = Vec::new();
        events.push(event);

        for event in window.poll_events() {
            events.push(event);
        }

        for event in events {
            match event {
                glutin::Event::Closed |
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) => break 'outer,

                _ => if example.on_event(event, &api, document_id) {
                    let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);
                    let mut resources = ResourceUpdates::new();

                    example.render(
                        &api,
                        &mut builder,
                        &mut resources,
                        layout_size,
                        pipeline_id,
                        document_id,
                    );
                    api.set_display_list(
                        document_id,
                        epoch,
                        Some(root_background_color),
                        LayoutSize::new(width as f32, height as f32),
                        builder.finalize(),
                        true,
                        resources,
                    );
                    api.generate_frame(document_id, None);
                },
            }
        }

        renderer.update();
        renderer.render(DeviceUintSize::new(width, height)).unwrap();
        window.swap_buffers().ok();
    }

    renderer.deinit();
}
