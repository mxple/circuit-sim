use profiler::Profiler;
mod profiler;

pub fn profile_start(name: &str) {
    Profiler::global().lock().unwrap().start(name);
}

pub fn profile_end(name: &str) {
    Profiler::global().lock().unwrap().end(name);
}

pub fn profile_update(ctx: &egui::Context) {
    Profiler::global().lock().unwrap().update(ctx);
}

pub struct ProfileScope {
    name: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl ProfileScope {
    pub fn new(name: &str) -> Self {
        profile_start(name);
        Self {
            name: name.to_string(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ProfileScope {
    fn drop(&mut self) {
        profile_end(&self.name);
    }
}

#[cfg(target_arch = "wasm32")]
impl ProfileScope {
    pub fn new(_name: &str) -> Self {
        Self {
            name: String::new(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for ProfileScope {
    fn drop(&mut self) {}
}

#[macro_export]
macro_rules! profile_scope {
    ($name:expr) => {
        let _profiler_scope = $crate::profiler::ProfileScope::new($name);
    };
}
