use crate::core::riot::karma::RiotKarma;
use crate::core::riot::evolution::RiotEvolutionState;
use crate::core::riot::experience::RiotExperience;
use crate::core::riot::hero_vo_component::RiotHeroVoComponent;
use crate::core::riot::contextual_emote_component::RiotContextualEmoteComponent;
use crate::core::riot::region::RiotRegion;
use crate::core::msvc::smart_pointers::StdWeakPtr;
use crate::core::riot::avatar::RiotAvatar;
use crate::core::riot::event_system::hero_event_manager::RiotHeroEventManager;

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