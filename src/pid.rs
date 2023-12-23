use windows::Win32::System::Diagnostics::ToolHelp::*;

pub fn pid_list() -> windows::core::Result<Vec<u32>> {
    let mut pid_list: Vec<u32> = Vec::new();

    let snapshot = unsafe {
        CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
    }?;

    let mut init_process_entry32w = PROCESSENTRY32W {
        dwSize: core::mem::size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    unsafe {
        Process32FirstW(snapshot, &mut init_process_entry32w)
    }?;
    
    while unsafe {Process32NextW(snapshot, &mut init_process_entry32w)}.is_ok() {
        pid_list.push(init_process_entry32w.th32ProcessID)
    }

    Ok(pid_list)
}