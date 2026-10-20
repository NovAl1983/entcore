use rune::{Context, ContextError, Module, Unit};
use std::sync::Arc;
use ent_core::state::{LightState, SensorState, BinarySensorState, CounterState, InputBooleanState};
use crate::{create_automation_module, source::Script};

fn create_core_types_module() -> Result<Module, ContextError> {
    let mut module = Module::with_crate("core")?;
    module.ty::<LightState>()?;
    module.ty::<SensorState>()?;
    module.ty::<BinarySensorState>()?;
    module.ty::<CounterState>()?;
    module.ty::<InputBooleanState>()?;
    Ok(module)
}

/// Подготовка контекста Rune со всеми модулями
pub fn setup_context() -> Result<Context, ContextError> {
    let mut context = rune::Context::with_default_modules()?;
    
    let automation_module = create_automation_module()?;
    context.install(automation_module)?;
    
    let core_module = create_core_types_module()?;
    context.install(core_module)?;
    
    Ok(context)
}

pub fn compile_scripts(context: &Context, scripts: &[Script]) -> Result<Arc<Unit>, String> {
    let mut sources = rune::Sources::new();

    for script in scripts {
        let source = rune::Source::new(&script.name, &script.source_code)
            .map_err(|e| format!("Ошибка создания источника {}: {}", script.name, e))?;
        sources.insert(source)
            .map_err(|e| format!("Ошибка вставки источника {}: {:?}", script.name, e))?;
    }

    let mut diagnostics = rune::Diagnostics::new();
    let result = rune::prepare(&mut sources)
        .with_context(context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        println!("⚠️ Диагностика компиляции Rune:\n{:?}", diagnostics);
    }

    match result {
        Ok(unit) => Ok(Arc::new(unit)),
        Err(e) => Err(format!("Ошибка компиляции автоматизаций: {}", e)),
    }
}
