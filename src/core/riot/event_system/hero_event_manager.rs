use crate::core::{
    msvc::smart_pointers::StdSharedPtr,
    riot::{ai_hero::RiotAiHero, event_system::game_event_network_dispatcher::RiotGameEventNetworkDispatcher, timer::RiotBasicTimer},
};

#[repr(C)]
pub struct RiotHeroEventManager {
    /*

      AIHero *mOwner;
    EventSystem::EventDispatcherPtr mEventDispatcher;
    Time::DispatcherLifetimeListSharedPtr<EventSystem::HeroEventManager,EventSystem::GameEventNetworkDispatcher> mEvents;
    int mMultiKillNumber;
    Riot::GameTimer mMultiKillTimer;
    int mMultiSpreeNumber;
    Riot::GameTimer mMultiSpreeTimer;
    int mSpreeKillNumber;
    float mSpreeDeathNumber;
    int mAssistKarma;
    int mSpreeAssistNumber;
    int mAssistsNumber;
    float mTimeDeathStart;
    float mTimeAliveStart;
    std::__1::set<Net::ID<Net::_NET_ID>,std::__1::less<Net::ID<Net::_NET_ID> >,std::__1::allocator<Net::ID<Net::_NET_ID> > > mHealedUnits;
    Net::NET_ID mLastHeroToAttackID;
    float mTimeLastHeroAttacked;
    float mLastTimeDamageTaken;
    float mMinionGoldFarmed;
    EventSystem::HeroEventManager::LastUnitsToAttackArray mLastUnitToAttack;
    HeroStatsCollection mHeroStatsCollection;

       */
    owner: *mut RiotAiHero,
    event_dispatcher: StdSharedPtr<RiotGameEventNetworkDispatcher>,
    events: [u8; 0x18], // Time::DispatcherLifetimeListSharedPtr<EventSystem::HeroEventManager,EventSystem::GameEventNetworkDispatcher>,
    multi_kill_number: i32,
    multi_kill_timer: RiotBasicTimer,
    multi_spree_numer: i32,
    multi_spree_timer: RiotBasicTimer,
    spree_kill_number: i32,
    spree_death_number: f32,
    assist_karma: i32,
    spree_assist_number: i32,
    assists_number: i32,
    time_death_start: f32,
    time_alive_start: f32,
    //healed_units: StdSet
}
