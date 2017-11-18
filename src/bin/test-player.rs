// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate gecko_media;

use gecko_media::{GeckoMedia, Metadata, PlayerEventSink, PlanarYCbCrImage, TimeStamp};
use std::env;
use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

extern crate gleam;
extern crate glutin;
extern crate time;
extern crate webrender;

use webrender::api::*;

mod ui;

enum PlayerEvent {
    BreakOutOfEventLoop,
    Error,
    Ended,
    AsyncEvent(CString),
    MetadataLoaded(Metadata),
}

struct PlayerWrapper {
    sender: mpsc::Sender<PlayerEvent>,
    ended_receiver: mpsc::Receiver<()>,
}

impl PlayerWrapper {
    pub fn new(bytes: Vec<u8>, mime: &'static str, frame_sender: mpsc::Sender<Vec<PlanarYCbCrImage>>) -> PlayerWrapper {
        let (sender, receiver) = mpsc::channel();
        struct Sink {
            sender: mpsc::Sender<PlayerEvent>,
            frame_sender: mpsc::Sender<Vec<PlanarYCbCrImage>>,
        }
        impl PlayerEventSink for Sink {
            fn playback_ended(&self) {
                self.sender.send(PlayerEvent::Ended).unwrap();
            }
            fn decode_error(&self) {
                self.sender.send(PlayerEvent::Error).unwrap();
            }
            fn async_event(&self, name: &str) {
                self.sender
                    .send(PlayerEvent::AsyncEvent(CString::new(name).unwrap()))
                    .unwrap();
            }
            fn metadata_loaded(&self, metadata: Metadata) {
                self.sender.send(PlayerEvent::MetadataLoaded(metadata)).unwrap();
            }
            fn duration_changed(&self, _duration: f64) {}
            fn loaded_data(&self) {}
            fn time_update(&self, _time: f64) {}
            fn seek_started(&self) {}
            fn seek_completed(&self) {}
            fn update_current_images(&self, images: Vec<PlanarYCbCrImage>) {
                // for img in images.iter() {
                    // let _pixels = img.y_plane.data();
                    // let now = TimeStamp(time::precise_time_ns());
                    // if img.time_stamp > now {
                    //     println!("frame display at {} (now is {})", img.time_stamp, now);
                    // }
                // }
                self.frame_sender.send(images);
            }
        }

        let (ended_sender, ended_receiver) = mpsc::channel();

        let wrapper_sender = sender.clone();
        thread::spawn(move || {
            let sink = Box::new(Sink { sender: sender, frame_sender: frame_sender });
            let player = GeckoMedia::get()
                .unwrap()
                .create_blob_player(bytes, mime, sink)
                .unwrap();
            player.play();
            player.set_volume(1.0);
            loop {
                match receiver.recv().unwrap() {
                    PlayerEvent::Ended => {
                        println!("Ended");
                        break;
                    }
                    PlayerEvent::Error => {
                        println!("Error");
                        break;
                    }
                    PlayerEvent::AsyncEvent(name) => {
                        println!("Event received: {:?}", name);
                    }
                    PlayerEvent::MetadataLoaded(metadata) => {
                        println!("MetadataLoaded; duration: {:?}", metadata.duration);
                    }
                    PlayerEvent::BreakOutOfEventLoop => {
                        break;
                    }
                };
            }
            drop(player);
            ended_sender.send(()).unwrap();
        });
        PlayerWrapper {
            sender: wrapper_sender,
            ended_receiver: ended_receiver,
        }
    }
    pub fn shutdown(&self) {
        self.sender.send(PlayerEvent::BreakOutOfEventLoop).unwrap();
        self.await_ended();
    }
    pub fn await_ended(&self) {
        self.ended_receiver.recv().unwrap();
    }
}

// struct ImageGenerator {
//     frame_receiver: mpsc::Receiver<Vec<PlanarYCbCrImage>>,
//     frame_queue: Vec<PlanarYCbCrImage>,
// }

// impl ImageGenerator {

//     fn generate_image(&mut self, size: u32) {
//         let pattern = &self.patterns[self.next_pattern];
//         self.current_image.clear();
//         for y in 0 .. size {
//             for x in 0 .. size {
//                 let lum = 255 * (1 - (((x & 8) == 0) ^ ((y & 8) == 0)) as u8);
//                 self.current_image.extend_from_slice(&[
//                     lum * pattern[0],
//                     lum * pattern[1],
//                     lum * pattern[2],
//                     0xff,
//                 ]);
//             }
//         }

//         self.next_pattern = (self.next_pattern + 1) % self.patterns.len();
//     }

//     fn take(&mut self) -> Vec<u8> {
//         mem::replace(&mut self.current_image, Vec::new())
//     }
// }

// impl webrender::ExternalImageHandler for ImageGenerator {
//     fn lock(&mut self, _key: ExternalImageId, channel_index: u8) -> webrender::ExternalImage {

//         self.generate_image(channel_index as u32);
//         webrender::ExternalImage {
//             u0: 0.0,
//             v0: 0.0,
//             u1: 1.0,
//             v1: 1.0,
//             source: webrender::ExternalImageSource::RawData(&self.current_image),
//         }
//     }
//     fn unlock(&mut self, _key: ExternalImageId, _channel_index: u8) {}
// }

struct App {
    // image_handler: Option<Box<webrender::ExternalImageHandler>>,
    frame_receiver: mpsc::Receiver<Vec<PlanarYCbCrImage>>,
    frame_queue: Vec<PlanarYCbCrImage>,
}

impl App {
    fn new() -> (App, mpsc::Sender<Vec<PlanarYCbCrImage>>) {
        let (frame_sender, frame_receiver) = mpsc::channel();
        // let handler = Box::new(ImageGenerator { frame_receiver: receiver, frame_queue: vec![] });
        let app = App {
            // image_handler: handler,
            frame_receiver: frame_receiver,
            frame_queue: vec![],
        };
        (app, frame_sender)
    }
}

impl ui::Example for App {
    fn render(
        &mut self,
        api: &RenderApi,
        builder: &mut DisplayListBuilder,
        resources: &mut ResourceUpdates,
        layout_size: LayoutSize,
        _pipeline_id: PipelineId,
        _document_id: DocumentId,
    ) {
        let bounds = LayoutRect::new(LayoutPoint::zero(), layout_size);
        let info = LayoutPrimitiveInfo::new(bounds);
        builder.push_stacking_context(
            &info,
            ScrollPolicy::Scrollable,
            None,
            TransformStyle::Flat,
            None,
            MixBlendMode::Normal,
            Vec::new(),
        );

        if let Ok(v) = self.frame_receiver.try_recv() {
            self.frame_queue = v;
        }

        if self.frame_queue.len() > 0 {
            // Just paint the first of now...
            let frame = &self.frame_queue[0];

            let y_chanel = ImageKey::new(244, 0);
            let u_chanel = ImageKey::new(244, 1);
            let v_chanel = ImageKey::new(244, 2);

            resources.add_image(
                y_chanel,
                ImageDescriptor::new(frame.y_plane.width, frame.y_plane.height, ImageFormat::A8, true),
                ImageData::new(frame.y_plane.data()),
                None,
            );
            resources.add_image(
                u_chanel,
                ImageDescriptor::new(frame.u_plane.width, frame.u_plane.height, ImageFormat::A8, true),
                ImageData::new(frame.u_plane.data()),
                None,
            );
            resources.add_image(
                v_chanel,
                ImageDescriptor::new(frame.v_plane.width, frame.v_plane.height, ImageFormat::A8, true),
                ImageData::new(frame.v_plane.data()),
                None,
            );

            let info = LayoutPrimitiveInfo::with_clip_rect(
                LayoutRect::new(LayoutPoint::new(300.0, 0.0), LayoutSize::new(100.0, 100.0)),
                bounds,
            );
            builder.push_yuv_image(
                &info,
                YuvData::PlanarYCbCr(y_chanel, u_chanel, v_chanel),
                YuvColorSpace::Rec601,
                ImageRendering::Auto,
            );

        }

        builder.pop_stacking_context();
    }

    fn on_event(&mut self, event: glutin::Event, api: &RenderApi, document_id: DocumentId) -> bool {
         false
    }

    // fn get_external_image_handler(&mut self) -> Option<Box<webrender::ExternalImageHandler>> {
    //     mem::replace(self.image_handler, None)
    // }
}


fn main() {
    let args: Vec<_> = env::args().collect();
    let filename: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        panic!("Usage: test-player file_path")
    };

    let path = Path::new(filename);
    let mime = match path.extension().unwrap().to_str() {
        Some("wav") => "audio/wav",
        Some("mp3") => "audio/mp3",
        Some("flac") => "audio/flac",
        Some("ogg") => "audio/ogg; codecs=\"vorbis\"",
        Some("m4a") => "audio/mp4",
        Some("mp4") => "video/mp4",
        _ => "",
    };
    if mime == "" {
        panic!(
            "Unknown file type. Currently supported: wav, mp3, m4a, flac and ogg/vorbis files.\
                Video files supported: mp4."
        )
    }

    let mut file = File::open(filename).unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();

    let (app, frame_sender) = App::new();

    let player = PlayerWrapper::new(bytes, mime, frame_sender);
    player.await_ended();
    GeckoMedia::shutdown().unwrap();
}
