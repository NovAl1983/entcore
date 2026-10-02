use ent_core::{Entity, EntityHandle, EntityManager, EntityState, UpdateSource};

#[test]
fn test_register_and_get() {
    // 1. Создаём EntityManager
    let handle = EntityManager::spawn();
    
    // 2. Создаём сущность
    let light = Entity {
        id: "light.living_room".to_string(),
        friendly_name: "Люстра в зале".to_string(),
        state: EntityState::Light {
            is_on: false,
            brightness: None,
            color_temp: None,
        },
    };
    
    // 3. Регистрируем
    handle.register(light.clone());
    
    // 4. Небольшая пауза, чтобы сообщение дошло
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    // 5. Читаем через handle
    let result = handle.get_state_blocking("light.living_room");
    
    // 6. Проверяем
    assert!(result.is_some(), "Сущность должна быть найдена");
    let entity = result.unwrap();
    assert_eq!(entity.id, "light.living_room");
    assert_eq!(entity.friendly_name, "Люстра в зале");
}