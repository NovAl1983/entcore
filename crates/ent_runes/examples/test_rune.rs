use rune::{Context, Diagnostics, Source, Sources, Vm};
use std::println;
use  std::sync::Arc;

fn main()-> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Запуск тестового скрипта на Rune v0.14.1...");

    // 1. Создаем пустой базовый контекст
    let  context = Context::with_default_modules()?;
    println!("Создаем пустой базовый контекст");

    
    let runtime = Arc::new(context.runtime()?);
     println!("Создаем рантайм-компоненты на основе контекста");

    // 2. Пишем наш скрипт. В Rune 0.14.1 для вывода используется макрос println!
    // Точно так же, как в чистом Rust!
        let script = r#"
    pub fn hello_world(name) {
        println!("Привет, {}! Скрипт успешно работает на Rune 0.14.1!", name);
        return 42;
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

    // 6. Вызываем нашу функцию по её имени. 
    // Аргумент передаем как кортеж. Rune автоматически преобразует строку Rust в тип Rune.
    println!("🏃 Запускаем функцию hello_world...");
    let value = vm.call(["hello_world"], ("Alexandro",))?;

    // 7. Извлекаем возвращенное значение из типа-обертки Rune Value
    let return_value: i64 = rune::from_value(value)?;
    println!("🎉 Скрипт вернул значение: {}", return_value);



    Ok(())
}