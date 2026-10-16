use rune::{Context, Diagnostics, Source, Sources, Vm, Module};
use std::println;
use  std::sync::Arc;

use ent_core::state::{LightState, SensorState, BinarySensorState, CounterState, InputBooleanState};

fn main()-> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Запуск тестового скрипта на Rune v0.14.1...");

    // 1. Создаем пустой базовый контекст
    let mut context = Context::with_default_modules()?;
    println!("Создаем базовый контекст");

    let mut my_module = Module::new();

    my_module.ty::<LightState>()?;
    my_module.ty::<SensorState>()?;
    my_module.ty::<BinarySensorState>()?;
    my_module.ty::<CounterState>()?;
    my_module.ty::<InputBooleanState>()?;
    
    
    // Устанавливаем модуль в контекст
    context.install(my_module)?;

    
    let runtime = Arc::new(context.runtime()?);
     println!("Создаем рантайм-компоненты на основе контекста");

    // 2. Пишем наш скрипт. В Rune 0.14.1 для вывода используется макрос println!
    // Точно так же, как в чистом Rust!
        let script = r#"
pub fn check_automation(light) {
        println!("Rune: Проверяю состояние света...");
        
        if light.is_on {
            println!("Rune: Свет горит! Яркость устройства: {:?}", light.brightness);
        } else {
            println!("Rune: Свет выключен, автоматизация не требуется.");
        }
        
        return "Проверка завершена";
    }
    "#;

    // 3. Загружаем строку скрипта в менеджер источников
    let  mut sources = Sources::new();
    sources.insert(Source::new("my_test_script", script)?)?;

    // 4. Компилируем текстовый скрипт в байт-код (Unit)
    let mut diagnostics= Diagnostics::new();
    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        println!("⚠️ Диагностические сообщения/ошибки компиляции:");
        println!("{:?}", diagnostics);
    }

    // let unit = result?;
    let unit = Arc::new(result?); // решил сразу обернуть в Arc
    println!("✅ Скрипт успешно скомпилирован в байт-код.");

    // 5. Инициализируем виртуальную машину
    let mut vm = Vm::new(runtime, unit.clone()); // а тут копирую ссылку (увеличиваю счетчик ссылок)

        // 6. Создаем НАСТОЯЩИЙ объект Rust с данными, имитируя датчик или команду
    let sample_light = LightState {
        is_on: true,
        brightness: Some(185),
        color_temp: None,
    };

    // 6. Вызываем нашу функцию по её имени. 
    // Аргумент передаем как кортеж. Rune автоматически преобразует строку Rust в тип Rune.
    println!("🏃 Запускаем функцию check_automation и передаем туда структуру из Rust...");
    let value = vm.call(["check_automation"], (sample_light,))?;

    // 7. Извлекаем возвращенное значение из типа-обертки Rune Value
    let return_value: String = rune::from_value(value)?;
    println!("🎉 Скрипт вернул значение: {}", return_value);



    Ok(())
}