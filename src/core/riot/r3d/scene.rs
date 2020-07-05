use winapi::shared::minwindef::LPVOID;

#[repr(C)]
#[derive(Debug)]
pub struct R3dSceneLayer {
    simple_skin_renderer: LPVOID,        // RiotSimpleSkinRenderer
    simple_environment_renderer: LPVOID, // RiotSimpleEnvironmentRenderer
    perf_test_result: LPVOID,            // Riot::Eternity::PerfTest::RESULT*
    dxut_dialog_manager: LPVOID,         // DXUTDialogManager*
    death_screen: bool,
}
