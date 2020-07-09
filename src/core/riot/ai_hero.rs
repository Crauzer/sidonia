use crate::core::{
    msvc::smart_pointers::StdWeakPtr,
    riot::{
        avatar::RiotAvatar, contextual_emote_component::RiotContextualEmoteComponent,
        event_system::hero_event_manager::RiotHeroEventManager, evolution::RiotEvolutionState, experience::RiotExperience,
        hero_vo_component::RiotHeroVoComponent, karma::RiotKarma, region::RiotRegion,
    },
};

#[repr(C)]
pub struct RiotAiHero {
    /*
      Karma mKarma;
    EvolutionState mEvolution;
    Experience mExperience;
    HeroVOComponent *mVOComponent;
    ContextualEmoteComponent *mContextualEmoteComponent;
    std::__1::weak_ptr<Riot::Region> mVisionRegion;
    Avatar mAvatar;
    EventSystem::HeroEventManager mEventManager;
    bool mbGreyscaleEnabledWhenDead;
    bool mbAutoRespawn;
    LoL::PlayerScore mPlayerScore;
    Replicate<int> mReplicatedNeutralKills;
    bool mShopEnabled;
    bool mShopForceEnabled;
    int mMaxLevelOverride;
      */
    karma: RiotKarma,
    evolution: RiotEvolutionState,
    experience: RiotExperience,
    vo_component: *mut RiotHeroVoComponent,
    contextual_emote_component: *mut RiotContextualEmoteComponent,
    vision_region: StdWeakPtr<RiotRegion>,
    avatar: RiotAvatar,
    //event_manager: RiotHeroEventManager
}
