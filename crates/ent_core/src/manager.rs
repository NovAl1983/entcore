use std::collections::HashMap;
use std::{println, thread,};

use tokio::sync::mpsc;

use crate::entity::Entity;
use crate::handle::EntityHandle;
use crate::message::EntityMessage;
use  crate::state::{EntityState, FieldUpdate};

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
                EntityMessage::UpdateStateField { entity_id, field, source } => {
    if let Some(entity) = self.entities.get_mut(&entity_id) {
        match (&mut entity.state, field) {
            (EntityState::Light { is_on, .. }, FieldUpdate::IsOn(v)) => {
                if *is_on == v { continue; }
                *is_on = v;
            }
            (EntityState::Light { brightness, .. }, FieldUpdate::Brightness(v)) => {
                if *brightness == v { continue; }
                *brightness = v;
            }
            (EntityState::Light { color_temp, .. }, FieldUpdate::ColorTemp(v)) => {
                if *color_temp == v { continue; }
                *color_temp = v;
            }
            (EntityState::Sensor { value }, FieldUpdate::Value(v)) => {
                if *value == v { continue; }
                *value = v;
            }
            (EntityState::BinarySensor { is_on }, FieldUpdate::IsOn(v)) => {
                if *is_on == v { continue; }
                *is_on = v;
            }
            (EntityState::InputBoolean { is_on }, FieldUpdate::IsOn(v)) => {
                if *is_on == v { continue; }
                *is_on = v;
            }
            _ => {
                eprintln!("⚠️ Несовместимое поле для '{}'", entity_id);
                continue;
            }
        }
        println!("🔄 '{}' обновлена (field, source: {:?})", entity_id, source);
    } else {
        eprintln!("⚠️ UpdateStateField: неизвестная '{}'", entity_id);
    }
}
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



