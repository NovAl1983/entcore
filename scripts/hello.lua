
registr_entity_id({
    "light.living_room", 
    "light.kitchen", 
    "sensor.kitchen_motion",
}) -- rust функция которая возвращает в ядро коллекцию entity_id

-- для теста: вызвать при загрузке
-- get_state("light.kitchen")
local light = get_state("light.kitchen")  -- true / false / 42.0 / nil
if light ~= nil then
    log("свет: " .. tostring(light.is_on))
    log("свет: " .. tostring(light.brightness))
end



return function(event)
            log("событие: " .. event.entity_id)
            log("значение: " .. tostring(event.value))
        end