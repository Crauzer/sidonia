use crate::core::{
    msvc::{smart_pointers::StdSharedPtr, string::StdString, vector::StdVector},
    riot::{
        net::{RiotNetId, RiotNetVisibilityObject, RiotOnNetworkRegister},
        r3d::texture::R3dTexture,
        spellbook::{RiotSpellbook, RiotSpellbookOwnerOverride},
        talent::RiotTalent,
    },
};
use winapi::shared::minwindef::LPVOID;

#[repr(C)]
pub struct RiotAvatar {
    spellbook: RiotSpellbook,
    pimpl: StdSharedPtr<RiotAvatarImpl>,
}

#[repr(C)]
struct RiotAvatarImpl {
    on_network_register: RiotOnNetworkRegister<!>, // T: PKT_AvatarInfo_Server_s
    owner_id: RiotNetId,
    spellbook_owner_override: RiotSpellbookOwnerOverride,
    no_issue_order: RiotNoIssueOrder,
    avatar_info: RiotAvatarInfo,
    net_visibility_object: *mut RiotNetVisibilityObject,
    talents: StdVector<StdString>,
    parent: *mut RiotAvatar,
    avatar_icon: *mut R3dTexture,
}

#[repr(C)]
pub struct RiotNoIssueOrder {
    vtable: LPVOID,
}

#[repr(C)]
pub struct RiotAvatarInfo {
    item_ids: [u32; 30],
    spell_hashes: [u32; 2],
    talents_hashes: [RiotTalent; 80],
    level: u8,
    ward_skin: u8,
}
