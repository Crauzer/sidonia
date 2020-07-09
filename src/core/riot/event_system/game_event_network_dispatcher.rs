use crate::core::{msvc::vector::StdVector, riot::net::RiotNetVisibilityObject};

#[repr(C)]
pub struct RiotGameEventNetworkDispatcher {
    dispatcher: Dispatcher,
    global_callback_list: StdVector<!>, // T: Loki::Functor<bool,Loki::Typelist<EventSystem::EventsNames,Loki::Typelist<const EventSystem::EventDetails &,Loki::Typelist<const EventSystem::BaseParams &,Loki::NullType> > >,::Loki::SingleThreaded> *,std::__1::allocator<const Loki::Functor<bool,Loki::Typelist<EventSystem::EventsNames,Loki::Typelist<const EventSystem::EventDetails &,Loki::Typelist<const EventSystem::BaseParams &,Loki::NullType> > >,::Loki::SingleThreaded> *>
    net_visibility: *mut RiotNetVisibilityObject,
}

#[repr(C)]
struct Dispatcher {
    data: [u8; 0x30],
}
