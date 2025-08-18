use bevy::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Initialize logging system
pub fn setup_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,wgpu_core=warn,wgpu_hal=warn,naga=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Logging system initialized");
}

// Log system performance metrics
pub fn log_performance_metrics(
    time: Res<Time>,
    mut last_log: Local<f32>,
) {
    let elapsed = time.elapsed_secs();
    if elapsed - *last_log >= 10.0 { // Log every 10 seconds
        let fps = 1.0 / time.delta_secs();
        debug!("Performance: FPS: {:.1}, Frame time: {:.3}ms", fps, time.delta_secs() * 1000.0);
        *last_log = elapsed;
    }
}