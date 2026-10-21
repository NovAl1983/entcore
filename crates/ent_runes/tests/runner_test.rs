use std::sync::Arc;
use rune::Vm;

use ent_runes::rule::{Rule, Trigger, AutomationEvent, TriggerKind};
use ent_runes::compiler::setup_context;
use ent_runes::source::Script;

#[test]
fn test_script_automation_flow() {
    // 1. Имитируем скрипт автоматизации в памяти через вызов фабрики create_rule
    let test_script_code = r#"
    pub fn init() {
        return automation::create_rule(
            [automation::entity_trigger("light.living_room")],

            |event| {
                println!("🎉 [Rune Test] Автоматизация сработала! Значение: {}", event.value());
            }
        );
    }
    "#;


    let mock_script = Script {
        name: "test_automation".to_string(),
        path: std::path::PathBuf::from("test_automation.rn"),
        source_code: test_script_code.to_string(),
    };

    // 2. Готовим чистый контекст БЕЗ загрузки файлов с диска
    let context = setup_context().expect("Контекст должен собраться");
    let rune_runtime = Arc::new(context.runtime().expect("Рантайм должен создаться"));
    
    // 3. Компилируем СТРОГО ОДИН наш mock-скрипт из памяти
    let mut sources = rune::Sources::new();
    let source = rune::Source::new(&mock_script.name, &mock_script.source_code)
        .expect("Ошибка создания источника");
    sources.insert(source).expect("Ошибка вставки источника");

    let mut diagnostics = rune::Diagnostics::new();
    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        println!("⚠️ Диагностика компиляции внутри теста:\n{:?}", diagnostics);
    }

    let unit = Arc::new(result.expect("Скрипт в памяти должен успешно скомпилироваться"));

    // 4. ИСПРАВЛЕНО: Передаем корневую функцию как массив из одного элемента ["init"]
    let mut vm = Vm::new(rune_runtime, unit);
    let init_value = vm.call(["init"], ()) 
        .expect("Функция init должна успешно выполниться");

        
    let rule: Rule = rune::from_value(init_value)
        .expect("Возвращенное значение должно быть структурой Rule");

    // 5. Проверяем триггер, созданный внутри Rune
    assert_eq!(rule.when.len(), 1);
    assert_eq!(rule.when[0].kind, TriggerKind::Entity);
    assert_eq!(rule.when[0].id, "light.living_room");
    
    let incoming_entity_id = "light.living_room";
    
    // 6. Имитируем логику матчинга ScriptRunner для нашей структуры
    for trigger in &rule.when {
        if trigger.kind == TriggerKind::Entity && trigger.id == incoming_entity_id {
            let event = AutomationEvent {
                trigger: trigger.clone(),
                entity_id: Some(incoming_entity_id.to_string()),
                value: 1.0,
            };

            // Вызываем Rune-замыкание напрямую через одиночный кортеж и в конце пишем .into_result()
            let result = rule.then.call::<()>((event,)).into_result();
            
            assert!(result.is_ok(), "Замыкание автоматизации должно выполниться без ошибок. Ошибка: {:?}", result.err());
            println!("🚀 Интеграционный тест Rune успешно пройден!");
        }
    }

}
