use crate::core::riot::camera_logic::RiotCameraLogic;

#[repr(C)]
#[derive(Debug)]
pub struct RiotHudManager {
    camera_logic: *mut RiotCameraLogic,
    /* HudDragScaleLogic *mDragScaleLogic;
    HudCursorTargetLogic *mCursorTargetLogic;
    HudFeedbackDamage *mFeedbackDamage;
    HudFeedbackStun *mFeedbackStun;
    HudInputLogic *mInputLogic;
    HudPetLogic *mPetLogic;
    HudRadialMenuLogic *mRadialMenuLogic;
    HudReplayLogic *mReplayLogic;
    HudSelectLogic *mSelectLogic;
    HudSpellLogic *mSpellLogic;
    HudHeatmapWriter *mHeatmapWriter;
    HUDOptionsManager *mHUDOptionsManager;
    HudManager::EventList mEvents;
    HudManager::DeviceFocus mDeviceFocus;
    Riot::uint32 mFlags;
    HudManager::MinionHealthBarState mMinionHealthBarState;
    bool mHudManagerInitialized;
    bool mDrawLifeBars;
    bool mDrawLossOfControlBars;
    bool mWaitingForDeviceRegainingFocus;*/
}

impl RiotHudManager {
    pub fn camera_logic_mut(&mut self) -> Option<&'static mut RiotCameraLogic> {
        unsafe { self.camera_logic.as_mut::<'static>() }
    }
}
