// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use CanPlayType;
use bindings::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::ops::Drop;
use std::os::raw::c_void;
use std::slice;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::mpsc::{self, Sender};
use std::thread::Builder;

pub struct GeckoMedia {
    sender: Sender<GeckoMediaMsg>,
}

pub struct Player {
    gecko_media: GeckoMedia,
    id: usize,
}

pub trait PlayerEventSink {
    fn playback_ended(&self);
    fn decode_error(&self);
    fn async_event(&self, name: &str);
    fn metadata_loaded(&self);
    fn loaded_data(&self);
    fn time_update(&self, time: f64);
    fn seek_started(&self);
    fn seek_completed(&self);
}

impl Player {
    pub fn load_blob(&self, media_data: Vec<u8>, mime_type: &str) -> Result<(), ()> {
        let media_data = to_ffi_vec(media_data);
        let mime_type = match CString::new(mime_type.as_bytes()) {
            Ok(mime_type) => mime_type,
            _ => return Err(()),
        };
        self.gecko_media.queue_task(move || unsafe {
            GeckoMedia_Player_LoadBlob(self.id, media_data, mime_type.as_ptr());
        });
        Ok(())
    }
    pub fn play(&self) {
        self.gecko_media.queue_task(move || unsafe {
            GeckoMedia_Player_Play(self.id);
        });
    }
    pub fn pause(&self) {
        self.gecko_media.queue_task(move || unsafe {
            GeckoMedia_Player_Pause(self.id);
        });
    }
    pub fn shutdown(&self) {
        self.gecko_media.queue_task(move || unsafe {
            GeckoMedia_Player_Shutdown(self.id);
        });
    }
    pub fn set_volume(&self, volume: f64) {
        self.gecko_media.queue_task(move || unsafe {
            GeckoMedia_Player_SetVolume(self.id, volume);
        });
    }
    pub fn get_duration(&self) -> f64 {
        self.gecko_media.get_duration(self.id)
    }
}

impl GeckoMedia {
    pub fn get() -> Result<Self, ()> {
        OUTSTANDING_HANDLES.fetch_add(1, Ordering::SeqCst);
        let sender = SENDER.lock().unwrap();
        match sender.clone() {
            Some(sender) => Ok(GeckoMedia { sender }),
            None => {
                OUTSTANDING_HANDLES.fetch_sub(1, Ordering::SeqCst);
                Err(())
            }
        }
    }

    pub fn shutdown() -> Result<(), ()> {
        let mut sender = SENDER.lock().unwrap();
        if OUTSTANDING_HANDLES.load(Ordering::SeqCst) > 0 {
            return Err(());
        }
        if let Some(ref sender) = *sender {
            let (ok_sender, ok_receiver) = mpsc::channel();
            let _ = sender.send(GeckoMediaMsg::Exit(ok_sender));
            ok_receiver.recv().unwrap();
        }
        *sender = None;
        Ok(())
    }

    pub fn can_play_type(&self, mime_type: &str) -> CanPlayType {
        if let Ok(mime_type) = CString::new(mime_type.as_bytes()) {
            let (sender, receiver) = mpsc::channel();
            self.sender
                .send(GeckoMediaMsg::CanPlayType(mime_type, sender))
                .unwrap();
            receiver.recv().unwrap()
        } else {
            CanPlayType::No
        }
    }

    pub fn get_duration(&self, player_id: usize) -> f64 {
        let (sender, receiver) = mpsc::channel();
        self.sender
            .send(GeckoMediaMsg::DurationQuery(player_id, sender))
            .unwrap();
        receiver.recv().unwrap()
    }

    pub fn queue_task<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        unsafe extern "C" fn call<F>(ptr: *mut c_void)
        where
            F: FnOnce(),
        {
            Box::from_raw(ptr as *mut F)()
        }

        let runnable = RustRunnable {
            mData: Box::into_raw(Box::new(f)) as *mut c_void,
            mFunction: Some(call::<F>),
        };

        unsafe { GeckoMedia_QueueRustRunnable(runnable) }
    }

    pub fn create_player(&self, sink: Box<PlayerEventSink>) -> Result<Player, ()> {
        let handle = GeckoMedia::get()?;
        let id = NEXT_PLAYER_ID.fetch_add(1, Ordering::SeqCst);

        let callback = self.to_ffi_callback(sink);
        self.queue_task(move || unsafe {
            GeckoMedia_Player_Create(id, callback);
        });

        Ok(Player {
            gecko_media: handle,
            id,
        })
    }

    fn to_ffi_callback(&self, sink: Box<PlayerEventSink>) -> PlayerCallbackObject {
        // Can't cast from *c_void to a Trait, so wrap in a concrete type
        // when we pass into C++ code.
        struct Wrapper {
            sink: Box<PlayerEventSink>,
        }
        unsafe extern "C" fn free(ptr: *mut c_void) {
            drop(Box::from_raw(ptr as *mut Wrapper));
        }
        unsafe extern "C" fn decode_error(ptr: *mut c_void) {
            let wrapper = &*(ptr as *mut Wrapper);
            wrapper.sink.decode_error();
        }
        unsafe extern "C" fn playback_ended(ptr: *mut c_void) {
            let wrapper = &*(ptr as *mut Wrapper);
            wrapper.sink.playback_ended();
        }
        unsafe extern "C" fn async_event(ptr: *mut c_void, name: *const i8) {
            let wrapper = &*(ptr as *mut Wrapper);
            let c_str: &CStr = CStr::from_ptr(name);
            wrapper.sink.async_event(c_str.to_str().unwrap());
        }
        unsafe extern "C" fn metadata_loaded(ptr: *mut c_void) {
            let wrapper = &*(ptr as *mut Wrapper);
            wrapper.sink.metadata_loaded();
        }
        unsafe extern "C" fn loaded_data(ptr: *mut c_void) {
            let wrapper = &*(ptr as *mut Wrapper);
            wrapper.sink.loaded_data();
        }
        unsafe extern "C" fn seek_started(ptr: *mut c_void) {
            let wrapper = &*(ptr as *mut Wrapper);
            wrapper.sink.seek_started();
        }
        unsafe extern "C" fn seek_completed(ptr: *mut c_void) {
            let wrapper = &*(ptr as *mut Wrapper);
            wrapper.sink.seek_completed();
        }
        unsafe extern "C" fn time_update(ptr: *mut c_void, time: f64) {
            let wrapper = &*(ptr as *mut Wrapper);
            wrapper.sink.time_update(time);
        }
        PlayerCallbackObject {
            mContext: Box::into_raw(Box::new(Wrapper { sink: sink })) as *mut c_void,
            mPlaybackEnded: Some(playback_ended),
            mDecodeError: Some(decode_error),
            mAsyncEvent: Some(async_event),
            mMetadataLoaded: Some(metadata_loaded),
            mLoadedData: Some(loaded_data),
            mSeekStarted: Some(seek_started),
            mSeekCompleted: Some(seek_completed),
            mTimeUpdate: Some(time_update),
            mFree: Some(free),
        }
    }

    #[cfg(test)]
    pub fn test(&self) {
        self.sender.send(GeckoMediaMsg::Test).unwrap();
    }
}

impl Drop for GeckoMedia {
    fn drop(&mut self) {
        OUTSTANDING_HANDLES.fetch_sub(1, Ordering::SeqCst);
    }
}

enum GeckoMediaMsg {
    Exit(Sender<()>),
    CanPlayType(CString, Sender<CanPlayType>),
    #[cfg(test)]
    Test,
    CallProcessGeckoEvents,
    DurationQuery(usize, Sender<f64>),
}

static OUTSTANDING_HANDLES: AtomicUsize = ATOMIC_USIZE_INIT;
static NEXT_PLAYER_ID: AtomicUsize = ATOMIC_USIZE_INIT;

lazy_static! {
    static ref SENDER: Mutex<Option<Sender<GeckoMediaMsg>>> = {
        let (msg_sender, msg_receiver) = mpsc::channel();
        let (ok_sender, ok_receiver) = mpsc::channel();
        let msg_sender_clone = msg_sender.clone();
        Builder::new().name("GeckoMedia".to_owned()).spawn(move || {
            let thread_observer_object = thread_observer_object(msg_sender_clone);
            assert!(
                unsafe { GeckoMedia_Initialize(thread_observer_object) },
                "failed to initialize GeckoMedia"
            );
            ok_sender.send(()).unwrap();
            drop(ok_sender);
            loop {
                match msg_receiver.recv().unwrap() {
                    GeckoMediaMsg::Exit(sender) => {
                        unsafe { GeckoMedia_Shutdown() };
                        sender.send(()).unwrap();
                        break;
                    },
                    GeckoMediaMsg::CanPlayType(mime_type, sender) => {
                        unsafe {
                            sender.send(GeckoMedia_CanPlayType(mime_type.as_ptr())).unwrap();
                        }
                    },
                    GeckoMediaMsg::CallProcessGeckoEvents => {
                        // Process any pending messages in Gecko's main thread
                        // event queue.
                        unsafe {
                            GeckoMedia_ProcessEvents();
                        }
                    },
                    GeckoMediaMsg::DurationQuery(player_id, sender) => {
                        unsafe {
                            sender.send(GeckoMedia_Player_GetDuration(player_id)).unwrap();
                        }
                    },
                    #[cfg(test)]
                    GeckoMediaMsg::Test => {
                        extern "C" { fn TestGecko(); }
                        unsafe { TestGecko(); }
                    }
                }
            }
        }).unwrap();
        ok_receiver.recv().unwrap();
        Mutex::new(Some(msg_sender))
    };
}

fn thread_observer_object(sender: Sender<GeckoMediaMsg>) -> ThreadObserverObject {
    unsafe extern "C" fn on_dispatched_event(ptr: *mut c_void) {
        let sender = &*(ptr as *const Sender<GeckoMediaMsg>);
        sender.send(GeckoMediaMsg::CallProcessGeckoEvents).unwrap();
    }

    unsafe extern "C" fn free(ptr: *mut c_void) {
        drop(Box::from_raw(ptr as *mut Sender<GeckoMediaMsg>));
    }

    static VTABLE: ThreadObserverVtable = ThreadObserverVtable {
        mOnDispatchedEvent: Some(on_dispatched_event),
        mFree: Some(free),
    };

    ThreadObserverObject {
        mData: Box::into_raw(Box::new(sender)) as *mut c_void,
        mVtable: &VTABLE,
    }
}

fn to_ffi_vec(bytes: Vec<u8>) -> RustVecU8Object {
    unsafe extern "C" fn free(ptr: *mut u8, len: usize) {
        let ptr = slice::from_raw_parts_mut(ptr, len) as *mut [u8];
        drop(Box::from_raw(ptr));
    }
    let mut bytes = bytes.into_boxed_slice();
    let data = bytes.as_mut_ptr();
    let len = bytes.len();
    mem::forget(bytes);

    RustVecU8Object {
        mData: data,
        mLength: len,
        mFree: Some(free),
    }
}
