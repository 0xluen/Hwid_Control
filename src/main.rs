#[cfg(target_os = "macos")]
fn get_hwid() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;

    let command = Command::new("ioreg")
        .arg("-rd1")
        .arg("-c")
        .arg("IOPlatformExpertDevice")
        .output()?;
    let output = String::from_utf8_lossy(&command.stdout);
    let hwid = output
        .lines()
        .find(|line| line.contains("IOPlatformUUID"))
        .map(|line| line.split('"').nth(3).unwrap())
        .ok_or("HWID not found")?;

    Ok(hwid.to_string())
}

#[cfg(target_os = "windows")]
fn get_hwid() -> Result<String, Box<dyn std::error::Error>> {
    use winreg::enums::*;
    use winreg::RegKey;

    let key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags("SOFTWARE\\Microsoft\\Cryptography", KEY_READ)?;
    let hwid: String = key.get_value("MachineGuid")?;

    Ok(hwid)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let os = std::env::consts::OS;

    match os {
        "macos" => {
            let hwid = get_hwid()?;
            println!("Mac HWID: {}", hwid);
        }
        "windows" => {
            let hwid = get_hwid()?;
            println!("Windows HWID: {}", hwid);
        }
        _ => {
            println!("Unsupported OS");
        }
    }

    Ok(())
}