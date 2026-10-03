use std::collections::HashMap;
use std::{println, thread,};

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
        while let Some(msg) = self.rx.blocking_recv() {
            match msg {
                EntityMessage::Register(entity) => {
                    let id = entity.id.clone();
                  if  self.entities.contains_key(&id) {
                    println!("Сущность с  id = {} уже существует", id);
                    continue;
                    }
                   
                    self.entities.insert(entity.id.clone(), entity);
                    println!("Сущность с  id = {} добавленна в entities", id);
                   
                },
                EntityMessage::UpdateState {entity_id, new_state, source } => {
                    if let Some(entity) = self.entities.get_mut(&entity_id) {
                        if entity.state == new_state {
                            continue;
                        }
                        entity.state = new_state;
                        println!("🔄 '{}' обновлена (source: {:?})", entity_id, source);

                    } else {
                        eprintln!("⚠️ UpdateState: неизвестная '{}'", entity_id);
                    }
                },
                EntityMessage::GetState { entity_id, reply } => {
                        let entity = self.entities.get(&entity_id).cloned();
                        let _ = reply.send(entity);

                },
                EntityMessage::ListAll { reply } => {
                    let all: Vec<Entity> = self.entities.values().cloned().collect();
                    let _ =reply.send(all);
                },
                EntityMessage::Shutdown => {
                    println!("EntityManager: получен Shutdown");
                    break;
                },
            }
        }


    }
}



