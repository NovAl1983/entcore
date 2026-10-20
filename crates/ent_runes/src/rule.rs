use rune_macros::Any;
use rune::alloc::clone::TryClone;

/// Перечисление для внутренней логики Rust.
/// Добавляем TryClone и Copy, чтобы структура Trigger могла беспрепятственно клонироваться.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryClone)]
pub enum TriggerKind {
    Entity,
    Timer,
    Custom,
}

/// Чистая структура, которая идеально пробрасывается в Rune
#[derive(Debug, Clone, PartialEq, Eq, Hash, Any, TryClone)]
pub struct Trigger {
    pub kind: TriggerKind,
    pub id: String,
}

impl Trigger {
    pub fn entity(id: String) -> Self {
        Self { kind: TriggerKind::Entity, id }
    }

    pub fn timer(name: String) -> Self {
        Self { kind: TriggerKind::Timer, id: name }
    }

    pub fn custom(name: String) -> Self {
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
