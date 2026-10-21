use rune_macros::Any;
use rune::alloc::clone::TryClone;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryClone)]
pub enum TriggerKind {
    Entity,
    Timer,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Any, TryClone)]
pub struct Trigger {
    pub kind: TriggerKind,
    pub id: String,
}

impl Trigger {
    // Переименовали методы в snake_case, чтобы макрос Any не сходил с ума
    pub fn new_entity(id: String) -> Self {
        Self { kind: TriggerKind::Entity, id }
    }

    pub fn new_timer(name: String) -> Self {
        Self { kind: TriggerKind::Timer, id: name }
    }

    pub fn new_custom(name: String) -> Self {
        Self { kind: TriggerKind::Custom, id: name }
    }
}

#[derive(Debug, Clone, Any, TryClone)]
pub struct AutomationEvent {
    pub trigger: Trigger,
    pub entity_id: Option<String>,
    pub value: f64,
}

impl AutomationEvent {
    pub fn trigger(&self) -> Trigger { self.trigger.clone() }
    pub fn entity_id(&self) -> Option<String> { self.entity_id.clone() }
    pub fn value(&self) -> f64 { self.value }
}

#[derive(Debug, Any)]
pub struct Rule {
    pub when: Vec<Trigger>,
    pub then: rune::runtime::Function,
}


/// Фабричная функция для создания правила автоматизации внутри Rune
pub fn create_rule(when: Vec<Trigger>, then: rune::runtime::Function) -> Rule {
    Rule { when, then }
}
