use std::collections::HashMap;
use std::{println, thread};

use tokio::sync::mpsc;

use crate::entity::{self, Entity};
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
                EntityMessage::UpdateState { 
                    entity_id, 
                    new_state, 
                    source 
                } => {
                    // let old_state = self.entities.get(&entity_id);
                    
                    if let Some(old) = self.entities.get(&entity_id) {
                        if old.state == new_state {
                            continue;
                        }

                        let _ = self.entities.insert(entity_id.clone(), Entity { id: entity_id.clone(), friendly_name: entity_id.clone(), state: new_state });
                    


                    }
                    // self.entities.insert(entity_id, Entity { id: entity_id.clone(), friendly_name: entity_id.clone(), state: new_state });
                },
                _ => todo!()


            }
        }
    }
}

    // Register(Entity),

    // /// Обновить состояние сущности.
    // UpdateState {
    //     entity_id: String,
    //     new_state: EntityState,
    //     source: UpdateSource,
    // },

    // /// Прочитать сущность по id. Ответ — через oneshot.
    // GetState {
    //     entity_id: String,
    //     reply: oneshot::Sender<Option<Entity>>,
    // },

    // /// Получить список всех сущностей. Для отладки и WebUI.
    // ListAll {
    //     reply: oneshot::Sender<Vec<Entity>>,
    // },

    // /// Завершить работу актора.
    // Shutdown,