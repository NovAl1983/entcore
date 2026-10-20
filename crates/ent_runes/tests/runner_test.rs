use std::sync::Arc;
use rune::Vm;

use ent_runes::rule::{Rule, Trigger, AutomationEvent, TriggerKind};
use ent_runes::compiler::{setup_context, compile_scripts};
use ent_runes::source::Script;

#[test]
fn test_script_automation_flow() {
    // 1. Имитируем скрипт автоматизации
    let test_script_code = r#"
    pub fn init() {
        return automation::Rule {
            when: [
                automation::Trigger::Entity("light.living_room"),
            ],
            then: |event| {
                println!("🎉 [Rune Test] Автоматизация сработала! Значение: {}", event.value());
            }
        };
    }
    "#;

    let mock_script = Script {
        name: "test_automation".to_string(),
        path: std::path::PathBuf::from("test_automation.rn"),
        source_code: test_script_code.to_string(),
    };

    // 2. Готовим контекст и компилируем скрипт
    let context = setup_context().expect("Контекст должен собраться");
    let rune_runtime = Arc::new(context.runtime().expect("Рантайм должен создаться"));
    
    let unit = compile_scripts(&context, &[mock_script])
        .expect("Скрипт должен успешно скомпилироваться");

    // 3. Вызываем функцию init(), чтобы вытащить правило в Rust
    let mut vm = Vm::new(rune_runtime, unit);
    let init_value = vm.call(["test_automation", "init"], ())
        .expect("Функция init должна успешно выполниться");
        
    let rule: Rule = rune::from_value(init_value)
        .expect("Возвращенное значение должно быть структурой Rule");

    // Проверяем первый триггер из вектора rule.when
    assert_eq!(rule.when.len(), 1);
    assert_eq!(rule.when[0].kind, TriggerKind::Entity);
    assert_eq!(rule.when[0].id, "light.living_room");
    
    let incoming_entity_id = "light.living_room";
    
    // 4. Имитируем логику матчинга ScriptRunner для нашей структуры Trigger
    for trigger in &rule.when {
        if trigger.kind == TriggerKind::Entity && trigger.id == incoming_entity_id {
            // Создаем контекст события
            let event = AutomationEvent {
                trigger: trigger.clone(),
                entity_id: Some(incoming_entity_id.to_string()),
                value: 1.0,
            };

            // Вызываем Rune-замыкание напрямую
            let result = rule.then.call::<()>((event,)).into_result();
            
            assert!(result.is_ok(), "Замыкание автоматизации должно выполниться без ошибок");
            println!("🚀 Интеграционный тест Rune успешно пройден!");
        }
    }
}
