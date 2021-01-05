use smash::app::sv_module_access::*;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::phx::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::app::sv_battle_object::module_accessor;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::USE_AIR_SIDE_SPECIAL;
use skyline::nn::ro::LookupSymbol;
use smash::hash40;
use acmd::{acmd, acmd_func};

pub static mut GANONDORF_ENCHANTMENT: [bool; 8] = [false; 8];
pub static mut GANONDORF_NEUTRAL_SPECIAL_STATE: [bool; 8] = [false; 8];

//Jab
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_11",
    animcmd = "game_attack11")]
    pub fn Swordfighter_Ganondorf_Jab(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                frame(Frame=4)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=7.0, Angle=361, KBG=85, FKB=0, BKB=50, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=4)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=59)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
        }
//Jab Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_11",
    animcmd = "effect_attack11_ganon")]
pub fn Swordfighter_Ganondorf_Jab_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(4)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(8)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(15)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Dash Attack
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_dash",
    animcmd = "game_attackdash")]
    pub fn Swordfighter_Ganondorf_Dash_Attack(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                frame(Frame=15)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=14.0, Angle=55, KBG=95, FKB=0, BKB=35, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=3)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=59)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
        }
//Dash Attack Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_dash",
    animcmd = "effect_attackdash_ganon")]
pub fn Swordfighter_Ganondorf_Dash_Attack_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(14)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(18)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(20)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Forward Tilt
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_s3_s",
    animcmd = "game_attacks3")]
    pub fn Swordfighter_Ganondorf_Forward_Tilt(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                frame(Frame=13)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=16.0, Angle=22, KBG=65, FKB=0, BKB=45, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=3)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=59)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
        }
//Forward Tilt Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_s3_s",
    animcmd = "effect_attacks3_ganon")]
pub fn Swordfighter_Ganondorf_Forward_Tilt_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(13)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(15)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(18)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Up Tilt
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
    pub fn Swordfighter_Ganondorf_Up_Tilt(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                frame(Frame=11)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=90, KBG=75, FKB=0, BKB=45, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=4)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=58)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
        }
//Up Tilt Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_hi3",
    animcmd = "effect_attackhi3_ganon")]
pub fn Swordfighter_Ganondorf_Up_Tilt_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(11)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(15)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(18)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Down Tilt
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
    pub fn Swordfighter_Ganondorf_Down_Tilt(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                frame(Frame=7)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=75, KBG=55, FKB=0, BKB=45, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=2)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=43)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
        }
//Down Tilt Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_lw3",
    animcmd = "effect_attacklw3_ganon")]
pub fn Swordfighter_Ganondorf_Down_Tilt_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(3)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(9)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(16)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Forward Smash
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_s4_s",
    animcmd = "game_attacks4")]
    pub fn Swordfighter_Ganondorf_Forward_Smash(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                    frame(Frame=1)
                    if(is_excute){
                    WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
                    }
                    frame(Frame=12)
                    if(is_excute){
                    sv_animcmd::EFFECT_FOLLOW(hash40("ganon_attack_purple"), hash40("arml"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, true);
                    ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=5.0, Angle=195, KBG=55, FKB=45, BKB=0, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0.0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
                    }
                    wait(Frames=4)
                    if(is_excute){
                    AttackModule::clear_all()
                    }
                    frame(Frame=41)
                    if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=18.0, Angle=40, KBG=65, FKB=0, BKB=70, Size=4.5, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    }
                    wait(Frames=4)
                    if(is_excute){
                    AttackModule::clear_all()
                    }
                    frame(Frame=81)
                    if(is_excute){
                        rust{
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
        }
//Forward Smash Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_s4_s",
    animcmd = "effect_attacks4_ganon")]
pub fn Swordfighter_Ganondorf_Forward_Smash_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(40)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(41)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(47)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Down Smash
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
    pub fn Swordfighter_Ganondorf_Down_Smash(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if (is_excute){
                rust{
                    ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                }
            }
            frame(Frame=5)
            if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            }
            frame(Frame=19)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=367, KBG=90, FKB=45, BKB=0, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=6.0, Angle=367, KBG=65, FKB=45, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
            }
            wait(Frames=3)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=25)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=367, KBG=90, FKB=45, BKB=0, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=6.0, Angle=367, KBG=65, FKB=45, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
            }
            wait(Frames=3)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=32)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=35, KBG=81, FKB=0, BKB=61, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=6.0, Angle=35, KBG=65, FKB=0, BKB=45, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
            }
            wait(Frames=3)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=37)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=35, KBG=81, FKB=0, BKB=61, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=6.0, Angle=35, KBG=65, FKB=0, BKB=45, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
            }
            wait(Frames=2)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=64)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
        });
    }
//Down Smash Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_lw4",
    animcmd = "effect_attacklw4_ganon")]
pub fn Swordfighter_Ganondorf_Down_Smash_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(17)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(19)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(35)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Nair
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
    pub fn Swordfighter_Ganondorf_Nair(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                frame(Frame=1)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=5)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=4.0, Angle=367, KBG=30, FKB=0, BKB=20, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=367, KBG=30, FKB=0, BKB=20, Size=8.0, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
                }
                wait(Frames=3)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=13)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=4.0, Angle=367, KBG=30, FKB=0, BKB=20, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=367, KBG=30, FKB=0, BKB=20, Size=8.0, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
                }
                wait(Frames=6)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=21)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=361, KBG=95, FKB=0, BKB=45, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=65, FKB=0, BKB=20, Size=8.0, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
                }
                wait(Frames=8)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=40)
                if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=47)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                }
            });
        }
//Nair Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_air_n",
    animcmd = "effect_attackairn_ganon")]
pub fn Swordfighter_Ganondorf_Nair_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(5)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(9)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(30)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Fair
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
    pub fn Swordfighter_Ganondorf_Fair(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                frame(Frame=1)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=13)
                if(is_excute){
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=17.0, Angle=280, KBG=45, FKB=0, BKB=30, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=13.0, Angle=361, KBG=75, FKB=0, BKB=55, Size=5.0, X=0.0, Y=10.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=5)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=50)
                if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=54)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                }
            });
        }
//Fair Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_air_f",
    animcmd = "effect_attackairf_ganon")]
pub fn Swordfighter_Ganondorf_Fair_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(10)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(15)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(19)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Bair
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
    pub fn Swordfighter_Ganondorf_Bair(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                frame(Frame=1)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=7)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=15.0, Angle=135, KBG=65, FKB=0, BKB=55, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=2)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=40)
                if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=60)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                }
            });
        }
//Bair Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_air_b",
    animcmd = "effect_attackairb_ganon")]
pub fn Swordfighter_Ganondorf_Bair_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(2)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(9)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(11)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Uair
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
    pub fn Swordfighter_Ganondorf_Uair(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                frame(Frame=1)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=12)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=15.0, Angle=85, KBG=65, FKB=0, BKB=45, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=8.0, Angle=85, KBG=65, FKB=0, BKB=45, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
                }
                wait(Frames=7)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=45)
                if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=62)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                }
            });
        }
//Uair Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_air_hi",
    animcmd = "effect_attackairhi_ganon")]
pub fn Swordfighter_Ganondorf_Uair_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(10)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(9)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(19)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Dair
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
    pub fn Swordfighter_Ganondorf_Dair(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if (is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                    }
                }
                frame(Frame=1)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=13)
                if(is_excute){
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=15.0, Angle=280, KBG=45, FKB=0, BKB=45, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=13.0, Angle=361, KBG=65, FKB=0, BKB=45, Size=5.0, X=0.0, Y=8.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=6)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=45)
                if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=67)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                }
            });
        }
//Dair Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "attack_air_lw",
    animcmd = "effect_attackairlw_ganon")]
pub fn Swordfighter_Ganondorf_Dair_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(5)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(11)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(13)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Up Throw
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "throw_hi",
    animcmd = "game_throwhi")]
pub fn Swordfighter_Ganondorf_Up_Throw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if (is_excute){
            rust{
                ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
            }
        }
        if(is_excute){
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=3.0, Angle=90, KBG=105, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=11)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=10.0, Angle=140, KBG=100, FKB=0, BKB=30, Size=4.3, X=4.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("shoulderr"), Damage=10.0, Angle=140, KBG=100, FKB=0, BKB=30, Size=3.8, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=10.0, Angle=140, KBG=100, FKB=0, BKB=30, Size=3.7, X=-0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_catch_only_all(true, false)
        CHECK_FINISH_CAMERA(-1, 28)
        }
        frame(Frame=13)
        if(is_excute){
        ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }  
        frame(Frame=14)
        if(is_excute){
        AttackModule::clear_all()
        }
    });
}
//Neutral Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_n",
    animcmd = "game_specialn")]
    pub fn Swordfighter_Ganondorf_Neutral_Special(fighter: &mut L2CFighterCommon) {
        acmd!({
            if (is_excute){
                rust{
                    ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                    ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                }
            }
            frame(Frame=1)
            if(is_excute){
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
            }
            frame(Frame=12)
            if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN)
            }
            frame(Frame=23)
            if(is_excute){
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
            }
            frame(Frame=24)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=15.0, Angle=35, KBG=55, FKB=0, BKB=45, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=81)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=93)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
        });
    }
//Neutral Special Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_n",
    animcmd = "effect_specialn_ganon")]
pub fn Swordfighter_Ganondorf_Neutral_Special_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(24)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(25)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(81)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
        }
    });
}
//Reverse Neutral Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_n_turn",
    animcmd = "game_specialnturn")]
    pub fn Swordfighter_Ganondorf_Reverse_Neutral_Special(fighter: &mut L2CFighterCommon) {
        acmd!({
            if (is_excute){
                rust{
                    ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                    ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                }
            }
            frame(Frame=1)
            if(is_excute){
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
            }
            frame(Frame=12)
            if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN)
            }
            frame(Frame=23)
            if(is_excute){
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
            }
            frame(Frame=24)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=18.0, Angle=40, KBG=60, FKB=0, BKB=50, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=81)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=93)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
        });
    }
//Reverse Neutral Special Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_n_turn",
    animcmd = "effect_specialnturn_ganon")]
pub fn Swordfighter_Ganondorf_Reverse_Neutral_Special_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(24)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(25)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(81)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
        }
    });
}
//Aerial Neutral Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_air_n",
    animcmd = "game_specialairn")]
    pub fn Swordfighter_Ganondorf_Aerial_Neutral_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
        if (is_excute){
            rust{
                let kind = smash::app::utility::get_kind(module_accessor);
                ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
        }
        frame(Frame=4)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN)
        }
        frame(Frame=9)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=4.0, Angle=367, KBG=60, FKB=150, BKB=0, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=19)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=20)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=4.0, Angle=367, KBG=60, FKB=150, BKB=0, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=36)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=37)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=361, KBG=85, FKB=0, BKB=35, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=50)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=74)
            if(is_excute){
                rust{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        });
    }
//Aerial Neutral Special Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_air_n",
    animcmd = "effect_specialairn_ganon")]
    pub fn Swordfighter_Ganondorf_Aerial_Neutral_Special_Effect(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            frame(9)
            if(is_excute){
                AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(10)
            if(is_excute){
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(51)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
            }
        });
    }
//Aerial Reverse Neutral Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_air_n_turn",
    animcmd = "game_specialairnturn")]
    pub fn Swordfighter_Ganondorf_Aerial_Reverse_Neutral_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
        if (is_excute){
            rust{
                let kind = smash::app::utility::get_kind(module_accessor);
                ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
        }
        frame(Frame=4)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN)
        }
        frame(Frame=9)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=4.0, Angle=367, KBG=60, FKB=150, BKB=0, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=19)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=20)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=4.0, Angle=367, KBG=60, FKB=150, BKB=0, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=36)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=37)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=361, KBG=85, FKB=0, BKB=35, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=50)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=74)
            if(is_excute){
                rust{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        });
    }
//Aerial Reverse Neutral Special Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_air_n_turn",
    animcmd = "effect_specialairnturn_ganon")]
    pub fn Swordfighter_Ganondorf_Aerial_Reverse_Neutral_Special_Effect(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            frame(9)
            if(is_excute){
                AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(10)
            if(is_excute){
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(51)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
            }
        });
    }
//Aerial Side Special Start
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_air_s_start",
    animcmd = "game_specialairsstart")]
    pub fn Swordfighter_Ganondorf_Aerial_Side_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if (is_excute){
                rust{
                    ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                    ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                }
            }
            frame(Frame=1)
            if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            FT_MOTION_RATE(FSM=0.8)
            }
            frame(Frame=27)
            if(is_excute){
                rust {
                    let speedVec = Vector3f{x: 0.1, y: 0.0, z: 0.0};
                    KineticModule::add_speed(module_accessor, &speedVec);
                  }
                  ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=361, KBG=75, FKB=0, BKB=45, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            FT_MOTION_RATE(FSM=0.8)
            frame(Frame=29)
            if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GRAVITY_ONOFF)
            }
            frame(Frame=31)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=48)
            if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=57)
            if(is_excute){
                StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_FALL, false);
            }
        });
    }
//Aerial Side Special Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_air_s_start",
    animcmd = "effect_specialairsstart_ganon")]
pub fn Swordfighter_Ganondorf_Aerial_Side_Special_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(26)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(29)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(31)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
		}
    });
}
//Up Special Start
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_hi",
    animcmd = "game_specialhi")]
    pub fn Swordfighter_Ganondorf_Up_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if (is_excute){
            rust{
                ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
            }
        }   
            frame(Frame=12)
            if(is_excute){
                rust {
                    let speedVec = Vector3f{x: 0.65, y: 0.0, z: 0.0};
                    KineticModule::add_speed(module_accessor, &speedVec);
                  }
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=13)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=14)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=15)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=16)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=17)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=18)
            if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
            AttackModule::clear_all()
            }
            frame(Frame=19)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=20)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=21)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=22)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=31)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=367, KBG=35, FKB=200, BKB=0, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=38)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=46)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=361, KBG=75, FKB=0, BKB=45, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=5)
            if(is_excute){
            AttackModule::clear_all()
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE)
            }
            frame(Frame=61)
            if(is_excute){
                StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            }
        });
    }
//Aerial Up Special Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_hi",
    animcmd = "effect_specialhi_ganon")]
pub fn Swordfighter_Ganondorf_Up_Special_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(26)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(29)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(38)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
        }
        frame(43)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
        }
        frame(47)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(50)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
        }
    });
}
//Aerial Up Special Start
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_air_hi",
    animcmd = "game_specialairhi")]
    pub fn Swordfighter_Ganondorf_Aerial_Up_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if (is_excute){
            rust{
                ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
            }
        }
            frame(Frame=12)
            if(is_excute){
                rust {
                    let speedVec = Vector3f{x: 0.65, y: 0.0, z: 0.0};
                    KineticModule::add_speed(module_accessor, &speedVec);
                  }
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            }
            frame(Frame=13)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=14)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=15)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=16)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=17)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=18)
            if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
            AttackModule::clear_all()
            }
            frame(Frame=19)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=20)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=21)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=367, KBG=35, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=22)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=31)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=367, KBG=35, FKB=200, BKB=0, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=38)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=46)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=361, KBG=75, FKB=0, BKB=45, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=5)
            if(is_excute){
            AttackModule::clear_all()
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE)
            }
            frame(Frame=61)
            if(is_excute){
                StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            }
        });
    }
//Aerial Up Special Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_air_hi",
    animcmd = "effect_specialairhi_ganon")]
pub fn Swordfighter_Ganondorf_Aerial_Up_Special_Effect(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(26)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
		}
		frame(29)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(38)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
        }
        frame(43)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
        }
        frame(47)
		if(is_excute){
			LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(50)
		if(is_excute){
			AFTER_IMAGE_OFF(4)
        }
    });
}
//Down Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_lw",
    animcmd = "game_speciallw")]
    pub fn Swordfighter_Ganondorf_Down_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if (is_excute){
                rust{
                    ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                    ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                }
            }
            frame(Frame=16)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=14.0, Angle=45, KBG=65, FKB=0, BKB=65, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=1)
            if(is_excute){
                rust{
                    WorkModule::on_flag(module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
                }
            }
            frame(Frame=36)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=37)
            if(is_excute){
                StatusModule::change_status_request_from_script(*FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, true);
            }
        });
    }
//Down Special Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_lw",
    animcmd = "effect_speciallw_ganon")]
    pub fn Swordfighter_Ganondorf_Down_Special_Effect(fighter: &mut L2CFighterCommon)
    {
        acmd!({		
            frame(16)
            if(is_excute){
                AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(17)
            if(is_excute){
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(37)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
            }
        });
    }
//Aerial Down Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_air_lw",
    animcmd = "game_specialairlw")]
    pub fn Swordfighter_Ganondorf_Aerial_Down_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if (is_excute){
                rust{
                    ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                    ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                }
            }
            frame(Frame=16)
            if(is_excute){
                rust{
                    WorkModule::on_flag(module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
                }
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=15.0, Angle=290, KBG=70, FKB=0, BKB=40, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            JostleModule::set_status(false)
            }
            frame(Frame=19)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=14.0, Angle=80, KBG=70, FKB=0, BKB=40, Size=5.0, X=0.0, Y=15.0, Z=0.0, X2=0.0, Y2=5.0, Z2=0.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=27)
            if(is_excute){
            AttackModule::clear_all()
            JostleModule::set_status(true)
            }
            frame(Frame=28)
            if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_FALL, true);
            }
        });
    }
//Aerial Down Special Effect
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_GANON, 
    animation = "special_air_lw",
    animcmd = "effect_specialairlw_ganon")]
    pub fn Swordfighter_Ganondorf_Aerial_Down_Special_Effect(fighter: &mut L2CFighterCommon)
    {
        acmd!({		
            frame(16)
            if(is_excute){
                AFTER_IMAGE4_ON_arg29(0x10523a2bc8 as u64, 0x10cb337a72 as u64, 7, hash40("haver"), 0, 2.29999995, 0, hash40("haver"), 0, 18.5, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(17)
            if(is_excute){
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(27)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
            }
        });
    }
//Down Taunt
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_lwr",
    animcmd = "game_appeallwr")]
    pub fn Swordfighter_Ganondorf_Down_Taunt(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if (is_excute){
                rust{
                    ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                    ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                }
            }
            frame(Frame=70)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
        });
    }
pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
      let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
      let kind = smash::app::utility::get_kind(module_accessor);
      let status_kind = StatusModule::status_kind(module_accessor);
      let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
      let motion_kind = MotionModule::motion_kind(module_accessor);
      let pos = Vector3f{x: 0.0, y: 10.0, z: 0.0};
      let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
          
      if kind == *FIGHTER_KIND_GANON {
        if StatusModule::situation_kind(module_accessor) == SITUATION_KIND_GROUND
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
        || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL {
          USE_AIR_SIDE_SPECIAL[entry_id] = true;
        }
        if status_kind == FIGHTER_STATUS_KIND_SPECIAL_S {
            USE_AIR_SIDE_SPECIAL[entry_id] = false;
          }
          if MotionModule::motion_kind(module_accessor) == hash40("appeal_lw_r")
          || MotionModule::motion_kind(module_accessor) == hash40("appeal_lw_l") 
          && MotionModule::frame(module_accessor) == 0.0 {
              if GANONDORF_ENCHANTMENT[entry_id] {
                GANONDORF_ENCHANTMENT[entry_id] = false;
              }
              else {
                GANONDORF_ENCHANTMENT[entry_id] = true;
              }
          }
          if GANONDORF_ENCHANTMENT[entry_id] {
              AttackModule::set_power_mul(module_accessor, 1.2);
              DamageModule::set_reaction_mul(module_accessor, 1.2);
                  EffectModule::req_follow(
                      module_accessor,
                      Hash40{hash: hash40("ganon_attack_purple")},
                      Hash40{hash: hash40("haver")},
                      &pos,
                      &zeros,
                      1.5 * 1.0,
                      true,
                      *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32
                      | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32
                      | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32,
                      0,
                      0,
                      0,
                      0,
                      true,
                      true,
                  );
            }
            else {
              AttackModule::set_power_mul(module_accessor, 1.0);
              DamageModule::set_reaction_mul(module_accessor, 1.0);
              EffectModule::kill_kind(module_accessor, Hash40{hash: hash40("ganon_attack_purple")}, false, true);
            }
        if GANONDORF_ENCHANTMENT[entry_id] {
            AttackModule::set_power_mul(module_accessor, 1.2);
            DamageModule::set_reaction_mul(module_accessor, 1.2);
                EffectModule::req_follow(
                    module_accessor,
                    Hash40{hash: hash40("ganon_attack_purple")},
                    Hash40{hash: hash40("haver")},
                    &pos,
                    &zeros,
                    1.5 * 1.0,
                    true,
                    *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32
                    | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32
                    | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32,
                    0,
                    0,
                    0,
                    0,
                    true,
                    true,
                );
          }
          else {
            AttackModule::set_power_mul(module_accessor, 1.0);
            DamageModule::set_reaction_mul(module_accessor, 1.0);
            EffectModule::kill_kind(module_accessor, Hash40{hash: hash40("ganon_attack_purple")}, false, true);
          }
          if motion_kind == hash40("special_air_n") 
          || motion_kind == hash40("special_air_n_turn") 
          || motion_kind == hash40("special_n") 
          || motion_kind == hash40("special_n_turn") {
            if motion_kind == hash40("special_air_n") 
            || motion_kind == hash40("special_air_n_turn") {
              GANONDORF_NEUTRAL_SPECIAL_STATE[entry_id] = true;
           }
           else {
             if GANONDORF_NEUTRAL_SPECIAL_STATE[entry_id] {
               StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
               AttackModule::clear_all(module_accessor);
             }
           }
         }
         else {
           GANONDORF_NEUTRAL_SPECIAL_STATE[entry_id] = false;
         }
        }
        if smash::app::utility::get_kind(module_accessor) == *FIGHTER_KIND_GANON {
            println!("Ganon current status: {}", StatusModule::status_kind(module_accessor));  
        }
    }
}
        pub fn install() {
            acmd::add_custom_hooks!(once_per_fighter_frame);
            acmd::add_hooks!(
                Swordfighter_Ganondorf_Jab,
                Swordfighter_Ganondorf_Jab_Effect,
                Swordfighter_Ganondorf_Dash_Attack,
                Swordfighter_Ganondorf_Dash_Attack_Effect,
                Swordfighter_Ganondorf_Forward_Tilt,
                Swordfighter_Ganondorf_Forward_Tilt_Effect,
                Swordfighter_Ganondorf_Up_Tilt,
                Swordfighter_Ganondorf_Up_Tilt_Effect,
                Swordfighter_Ganondorf_Down_Tilt,
                Swordfighter_Ganondorf_Down_Tilt_Effect,
                Swordfighter_Ganondorf_Forward_Smash,
                Swordfighter_Ganondorf_Forward_Smash_Effect,
                Swordfighter_Ganondorf_Down_Smash,
                Swordfighter_Ganondorf_Down_Smash_Effect,
                Swordfighter_Ganondorf_Nair,
                Swordfighter_Ganondorf_Nair_Effect,
                Swordfighter_Ganondorf_Fair,
                Swordfighter_Ganondorf_Fair_Effect,
                Swordfighter_Ganondorf_Bair,
                Swordfighter_Ganondorf_Bair_Effect,
                Swordfighter_Ganondorf_Uair,
                Swordfighter_Ganondorf_Uair_Effect,
                Swordfighter_Ganondorf_Dair,
                Swordfighter_Ganondorf_Dair_Effect,
                Swordfighter_Ganondorf_Up_Throw,
                Swordfighter_Ganondorf_Neutral_Special,
                Swordfighter_Ganondorf_Neutral_Special_Effect,
                Swordfighter_Ganondorf_Reverse_Neutral_Special,
                Swordfighter_Ganondorf_Reverse_Neutral_Special_Effect,
                Swordfighter_Ganondorf_Aerial_Neutral_Special,
                Swordfighter_Ganondorf_Aerial_Neutral_Special_Effect,
                Swordfighter_Ganondorf_Aerial_Reverse_Neutral_Special,
                Swordfighter_Ganondorf_Aerial_Reverse_Neutral_Special_Effect,
                Swordfighter_Ganondorf_Aerial_Side_Special,
                Swordfighter_Ganondorf_Aerial_Side_Special_Effect,
                Swordfighter_Ganondorf_Up_Special,
                Swordfighter_Ganondorf_Up_Special_Effect,
                Swordfighter_Ganondorf_Down_Special,
                Swordfighter_Ganondorf_Down_Special_Effect,
                Swordfighter_Ganondorf_Aerial_Up_Special,
                Swordfighter_Ganondorf_Aerial_Up_Special_Effect,
                Swordfighter_Ganondorf_Aerial_Down_Special,
                Swordfighter_Ganondorf_Aerial_Down_Special_Effect,
                Swordfighter_Ganondorf_Down_Taunt,
            );
        }