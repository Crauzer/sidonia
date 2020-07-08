use crate::core::msvc::smart_pointers::StdSharedPtr;
use crate::core::riot::net::{RiotOnNetworkRegister, RiotNetId, RiotNetVisibilityObject};
use crate::core::riot::spellbook::{RiotSpellbook, RiotSpellbookOwnerOverride};
use winapi::shared::minwindef::LPVOID;
use crate::core::riot::talent::RiotTalent;
use crate::core::msvc::vector::StdVector;
use crate::core::msvc::string::StdString;
use crate::core::riot::r3d::texture::R3dTexture;

#[repr(C)]
pub struct RiotAvatar {
    spellbook: RiotSpellbook,
    pimpl: StdSharedPtr<RiotAvatarImpl>
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
    avatar_icon: *mut R3dTexture
}

#[repr(C)]
pub struct RiotNoIssueOrder {
    vtable: LPVOID
}

#[repr(C)]
pub struct RiotAvatarInfo {
    item_ids: [u32; 30],
    spell_hashes: [u32; 2],
    talents_hashes: [RiotTalent; 80],
    level: u8,
    ward_skin: u8
}