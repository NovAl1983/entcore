registr_entity_id({"sensor.kitchen_motion"})

return function(event)
    log("kitchen.lua движение на кухне: " .. tostring(event.value))
end