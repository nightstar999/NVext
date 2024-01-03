// Copyright (c) 2023 Snipcola - Nightstar <nightstar6@protonmail.com>
// SPDX-License-Identifier: MIT

#![allow(non_snake_case, non_upper_case_globals)]

pub mod Offsets {
    pub mod C_BaseEntity {  // CEntityInstance
        pub const m_iHealth: usize = 0x32C; // int32_t
        pub const m_iTeamNum: usize = 0x3BF; // uint8_t
        pub const m_pGameSceneNode: usize = 0x310; // CGameSceneNode*
        pub const m_fFlags: usize = 0x3C8; // uint32_t
    }
    
    pub mod CBasePlayerController { // C_BaseEntity
        pub const m_hPawn: usize = 0x60C; // CHandle<C_BasePlayerPawn>
        pub const m_iszPlayerName: usize = 0x640; // char[128]
    }
    
    pub mod CCSPlayerController { // CBasePlayerController
        pub const m_hPlayerPawn: usize = 0x7EC; // CHandle<C_CSPlayerPawn>
        pub const m_bPawnIsAlive: usize = 0x7F4; // bool
    }
    
    pub mod C_BasePlayerPawn { // C_BaseCombatCharacter
        pub const m_pObserverServices: usize = 0x10C0; // CPlayer_ObserverServices*
        pub const m_pCameraServices: usize = 0x10E0; // CPlayer_CameraServices*
        pub const m_vOldOrigin: usize = 0x1224; // Vector
    }

    pub mod C_CSPlayerPawnBase { // C_BasePlayerPawn
        pub const m_vecLastClipCameraPos: usize = 0x1294; // Vector
        pub const m_angEyeAngles: usize = 0x1518; // QAngle
        pub const m_pClippingWeapon: usize = 0x12B0; // C_CSWeaponBase*
        pub const m_iIDEntIndex: usize = 0x1544; // CEntityIndex
        pub const m_entitySpottedState: usize = 0x1638; // EntitySpottedState_t
        pub const m_ArmorValue: usize = 0x1510; // int32_t
        pub const m_iShotsFired: usize = 0x1420; // int32_t
    }

    pub mod C_CSPlayerPawn { // C_CSPlayerPawnBase
        pub const m_aimPunchCache: usize = 0x1740; // CUtlVector<QAngle>
    }

    pub mod CGameSceneNode {
        pub const m_vecAbsOrigin: usize = 0xC8; // Vector
    }

    pub mod CCSPlayerBase_CameraServices { // CPlayer_CameraServices
        pub const m_iFOVStart: usize = 0x214; // uint32_t
    }

    pub mod EntitySpottedState_t {        
        pub const m_bSpottedByMask: usize = 0xC; // uint32_t[2]
    }

    pub mod CompositeMaterialEditorPoint_t {
        pub const m_vecCompositeMaterialAssemblyProcedures: usize = 0x1E0; // CUtlVector<CompositeMaterialAssemblyProcedure_t>
    }

    pub mod CPlayer_ObserverServices { // CPlayerPawnComponent
        pub const m_hObserverTarget: usize = 0x44; // CHandle<C_BaseEntity>
    }

    pub mod C_PlantedC4 { // CBaseAnimGraph
        pub const m_nBombSite: usize = 0xE84; // int32_t
    }

    pub mod CBasePlayerWeaponVData { // CEntitySubclassVDataBase
        pub const m_iMaxClip1: usize = 0x1FC; // int32_t
    }

    pub mod C_BasePlayerWeapon { // C_EconEntity
        pub const m_iClip1: usize = 0x1570; // int32_t
    }
}

pub mod Signatures {
    // https://github.com/a2x/cs2-dumper/blob/main/config.json //
    pub const dwEntityList: &str = "48 8B 0D ?? ?? ?? ?? 48 89 7C 24 ?? 8B FA C1 EB";
    pub const dwLocalPlayerController: &str = "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 4F";
    pub const dwLocalPlayerPawn: &str = "48 8D 05 ?? ?? ?? ?? C3 CC CC CC CC CC CC CC CC 48 83 EC ?? 8B 0D";
    pub const dwPlantedC4: &str = "48 8B 15 ?? ?? ?? ?? FF C0 48 8D 4C 24 40";
    pub const dwViewAngles: &str = "48 8B 0D ?? ?? ?? ?? E9 ?? ?? ?? ?? CC CC CC CC 40 55";
    pub const dwViewMatrix: &str = "48 8D 0D ?? ?? ?? ?? 48 C1 E0 06";
}

pub mod ProgramConfig {
    pub mod Package {
        pub const Name: &str = "nvext";
        pub const Description: &str = "An open-source, external CS2 enhancer.";
        pub const Executable: &str = "nvext.exe";
        pub const Version: &str = env!("CARGO_PKG_VERSION");
        pub const Authors: &str = &env!("CARGO_PKG_AUTHORS");
    }

    pub mod Imgui {
        pub const FontSize: f32 = 13.0;

        pub mod FontPaths {
            pub const Chinese: &str = "C:/Windows/Fonts/msyh.ttc";
            pub const Cryillic: &str = "C:/Windows/Fonts/Arial.ttf";
            pub const Arabic: &str = "C:/Windows/Fonts/calibri.ttf";
        }
    }

    pub mod Update {
        pub const Enabled: bool = false;
        pub const URL: &str = "";
        pub const HashURL: &str = "";
    }

    pub mod RPC {
        pub const Enabled: bool = false;
        pub const ClientID: u64 = 0;
        pub const ImageAsset: &str = "";
    }

    pub mod Links {
        pub const Source: &str = "https://github.com/nightstar999/NVext";
        pub const License: &str = "https://raw.githubusercontent.com/nightstar999/NVext/main/LICENSE";
        pub const Discord: &str = "https://discord.gg/8t2vNHMJW6"; // Original author discord, snipcola
    }

    pub mod Keys {
        use glutin::event::VirtualKeyCode;
        use mki::Keyboard;

        pub const Available: [&str; 20] = ["Alt", "Left Mouse", "Middle Mouse", "Right Mouse", "Side Mouse", "Extra Mouse", "Shift", "Control", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"];

        pub const ToggleKey: VirtualKeyCode = VirtualKeyCode::Insert;
        pub const ToggleKeyMKI: Keyboard = Keyboard::Insert;

        pub const ExitKey: VirtualKeyCode = VirtualKeyCode::End;
        pub const ExitKeyMKI: Keyboard = Keyboard::Other(0x23);
    }

    pub mod TargetProcess {
        pub const Executable: &str = "cs2.exe";
        pub const MaxAttempts: u32 = 30;
        pub const UpdateOffsetsMaxAttempts: u32 = 15;
        pub const InitAddressesMaxAttempts: u32 = 15;

        pub mod Window {
            pub const Title: &str = "Counter-Strike 2";
            pub const Class: &str = "SDL_app";
        }
    }

    pub mod CheckDelays {
        use std::time::Duration;

        pub const AttachProcess: Duration = Duration::from_millis(1000);
        pub const UpdateOffsets: Duration = Duration::from_millis(1000);
        pub const InitAddresses: Duration = Duration::from_millis(1000);
    }

    pub mod ThreadDelays {
        use std::time::Duration;
        
        pub const UpdateConfigs: Duration = Duration::from_millis(250);
        pub const WindowTasks: Duration = Duration::from_millis(25);
        pub const IOTasks: Duration = Duration::from_millis(25);
        pub const RPC: Duration = Duration::from_millis(100);
    }

    pub mod CheatDelays {
        use std::time::Duration;

        pub const Toggle: Duration = Duration::from_millis(200);
        pub const Aimbot: Duration = Duration::from_millis(10);
        pub const AimbotOffEntity: Duration = Duration::from_millis(500);
        pub const TriggerbotOffEntity: Duration = Duration::from_millis(500);
    }
}