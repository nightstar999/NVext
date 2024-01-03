// Copyright (c) 2023 Snipcola - Nightstar <nightstar6@protonmail.com>
// SPDX-License-Identifier: MIT

use std::ops::{BitAnd, Shr};
use mint::{Vector2, Vector3};

use crate::config::Offsets;

use crate::utils::cheat::process::{rpm_offset, rpm, rpm_auto, trace_address};
use crate::cheat::functions::{parse_weapon, WeaponType};

use crate::cheat::classes::bone::Bone;
use crate::cheat::classes::game::GAME;
use crate::cheat::classes::view::View;

#[derive(Clone, Copy)]
pub struct CUtlVector {
    pub count: u64,
    pub data: u64
}

impl Default for CUtlVector {
    fn default() -> Self {
        return Self {
            count: 0,
            data: 0
        };
    }
}

#[derive(Clone)]
pub struct PlayerController {
    pub address: u64,
    pub alive_status: i32,
    pub pawn: u64,
    pub team_id: i32,
    pub player_name: String
}

impl Default for PlayerController {
    fn default() -> Self {
        return Self {
            address: 0,
            alive_status: 0,
            pawn: 0,
            team_id: 0,
            player_name: String::new()
        };
    }
}

#[derive(Clone)]
pub struct PlayerPawn {
    pub address: u64,
    pub bone_data: Bone,
    pub view_angle: Vector2<f32>,
    pub pos: Vector3<f32>,
    pub screen_pos: Vector2<f32>,
    pub camera_pos: Vector3<f32>,
    pub weapon_name: String,
    pub weapon_type: WeaponType,
    pub weapon_max_ammo: i32,
    pub weapon_ammo: i32,
    pub shots_fired: u32,
    pub aim_punch_cache: CUtlVector,
    pub health: i32,
    pub armor: i32,
    pub fov: i32,
    pub spotted_by_mask: u64,
    pub flags: i32
}

impl Default for PlayerPawn {
    fn default() -> Self {
        return Self {
            address: 0,
            bone_data: Bone::default(),
            view_angle: Vector2 { x: 0.0, y: 0.0 },
            pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            screen_pos: Vector2 { x: 0.0, y: 0.0 },
            camera_pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            weapon_name: "".to_string(),
            weapon_type: WeaponType::None,
            weapon_max_ammo: 0,
            weapon_ammo: 0,
            shots_fired: 0,
            aim_punch_cache: CUtlVector::default(),
            health: 0,
            armor: 0,
            fov: 0,
            spotted_by_mask: 0,
            flags: 0
        };
    }
}

pub enum Flags {
    InAir = (1 as isize).wrapping_shl(0)
}

#[derive(Clone)]
pub struct Entity {
    pub controller: PlayerController,
    pub pawn: PlayerPawn
}

impl Default for Entity {
    fn default() -> Self {
        return Entity {
            controller: PlayerController::default(),
            pawn: PlayerPawn::default()
        }
    }
}

pub fn buffer_to_string(buffer: &[u8]) -> String {
    let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
    let cleaned_buffer = &buffer[..len];
    String::from_utf8_lossy(cleaned_buffer).to_string()
}

impl Entity {
    pub fn update_controller(&mut self, player_controller_address: u64) -> bool {
        if player_controller_address == 0 {
            return false;
        }

        self.controller.address = player_controller_address;

        if !self.controller.get_is_alive() {
            return false;
        }

        if !self.controller.get_team_id() {
            return false;
        }

        if !self.controller.get_player_name() {
            return false;
        }

        self.pawn.address = self.controller.get_player_pawn_address();
        return true;
    }

    pub fn update_pawn(&mut self, player_pawn_address: u64, window_info: ((i32, i32), (i32, i32)), view: View) -> bool {
        if player_pawn_address == 0 {
            return false;
        }

        self.pawn.address = player_pawn_address;

        if !self.pawn.get_camera_pos() {
            return false;
        }

        if !self.pawn.get_pos() {
            return false;
        }

        if !self.pawn.get_view_angle() {
            return false;
        }

        if !self.pawn.get_weapon() {
            return false;
        }

        if !self.pawn.get_shots_fired() {
            return false;
        }

        if !self.pawn.get_aim_punch_cache() {
            return false;
        }

        if !self.pawn.get_health() {
            return false;
        }

        if !self.pawn.get_armor() {
            return false;
        }

        if !self.pawn.get_fov() {
            return false;
        }

        if !self.pawn.get_spotted() {
            return false;
        }

        if !self.pawn.get_f_flags() {
            return false;
        }

        if !self.pawn.bone_data.update_bone_data(player_pawn_address, window_info, view) {
            return false;
        }

        return true;
    }

    pub fn is_alive(&mut self) -> bool {
        return self.controller.alive_status == 1 && self.pawn.health > 0;
    }

    pub fn is_in_screen(&mut self, window_info: ((i32, i32), (i32, i32)), view: View) -> bool {
        return view.world_to_screen(self.pawn.pos, &mut self.pawn.screen_pos, window_info);
    }

    pub fn get_bone(&mut self) -> Option<Bone> {
        if self.pawn.address == 0 {
            return None;
        }

        return Some(self.pawn.bone_data.clone());
    }
}

impl PlayerController {
    pub fn get_is_alive(&mut self) -> bool {
        return rpm_offset(self.address, Offsets::CCSPlayerController::m_bPawnIsAlive as u64, &mut self.alive_status);
    }
    
    pub fn get_team_id(&mut self) -> bool {
        return rpm_offset(self.address, Offsets::C_BaseEntity::m_iTeamNum as u64, &mut self.team_id);
    }

    pub fn get_player_name(&mut self) -> bool {
        let mut buffer: [u8; 260] = [0; 260];
        
        if !rpm(self.address + Offsets::CBasePlayerController::m_iszPlayerName as u64, &mut buffer, 260) {
            return false;
        }

        let player_name = buffer_to_string(&buffer);

        if !player_name.is_empty() {
            self.player_name = player_name;
        }

        return true;
    }

    pub fn get_player_pawn_address(&mut self) -> u64 {
        let mut entity_pawn_list_entry = 0;
        let mut entity_pawn_address = 0;

        if !rpm_offset(self.address, Offsets::CCSPlayerController::m_hPlayerPawn as u64, &mut self.pawn) {
            return 0;
        }

        if !rpm_auto((*GAME.lock().unwrap()).address.entity_list, &mut entity_pawn_list_entry) {
            return 0;
        }

        if let Some(sum) = (8 as u64).checked_mul(self.pawn.bitand(0x7FFF).shr(9)) {
            if !rpm_offset(entity_pawn_list_entry, 0x10 + sum, &mut entity_pawn_list_entry) {
                return 0;
            }
        } else {
            return 0;
        }

        if let Some(sum) = (0x78 as u64).checked_mul(self.pawn.bitand(0x1FF)) {
            if !rpm_offset(entity_pawn_list_entry, sum, &mut entity_pawn_address) {
                return 0;
            }
        } else {
            return 0;
        }

        return entity_pawn_address;
    }
}

impl PlayerPawn {
    pub fn get_view_angle(&mut self) -> bool {
        return rpm_offset(self.address, Offsets::C_CSPlayerPawnBase::m_angEyeAngles as u64, &mut self.view_angle);
    }

    pub fn get_camera_pos(&mut self) -> bool {
        return rpm_offset(self.address, Offsets::C_CSPlayerPawnBase::m_vecLastClipCameraPos as u64, &mut self.camera_pos);
    }

    pub fn get_spotted(&mut self) -> bool {
        return rpm_offset(self.address, (Offsets::C_CSPlayerPawnBase::m_entitySpottedState + Offsets::EntitySpottedState_t::m_bSpottedByMask) as u64, &mut self.spotted_by_mask);
    }

    pub fn get_weapon(&mut self) -> bool {
        // Weapon Name & Type
        let weapon_name_address = trace_address(self.address + Offsets::C_CSPlayerPawnBase::m_pClippingWeapon as u64, &[0x10, 0x20, 0x0]);
        let mut buffer: [u8; 40] = [0; 40];

        if weapon_name_address == 0 {
            return false;
        }
        
        if !rpm(weapon_name_address, &mut buffer, 40) {
            return false;
        }
        
        let weapon_name = buffer_to_string(&buffer).to_lowercase().replace("weapon_", "");

        if !weapon_name.is_empty() {
            let (wtype, name) = parse_weapon(weapon_name.clone());

            self.weapon_type = wtype;
            self.weapon_name = if name == "" { weapon_name } else { name.to_string() };
        }

        // Weapon Data
        let mut clipping_weapon: u64 = 0;
        let mut weapon_data: u64 = 0;

        if !rpm_offset(self.address, Offsets::C_CSPlayerPawnBase::m_pClippingWeapon as u64, &mut clipping_weapon) {
            return false;
        }

        if !rpm_offset(clipping_weapon, 0x360, &mut weapon_data) {
            return false;
        }

        // Weapon Max Ammo
        if !rpm_offset(weapon_data, Offsets::CBasePlayerWeaponVData::m_iMaxClip1 as u64, &mut self.weapon_max_ammo) {
            return false;
        }

        // Weapon Ammo
        if !rpm_offset(clipping_weapon, Offsets::C_BasePlayerWeapon::m_iClip1 as u64, &mut self.weapon_ammo) {
            return false;
        }
        
        return true;
    }

    pub fn get_shots_fired(&mut self) -> bool {
        return rpm_offset(self.address, Offsets::C_CSPlayerPawnBase::m_iShotsFired as u64, &mut self.shots_fired);
    }

    pub fn get_aim_punch_cache(&mut self) -> bool {
        return rpm_offset(self.address, Offsets::C_CSPlayerPawn::m_aimPunchCache as u64, &mut self.aim_punch_cache);
    }

    pub fn get_pos(&mut self) -> bool {
        return rpm_offset(self.address, Offsets::C_BasePlayerPawn::m_vOldOrigin as u64, &mut self.pos);
    }

    pub fn get_health(&mut self) -> bool {
        return rpm_offset(self.address, Offsets::C_BaseEntity::m_iHealth as u64, &mut self.health);
    }

    pub fn get_armor(&mut self) -> bool {
        return rpm_offset(self.address, Offsets::C_CSPlayerPawnBase::m_ArmorValue as u64, &mut self.armor);
    }

    pub fn get_fov(&mut self) -> bool {
        let mut camera_services = 0;

        if !rpm_offset(self.address, Offsets::C_BasePlayerPawn::m_pCameraServices as u64, &mut camera_services) {
            return false;
        }

        return rpm_offset(camera_services, Offsets::CCSPlayerBase_CameraServices::m_iFOVStart as u64, &mut self.fov);
    }

    pub fn get_f_flags(&mut self) -> bool {
        return rpm_offset(self.address, Offsets::C_BaseEntity::m_fFlags as u64, &mut self.flags);
    }

    pub fn has_flag(&mut self, flag: Flags) -> bool {
        return self.flags & (flag as i32) != 0;
    }
}