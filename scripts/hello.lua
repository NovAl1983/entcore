
registr_entity_id({
    "light.living_room", 
    "light.kitchen", 
    "sensor.kitchen_motion",
}) -- rust функция которая возвращает в ядро коллекцию entity_id

-- для теста: вызвать при загрузке
get_entity("light.kitchen")




return function(event)
            log("событие: " .. event.entity_id)
            log("значение: " .. tostring(event.value))
        end