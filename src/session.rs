use tower_sessions::{SessionManagerLayer, MemoryStore};

pub fn session_layer() -> SessionManagerLayer<MemoryStore> {
    let store = MemoryStore::new();
    SessionManagerLayer::new(store)
}

