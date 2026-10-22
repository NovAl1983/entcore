return function(event)
            log("событие: " .. event.entity_id)
            log("значение: " .. tostring(event.value))
        end