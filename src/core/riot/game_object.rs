
// TODO: GameObject
#[repr(C)]
pub struct RiotGameObject {
    /*struct __cppobj __attribute__((aligned(4))) GameObject : Riot::ComponentHostInterface, UObject
{
  gobjid_t ID;
  gobjid_t ownerID;
  bool NetworkNeedRegister;
  bool NetworkLocal;
  bool bZombie;
  bool bBecomeZombie;
  float mLastTookDamageTime;
  team_e TeamID;
  int ObjTypeFlags;
  int bNoStaticCollisions;
  std::__1::string Name;
  const Riot::PackageInterface *Package;
  int OcclusionID;
  int CollisionType;
  float SelectionHeight;
  float SelectionRadius;
  GameObject::move_s Move;
  GameObject **ClipObjs;
  int NumClipObjs;
  r3dPoint3D Position;
  r3dPoint3D Velocity;
  r3dPoint3D Acceleration;
  r3dPoint3D Orientation;
  int DrawOrder;
  bool bCleanUpOnKeyframe;
  __attribute__((packed)) __attribute__((aligned(1))) r3dBoundBox BBox;
  bool Active;
  uint8_t mUpdateNumber;
  int ObjFlags;
  std::__1::string MultiByteName;
  Riot::SceneGraph::Object *m_pSceneGraphObject;
  uint32_t mSubmeshVisibilityMask;
  uint32_t mSubmeshShadowVisibilityMask;
  DWORD m_DrawStates;
  ObjectManager *m_pObjectManager;
  r3dPoint3D m_PreviousPosition;
  ProxyGamePtr_Shared mSharedPtr;
  Net::NET_ID mNetworkID;
  std::__1::map<EGameObjectInterfaces,void *,std::__1::less<EGameObjectInterfaces>,std::__1::allocator<std::__1::pair<const EGameObjectInterfaces,void *> > > mInterfaces;
  Riot::ComponentHost mComponentHost;
  unsigned int mLifeCount;
  bool mDead;
};*/

    data: [u8; 0x104]
}