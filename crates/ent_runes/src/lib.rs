pub mod source;
pub mod rule;
pub mod compiler;
pub mod runner;

pub use rule::{Trigger, Rule, AutomationEvent, TriggerKind};

use rune::{Module, ContextError};

/// Создает изолированный модуль автоматизации "automation" для движка Rune.
pub fn create_automation_module() -> Result<Module, ContextError> {
    let mut module = Module::with_crate("automation")?;
    
    // 1. Регистрируем типы данных
    module.ty::<Trigger>()?;
    module.ty::<Rule>()?;
    
    // Свободные фабричные функции для триггеров и самого правила автоматизации
    module.function("entity_trigger", rule::Trigger::new_entity).build()?;
    module.function("timer_trigger", rule::Trigger::new_timer).build()?;
    module.function("custom_trigger", rule::Trigger::new_custom).build()?;
    
    // Добавляем конструктор правила автоматизации!
    module.function("create_rule", rule::create_rule).build()?;
    
    // 2. Регистрируем тип AutomationEvent и его методы-геттеры
    module.ty::<AutomationEvent>()?;
    module.function("trigger", rule::AutomationEvent::trigger).build()?;
    module.function("entity_id", rule::AutomationEvent::entity_id).build()?;
    module.function("value", rule::AutomationEvent::value).build()?;

    Ok(module)
}
