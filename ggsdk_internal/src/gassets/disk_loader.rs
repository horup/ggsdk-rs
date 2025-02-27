use crate::{Loader, LoaderEvent};
use std::{
    collections::VecDeque,
    fs,
    path::Path,
    thread::{self, JoinHandle},
};

#[derive(Default)]
pub struct DiskLoader {
    requests: VecDeque<String>,
    load_handle: Option<JoinHandle<LoaderEvent>>,
}

impl Loader for DiskLoader {
    fn request(&mut self, path: String) {
        self.requests.push_back(path);
    }

    fn poll(&mut self) -> Option<LoaderEvent> {
        match &self.load_handle {
            Some(handle) => {
                if handle.is_finished() {
                    let handle = self.load_handle.take().unwrap();
                    match handle.join() {
                        Ok(e) => {
                            return Some(e);
                        }
                        Err(_) => {
                            return Some(LoaderEvent::LoadFailed("unknown error".to_string()));
                        }
                    }
                }
            }
            None => {
                if let Some(s) = self.requests.pop_front() {
                    let handle: JoinHandle<LoaderEvent> = thread::spawn(move || {
                        let path = Path::new(&s);
                        
                        match fs::read(path) {
                            Ok(data) => LoaderEvent::Load(s.clone(), data),
                            Err(_) => LoaderEvent::LoadFailed(s.clone()),
                        }
                    });
                    self.load_handle = Some(handle);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::DiskLoader;
    use crate::{Loader, LoaderEvent};

    #[test]
    fn test_disk_assets_io() {
        let mut io = DiskLoader::default();
        io.request("Cargo.toml".into());

        fn poll(io: &mut DiskLoader) -> LoaderEvent {
            loop {
                if let Some(e) = io.poll() {
                    return e;
                }
            }
        }

        match poll(&mut io) {
            LoaderEvent::Load(path, data) => {
                assert_eq!(path, "Cargo.toml");
                assert!(
                    !data.is_empty(),
                    "Data should not be empty for an existing file"
                );
            }
            LoaderEvent::LoadFailed(_) => panic!(),
        }

        io.request("Not found.toml".into());

        match poll(&mut io) {
            LoaderEvent::Load(..) => {
                panic!();
            }
            LoaderEvent::LoadFailed(path) => {
                assert_eq!(path, "Not found.toml");
            },
        }
    }
}
