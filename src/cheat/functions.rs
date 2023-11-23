use std::ops::BitAnd;
use mint::{Vector3, Vector2};
use crate::cheat::classes::{entity::Entity, offsets::Offsets, view::View};
use crate::utils::{config::Config, process_manager::{read_memory_auto, trace_address}};

pub fn is_enemy_at_crosshair(window_info: ((i32, i32), (i32, i32)), local_entity_pawn_address: u64, local_entity_pawn_team_id: i32, game_address_entity_list: u64, game_view: View, config: Config) -> (bool, bool) {
    let mut u_handle: u32 = 0;
    
    if !read_memory_auto(local_entity_pawn_address + Offsets::C_CSPlayerPawnBase::m_iIDEntIndex as u64, &mut u_handle) {
        return (false, false);
    }

    if !read_memory_auto(local_entity_pawn_address + Offsets::C_CSPlayerPawnBase::m_iIDEntIndex as u64, &mut u_handle) {
        return (false, false);
    }

    let list_entry: u64 = trace_address(game_address_entity_list, &[0x8 * (u_handle >> 9) + 0x10, 0x0]);

    if list_entry == 0 {
        return (false, false);
    }

    let mut pawn_address: u64 = 0;

    if !read_memory_auto(list_entry + 0x78 * u_handle.bitand(0x1FF) as u64, &mut pawn_address) {
        return (false, false);
    }

    let mut entity = Entity::default();

    if !entity.update_pawn(pawn_address, window_info, game_view) {
        return (false, false);
    }

    let allow_shoot = {
        if config.misc.enabled && config.misc.exclude_team {
            local_entity_pawn_team_id != entity.pawn.team_id && entity.pawn.health > 0
        } else {
            entity.pawn.health > 0
        }
    };

    return (true, allow_shoot);
}

pub fn is_enemy_visible(b_spotted_by_mask: u64, local_b_spotted_by_mask: u64, local_player_controller_index: u64, i: u64) -> bool {
    return b_spotted_by_mask.bitand(1 << local_player_controller_index) != 0 || local_b_spotted_by_mask.bitand(1 << i) != 0;
}

pub fn is_enemy_in_fov(config: Config, aim_pos: Vector3<f32>, camera_pos: Vector3<f32>, view_angle: Vector2<f32>) -> Option<f32> {
    let pos = Vector3 { x: aim_pos.x - camera_pos.x, y: aim_pos.y - camera_pos.y, z: aim_pos.z - camera_pos.z };
    let distance = (pos.x.powf(2.0) + pos.y.powf(2.0)).sqrt();
    let yaw = pos.y.atan2(pos.x) * 57.295779513 - view_angle.y;
    let pitch = -(pos.z / distance).atan() * 57.295779513 - view_angle.x;
    let norm = (yaw.powf(2.0) + pitch.powf(2.0)).sqrt() * 0.75;
    
    if norm > config.aimbot.fov {
        return None;
    }

    return Some(norm);
}

pub fn get_bomb(bomb_address: u64) -> Option<u64> {
    let mut planted_bomb: u64 = 0;

    if !read_memory_auto(bomb_address, &mut planted_bomb) {
        return None;
    }

    if !read_memory_auto(planted_bomb, &mut planted_bomb) {
        return None;
    }

    return Some(planted_bomb);
}

pub fn get_bomb_planted(bomb_address: u64) -> bool {
    let mut is_bomb_planted: bool = false;
    
    if !read_memory_auto(bomb_address - 0x8, &mut is_bomb_planted) {
        return false;
    };

    return is_bomb_planted;
}

pub fn get_bomb_site(planted_bomb: u64) -> Option<String> {
    let mut site: u32 = 0;

    if !read_memory_auto(planted_bomb + Offsets::C_PlantedC4::m_nBombSite as u64, &mut site) {
        return None;
    }

    if site == 1 {
        return Some("B".to_string());
    } else {
        return Some("A".to_string());
    }
}

pub fn get_bomb_position(planted_bomb: u64) -> Option<Vector3<f32>> {
    let mut bomb_node = 0;

    if !read_memory_auto(planted_bomb + Offsets::C_BaseEntity::m_pGameSceneNode as u64, &mut bomb_node) {
        return None;
    }

    let mut bomb_pos = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

    if !read_memory_auto(bomb_node + Offsets::CGameSceneNode::m_vecAbsOrigin as u64, &mut bomb_pos) {
        return None;
    }

    return Some(bomb_pos);
}

pub fn parse_weapon_name(name: String) -> String {
    return match name.as_str() {
        "ak47" => "AK-47",
        "aug" => "AUG",
        "awp" => "AWP",
        "bizon" => "PP-Bizon",
        "c4" => "Bomb",
        "cz75a" => "CZ-75 Auto",
        "deagle" => "Desert Eagle",
        "decoy" => "Decoy Grenade",
        "elite" => "Dual Berettas",
        "fists" => "Fists",
        "famas" => "FAMAS",
        "fiveseven" => "Five-SeveN",
        "flashbang" => "Flashbang",
        "g3sg1" => "G3SG1",
        "galilar" => "Galil AR",
        "glock" => "Glock",
        "healthshot" => "MediShot",
        "hkp2000" => "P2000",
        "hegrenade" => "Grenade",
        "incgrenade" => "Incendiary",
        "knife" => "Knife",
        "m249" => "M249",
        "m4a1" => "M4A4",
        "m4a1_silencer" => "M4A1-S",
        "mac10" => "MAC-10",
        "mag7" => "MAG-7",
        "molotov" => "Molotov",
        "mp5sd" => "MP5-SD",
        "mp7" => "MP7",
        "mp9" => "MP9",
        "negev" => "Negev",
        "nova" => "Nova",
        "p250" => "P250",
        "p90" => "P90",
        "revolver" => "Revolver",
        "sawedoff" => "Sawed-Off",
        "scar20" => "SCAR-20",
        "sg556" => "SG556",
        "smokegrenade" => "Smoke",
        "ssg08" => "SSG 08",
        "taser" => "Zeus x27",
        "tec9" => "TEC-9",
        "ump45" => "UMP-45",
        "usp_silencer" => "USP-S",
        "xm1014" => "XM1014",
        _ => &name
    }.to_string();
}