use std::time::Instant;

use tracing::info;

pub struct StartupTimer {
    started: Instant,
}

impl StartupTimer {
    pub fn start() -> Self {
        info!("boot: init");
        Self {
            started: Instant::now(),
        }
    }

    pub fn finish(self) {
        info!("boot: ready in {}ms", self.started.elapsed().as_millis());
    }
}
