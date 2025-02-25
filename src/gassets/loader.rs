#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub enum LoaderEvent {
    Load(String, Vec<u8>),
    LoadFailed(String)
}

#[cfg_attr(test, automock)]
pub trait Loader : 'static {
    fn request(&mut self, path:String);
    fn poll(&mut self) -> Option<LoaderEvent>;
}
