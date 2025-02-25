use std::collections::HashMap;

use futures::channel::oneshot;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{js_sys, Request, RequestInit, Response};

use crate::{Loader, LoaderEvent};

#[derive(Default)]
pub struct WebLoader {
    pending_requests: HashMap<String, oneshot::Receiver<LoaderEvent>>,
    new_requests: Vec<String>,
}

impl WebLoader {
    pub fn new() -> Self {
        Self {
            pending_requests: Default::default(),
            new_requests: Default::default(),
        }
    }

    async fn load_asset_async(path: String) -> LoaderEvent {
        let opts = RequestInit::new();
        opts.set_method("GET");

        // Create a new Request with the specified URL
        let request = Request::new_with_str_and_init(&path, &opts).unwrap();

        // Perform the fetch request
        let window = web_sys::window().unwrap();
        let response_js_value = match JsFuture::from(window.fetch_with_request(&request)).await {
            Ok(resp) => resp,
            Err(_) => return LoaderEvent::LoadFailed(path),
        };

        let response: Response = response_js_value.dyn_into().unwrap();

        // Check if the response is successful
        if !response.ok() {
            return LoaderEvent::LoadFailed(path);
        }

        // Get the response body as an array buffer
        let array_buffer_promise = response.array_buffer().unwrap();
        let array_buffer_js_value = match JsFuture::from(array_buffer_promise).await {
            Ok(buffer) => buffer,
            Err(_) => return LoaderEvent::LoadFailed(path),
        };

        let array_buffer = js_sys::Uint8Array::new(&array_buffer_js_value);
        let data = array_buffer.to_vec();

        LoaderEvent::Load(path, data)
    }

    fn spawn_asset_load(&mut self, path: String, sender: oneshot::Sender<LoaderEvent>) {
        // Spawn the async asset loading task
        spawn_local(async move {
            let event = WebLoader::load_asset_async(path).await;
            let _ = sender.send(event);
        });
    }
}

impl Loader for WebLoader {
    fn request(&mut self, path: String) {
        self.new_requests.push(path);
    }

    fn poll(&mut self) -> Option<LoaderEvent> {
        let mut processed = None;
        let mut event = None;

        if let Some(path) = self.new_requests.pop() {
            let (sender, receiver) = oneshot::channel();
            self.spawn_asset_load(path.clone(), sender);
            self.pending_requests.insert(path, receiver);
        }

        for (path, receiver) in self.pending_requests.iter_mut() {
            match receiver.try_recv() {
                Ok(e) => {
                    if let Some(e) = e {
                        event = Some(e);
                        processed = Some(path.clone());
                        break;
                    }
                }
                Err(_) => {
                    event = Some(LoaderEvent::LoadFailed(path.to_owned()));
                    processed = Some(path.to_string());
                    break;
                }
            }
        }

        if let Some(processed) = processed {
            self.pending_requests.remove(&processed);
        }

        event
    }
}
