use {
    lazy_static::lazy_static,
    tokio::sync::watch,
};

mod api;
mod config;
mod metrics_server;
mod network;
mod serde;
mod state;

lazy_static! {
    /// A static exit flag to indicate to running threads that we're shutting down. This is used to
    /// gracefully shut down the application.
    ///
    /// We make this global based on the fact the:
    /// - The `Sender` side does not rely on any async runtime.
    /// - Exit logic doesn't really require carefully threading this value through the app.
    /// - The `Receiver` side of a watch channel performs the detection based on if the change
    ///   happened after the subscribe, so it means all listeners should always be notified
    ///   correctly.
    pub static ref EXIT: watch::Sender<bool> = watch::channel(false).0;
}
