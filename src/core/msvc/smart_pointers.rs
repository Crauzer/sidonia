use winapi::shared::minwindef::LPVOID;

#[repr(C)]
pub struct StdSharedPtr<T> {
    ptr: *mut T,
    cntrl: *mut SharedWeakCount,
}

#[repr(C)]
pub struct StdWeakPtr<T> {
    ptr: *mut T,
    cntrl: *mut SharedWeakCount,
}

#[repr(C)]
struct SharedWeakCount {
    vtable: LPVOID,
    shared_owners: i32,
    shared_weak_owners: i32,
}

#[repr(C)]
pub struct StdUniquePtr<T> {
    ptr: *mut T,
}

// TODO: Implement unique_ptr fully
/*
struct std::__1::unique_ptr<Riot::BakedEnvironmentRenderer,std::__1::default_delete<Riot::BakedEnvironmentRenderer> >
{
  std::__1::__compressed_pair<Riot::BakedEnvironmentRenderer *,std::__1::default_delete<Riot::BakedEnvironmentRenderer> > __ptr_;
};
 */
