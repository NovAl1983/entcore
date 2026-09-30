use std::collections::HashMap;
use std::thread;

use tokio::sync::mpsc;

use crate::entity::Entity;
use crate::handle::EntityHandle;
use crate::message::EntityMessage;

pub struct EntityManager {
    rx: mpsc::UnboundedReceiver<EntityMessage>,
    entities: HashMap<String, Entity>,
}

impl EntityManager {
    pub fn spawn() -> EntityHandle {
        let (tx, rx) = mpsc::unbounded_channel();

        let manager = Self {
            rx,
            entities: HashMap::new(),
        };

        thread::Builder::new()
            .name("entity-manager".into())
            .spawn(move || manager.run())
            .expect("Не удалось запустить поток EntityManager");

        EntityHandle::new(tx)
    }

    fn run(mut self) {
        while let Some(_msg) = self.rx.blocking_recv() {
            // пока пусто
        }
    }
}