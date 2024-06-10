local adjust_damage = dofile_once("mods/quant.ew/files/src/system/damage/cbs/adjust_damage.lua").adjust_damage

-- Called on clients when they get damage and redirects it to the host's hp over the network, ignoring any resists.
function damage_received(damage, message, entity_thats_responsible, is_fatal, projectile_thats_responsible)
    local dtypes = GetDamageDetails().damage_types
    local new_damage = adjust_damage(damage, dtypes)
    
    -- Change our health back-ish
    local entity_id = GetUpdatedEntityID();
    local damageModelComponent = EntityGetFirstComponentIncludingDisabled( entity_id, "DamageModelComponent" )
    if damageModelComponent ~= nil then
        local health = ComponentGetValue2( damageModelComponent, "hp" )
        if health then
            ComponentSetValue2( damageModelComponent, "hp", health + (damage - new_damage) )
        end
    end

end
