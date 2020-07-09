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
