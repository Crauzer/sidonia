use clipboard_win::{get_clipboard_string, set_clipboard_string};
use crossbeam_utils::atomic::AtomicCell;
use imgui::{ClipboardBackend, Context, ImStr, ImString, Key};
use log::info;
use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, WPARAM},
        windef::HWND,
        windowsx::{GET_X_LPARAM, GET_Y_LPARAM},
    },
    um::winuser::{
        CallWindowProcW, DefWindowProcW, GetKeyState, SetWindowLongA, GET_WHEEL_DELTA_WPARAM, GET_XBUTTON_WPARAM, GWL_WNDPROC, VK_BACK,
        VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_HOME, VK_LEFT, VK_MENU, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT, VK_SHIFT,
        VK_SPACE, VK_TAB, VK_UP, WHEEL_DELTA, WM_CHAR, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_LBUTTONUP,
        WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEHWHEEL, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN,
        WM_RBUTTONUP, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_XBUTTONDBLCLK, WM_XBUTTONDOWN, WM_XBUTTONUP, WNDPROC, XBUTTON1, XBUTTON2,
    },
};

static ORIGINAL_WNDPROC: AtomicCell<Option<WNDPROC>> = AtomicCell::new(None);

pub struct InputManager {
    hwnd: HWND,
}

impl Drop for InputManager {
    fn drop(&mut self) {
        if let Some(wndproc) = ORIGINAL_WNDPROC.swap(None) {
            log::info!("Dropping InputManager.");
            unsafe { set_wndproc(wndproc, self.hwnd) };
        }
    }
}

impl InputManager {
    #[inline]
    pub unsafe fn new(imgui: &mut Context, hwnd: HWND) -> Result<InputManager, &'static str> {
        if ORIGINAL_WNDPROC.load().is_none() {
            ORIGINAL_WNDPROC.store(Some(core::mem::transmute::<_, winapi::um::winuser::WNDPROC>(set_wndproc(
                Some(wndproc),
                hwnd,
            ))));

            configure(imgui);
            Ok(InputManager { hwnd })
        } else {
            Err("WndProc is already hooked once, Drop that hook before trying to hook WndProc again.")
        }
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let io = &mut *imgui::sys::igGetIO();

    let mut mouse_event: bool = true;
    let mut keyboard_event: bool = true;

    match msg {
        WM_LBUTTONDOWN | WM_LBUTTONDBLCLK => io.MouseDown[0] = true,
        WM_RBUTTONDOWN | WM_RBUTTONDBLCLK => io.MouseDown[1] = true,
        WM_MBUTTONDOWN | WM_MBUTTONDBLCLK => io.MouseDown[2] = true,
        WM_XBUTTONDOWN | WM_XBUTTONDBLCLK => match GET_XBUTTON_WPARAM(w_param) {
            XBUTTON1 => io.MouseDown[3] = true,
            XBUTTON2 => io.MouseDown[4] = true,
            _ => {}
        },
        WM_LBUTTONUP => io.MouseDown[0] = false,
        WM_RBUTTONUP => io.MouseDown[1] = false,
        WM_MBUTTONUP => io.MouseDown[2] = false,
        WM_XBUTTONUP => match GET_XBUTTON_WPARAM(w_param) {
            XBUTTON1 => io.MouseDown[3] = false,
            XBUTTON2 => io.MouseDown[4] = false,
            _ => {}
        },
        WM_MOUSEWHEEL => io.MouseWheel = f32::from(GET_WHEEL_DELTA_WPARAM(w_param)) / f32::from(WHEEL_DELTA),
        WM_MOUSEHWHEEL => io.MouseWheelH = f32::from(GET_WHEEL_DELTA_WPARAM(w_param)) / f32::from(WHEEL_DELTA),
        WM_MOUSEMOVE => {
            io.MousePos = (GET_X_LPARAM(l_param) as f32, GET_Y_LPARAM(l_param) as f32).into();
        }
        _ => mouse_event = false,
    }

    match msg {
        WM_KEYDOWN | WM_SYSKEYDOWN if w_param < 256 => io.KeysDown[w_param] = true,
        WM_KEYUP | WM_SYSKEYUP if w_param < 256 => io.KeysDown[w_param] = false,
        WM_CHAR => imgui::sys::ImGuiIO_AddInputCharacter(io, w_param as _),
        _ => keyboard_event = false,
    }

    if keyboard_event {
        io.KeyCtrl = (GetKeyState(VK_CONTROL) as u16 & 0x8000) != 0;
        io.KeyShift = (GetKeyState(VK_SHIFT) as u16 & 0x8000) != 0;
        io.KeyAlt = (GetKeyState(VK_MENU) as u16 & 0x8000) != 0;
        io.KeySuper = false;
    }

    if (io.WantCaptureMouse && mouse_event) || (io.WantCaptureKeyboard && keyboard_event) {
        true as LRESULT
    } else {
        match ORIGINAL_WNDPROC.load() {
            Some(x) => CallWindowProcW(x, hwnd, msg, w_param, l_param),
            None => DefWindowProcW(hwnd, msg, w_param, l_param),
        }
    }
}

#[inline]
unsafe fn set_wndproc(wnd_proc: WNDPROC, hwnd: HWND) -> i32 {
    let wnd_proc = core::mem::transmute::<_, i32>(wnd_proc.expect("WNDPROC was None."));
    SetWindowLongA(hwnd, GWL_WNDPROC, wnd_proc)
}

fn configure(imgui: &mut Context) {
    let io = imgui.io_mut();

    io.key_map[Key::Tab as usize] = VK_TAB as _;
    io.key_map[Key::LeftArrow as usize] = VK_LEFT as _;
    io.key_map[Key::RightArrow as usize] = VK_RIGHT as _;
    io.key_map[Key::UpArrow as usize] = VK_UP as _;
    io.key_map[Key::DownArrow as usize] = VK_DOWN as _;
    io.key_map[Key::PageUp as usize] = VK_PRIOR as _;
    io.key_map[Key::PageDown as usize] = VK_NEXT as _;
    io.key_map[Key::Home as usize] = VK_HOME as _;
    io.key_map[Key::End as usize] = VK_END as _;
    io.key_map[Key::Delete as usize] = VK_DELETE as _;
    io.key_map[Key::Backspace as usize] = VK_BACK as _;
    io.key_map[Key::Enter as usize] = VK_RETURN as _;
    io.key_map[Key::Escape as usize] = VK_ESCAPE as _;
    io.key_map[Key::Space as usize] = VK_SPACE as _;
    io.key_map[Key::A as usize] = u32::from(b'A');
    io.key_map[Key::C as usize] = u32::from(b'C');
    io.key_map[Key::V as usize] = u32::from(b'V');
    io.key_map[Key::X as usize] = u32::from(b'X');
    io.key_map[Key::Y as usize] = u32::from(b'Y');
    io.key_map[Key::Z as usize] = u32::from(b'Z');

    imgui.set_clipboard_backend(Box::new(ClipboardSupport));
}

struct ClipboardSupport;

impl ClipboardBackend for ClipboardSupport {
    #[inline]
    fn get(&mut self) -> Option<ImString> {
        get_clipboard_string().map(Into::into).ok()
    }

    #[inline]
    fn set(&mut self, text: &ImStr) {
        let _ = set_clipboard_string(text.to_str());
    }
}
