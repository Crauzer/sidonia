use winapi::shared::minwindef::LPVOID;
use crate::core::riot::r3d::vector3::R3dVector3;
use crate::core::riot::net::{RiotNetVisibilityObject, RiotNetId};
use crate::core::riot::replication::RiotReplicate;
use crate::core::msvc::vector::StdVector;
use crate::core::msvc::smart_pointers::StdSharedPtr;
use crate::core::riot::issue_order::RiotIIssueOrders;

#[repr(C)]
pub struct RiotSpellbook {
    vtable: LPVOID,
    autocast_spell_slot: i32,
    is_summoner_spellbook: bool,
    spellbook_owner: *mut RiotISpellbookOwner,
    owner_position: R3dVector3,
    net_visibility: *mut RiotNetVisibilityObject,
    owner_spell_training_points: i32,
    active_spell_slot: i32,
    character_intermediate: *mut RiotCharacterIntermediate,
    owner_id: RiotNetId,
    selected_spell_slot: i32,
    spellcasting_object: StdSharedPtr<RiotSpellInstance>,
    current_mana_cost: [RiotReplicate<f32>; 63],
    owner_orders: *mut RiotIIssueOrders,
    is_casting_spell: bool,
    autocast_crit_slot: i32,
    auto_attack_override: bool,
    spell_can_cast_bits: CanCastBits,
    cast_bit_1: RiotReplicate<u32>,
    changed_tool_tip: bool,
    spell_max_levels_override: [u32; 4],
    spells_up_levels_override: [[u32; 6]; 4]
}

#[repr(C)]
pub struct RiotSpellbookOwnerOverride {
    spellbook_owner: RiotISpellbookOwner,
    owner: *mut RiotISpellbookOwner
}

#[repr(C)]
pub struct RiotISpellbookOwner {
    vtable: LPVOID
}

#[repr(C)]
pub struct RiotCharacterIntermediate {
    flat_cooldown_mod: f32,
    percent_cooldown_mod: RiotReplicate<f32>,
    passive_cooldown_end_time: f32,
    passive_cooldown_total_time: f32,
    flat_hppool_mod: f32,
    percent_hppool_mod: f32,
    flat_parpool_mod: f32,
    percent_parpool_mod: f32,
    flat_hpregen_mod: f32,
    percent_hpregen_mod: f32,
    percent_base_hpregen_mod: f32,
    flat_parregen_mod: f32,
    percent_parregen_mod: f32,
    percent_tenacity_rune_mod: f32,
    percent_tenacity_cleanse_mod: f32,
    percent_tenacity_mastery_mod: f32,
    percent_tenacity_character_mod: f32,
    percent_tenacity_item_mod: f32,
    percent_ccreduction: f32,
    percent_slow_resist_mod: f32,
    flat_movement_speed_mod: f32,
    percent_movement_speed_mod: f32,
    percent_multiplicative_movement_speed_mod: f32,
    move_speed_floor_mod: f32,
    percent_multiplicative_movement_speed_mods: StdVector<f32>,
    flat_armor_mod: f32,
    percent_armor_mod: f32,
    flat_armor_penetration_mod: f32,
    flat_magic_penetration_mod: f32,
    percent_armor_penetration_mod: f32,
    percent_magic_penetration_mod: f32,
    percent_bonus_armor_penetration_mod: f32,
    percent_bonus_magic_penetration_mod: f32,
    flat_spell_block_mod: f32,
    percent_spell_block_mod: f32,
    flat_miss_chance_mod: f32,
    flat_dodge_mod: f32,
    flat_crit_chance_mod: f32,
    flat_crit_damage_mod: f32,
    percent_crit_damage_mod: f32,
    flat_physical_damage_mod: f32,
    percent_physical_damage_mod: f32,
    flat_magic_damage_mod: f32,
    percent_magic_damage_mod: f32,
    flat_physical_reduction: f32,
    percent_physical_reduction: f32,
    flat_magic_reduction: f32,
    percent_magic_reduction: f32,
    percent_expbonus: f32,
    flat_attack_range_mod: f32,
    percent_attack_range_mod: f32,
    flat_cast_range_mod: RiotReplicate<f32>,
    percent_cast_range_mod: f32,
    acquisition_range_mod: f32,
    pathfinding_radius_mod: f32,
    percent_attack_speed_mod: f32,
    percent_multiplicative_attack_speed_mod: f32,
    percent_healing_amount_mod: f32,
    percent_life_steal_mod: f32,
    percent_spell_vamp_mod: f32,
    percent_respawn_time_mod: f32,
    percent_gold_lost_on_death_mod: f32,
    attack_speed_mod: f32,
    base_attack_damage: f32,
    base_ability_damage: f32,
    crit_damage_multiplier: f32,
    scale_skin_coef: f32,
    miss_chance: f32,
    dodge: f32,
    crit: f32,
    armor: f32,
    spell_block: f32,
    hpregen_rate: f32,
    base_parregen_rate: f32,
    percent_base_parregen_mod: f32,
    parregen_rate: f32,
    move_speed: f32,
    attack_range: f32,
    cast_range: f32,
    flat_bubble_radius_mod: f32,
    percent_bubble_radius_mod: f32,
    _flat_hpmod_per_level: f32,
    _flat_mpmod_per_level: f32,
    _flat_armor_mod_per_level: f32,
    _flat_spell_block_mod_per_level: f32,
    _flat_hpregen_mod_per_level: f32,
    _flat_mpregen_mod_per_level: f32,
    _flat_physical_damage_mod_per_level: f32,
    _flat_magic_damage_mod_per_level: f32,
    _flat_movement_speed_mod_per_level: f32,
    _percent_movement_speed_mod_per_level: f32,
    _percent_attack_speed_mod_per_level: f32,
    _flat_crit_chance_mod_per_level: f32,
    _flat_crit_damage_mod_per_level: f32,
    _flat_dodge_mod: f32,
    _flat_dodge_mod_per_level: f32,
    _flat_armor_penetration_mod: f32,
    _flat_armor_penetration_mod_per_level: f32,
    _percent_armor_penetration_mod: f32,
    _percent_armor_penetration_mod_per_level: f32,
    _percent_cooldown_mod: f32,
    _percent_cooldown_mod_per_level: f32,
    _flat_time_dead_mod: f32,
    _flat_time_dead_mod_per_level: f32,
    _percent_time_dead_mod: f32,
    _percent_time_dead_mod_per_level: f32,
    flat_gold_per10mod: f32,
    _flat_magic_penetration_mod: f32,
    _flat_magic_penetration_mod_per_level: f32,
    _percent_magic_penetration_mod: f32,
    _percent_magic_penetration_mod_per_level: f32,
    _non_healing_flat_hppool_mod: f32,
    flat_exp_reward_mod: f32,
    flat_gold_reward_mod: f32,
    percent_local_gold_reward_mod: f32,
}

#[repr(C)]
pub struct RiotSpellInstance {
    vtable: LPVOID
}

#[repr(C)]
union CanCastBits {
    bits32: [u32; 2],
    bits64: u64
}