pub fn init_discord_ipc() -> Result<(), String> {
    let pipe_name = format!("\\?\\pipe\\discord-ipc-{}", get_current_process_id());

    match create_pipe_handle(&pipe_name) {
        Ok(handle) => {
            discord_ipc_handle = Some(handle);
            Ok(())
        },
        Err(_) => Err(format!("Failed to create Discord IPC pipe: {}", pipe_name)),
    }
}

pub fn send_frame(opcode: u32, command: &str) -> Result<(), String> {
    if let Some(handle) = discord_ipc_handle {
        // Implementation for sending a frame using the handle
        Ok(())
    } else {
        Err("Discord IPC pipe not initialized.".to_string())
    }
}

pub fn read_frame() -> Result<String, String> {
    if let Some(handle) = discord_ipc_handle {
        // Implementation for reading a frame using the handle
        Ok("".to_string())
    } else {
        Err("Discord IPC pipe not initialized.".to_string())
    }
}

use std::sync::Mutex;

lazy_static! {
    static ref DISCORD_IPC_HANDLE: Mutex<Option<u32>> = Mutex::new(None);
}
    let handle = create_pipe_handle(&pipe_name)?;
    {
        let mut ipc_handle = DISCORD_IPC_HANDLE.lock().unwrap();
        *ipc_handle = Some(handle);

            Ok(())
        },
        Err(_) => Err(format!("Failed to create Discord IPC pipe: {}", pipe_name)),
    


pub fn send_frame(opcode: u32, command: &str) -> Result<(), String> {
    if let Some(handle) = *DISCORD_IPC_HANDLE.lock().unwrap() {
        // Implementation for sending a frame using the handle
        Ok(())
    } else {
        Err("Discord IPC pipe not initialized.".to_string())
    }
}

pub fn read_frame() -> Result<String, String> {
    if let Some(handle) = discord_ipc_handle {
        // Implementation for reading a frame using the handle
        Ok("".to_string())
    } else {
        Err("Discord IPC pipe not initialized.".to_string())
    }
}

lazy_static! {
    static ref discord_ipc_handle: Option<u32> = None;
}

fn get_current_process_id() -> u32 {
    use std::process;
    process::id()
}

use std::io;
use std::os::windows::io::{AsRawHandle, FromRawHandle};

fn create_pipe_handle(pipe_name: &str) -> Result<u32, String> {
    use winapi::shared::minwindef::{DWORD, HANDLE};
    use winapi::um::winbase::{CreateNamedPipeW, CreateFileW, FILE_FLAG_OVERLAPPED, PIPE_ACCESS_DUPLEX, PIPE_TYPE_BYTE, PIPE_READMODE_BYTE, PIPE_WAIT, FILE_ATTRIBUTE_NORMAL, GENERIC_READ, GENERIC_WRITE};
    use winapi::um::winnt::{OVERLAPPED, INVALID_HANDLE_VALUE};

    let pipe_name_wide: Vec<u16> = pipe_name.encode_utf16().chain(Some(0)).collect();
    let pipe_handle: HANDLE = unsafe {
        use winapi::um::winnt::HANDLE;
        CreateNamedPipeW(
            pipe_name_wide.as_ptr(),
            PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
            1,
            65536,
            65536,
            300,
            None,
        )
    };

    if pipe_handle == INVALID_HANDLE_VALUE {
        return Err(format!("Failed to create pipe: {}", io::Error::last_os_error()));
    }

    let client_handle: HANDLE = unsafe {
        use winapi::um::winnt::HANDLE;
        CreateFileW(
            pipe_name_wide.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            0,
            None,
            FILE_ATTRIBUTE_NORMAL,
            FILE_FLAG_OVERLAPPED,
        )
    };

    if client_handle == INVALID_HANDLE_VALUE {
        return Err(format!("Failed to connect to pipe: {}", io::Error::last_os_error()));
    }

    let handle = client_handle.as_raw_handle() as u32;
    Ok(handle)
}
