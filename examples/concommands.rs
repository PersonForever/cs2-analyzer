use cs2_analyzer::{Analyzer, AnalyzerOptions, Result};

use walkdir::WalkDir;

fn main() -> Result<()> {
    let install_path = find_cs2_install_path()?;

    let dll_paths: Vec<_> = WalkDir::new(&install_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".dll"))
        .map(|e| e.path().to_path_buf())
        .collect();

    let mut analyzer = Analyzer::new_with_opts(AnalyzerOptions {
        buttons: false,
        concommands: true,
        convars: false,
        interfaces: false,
        offsets: false,
        schemas: false,
    });

    analyzer.add_files(&dll_paths);

    // Analyze all the files (This may take a while).
    let result = analyzer.analyze();

    for (file_name, result) in result {
        for concommand in &result.concommands {
            println!("[{}] {:#?}", file_name, concommand);
        }
    }

    Ok(())
}

#[cfg(target_family = "windows")]
fn find_cs2_install_path() -> Result<String> {
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let cs2 = hklm.open_subkey(r"SOFTWARE\WOW6432Node\Valve\cs2")?;

    let install_path: String = cs2.get_value("installpath")?;

    Ok(install_path)
}

#[cfg(not(target_family = "windows"))]
fn find_cs2_install_path() -> Result<String> {
    unimplemented!("auto-detecting cs2 install path is only supported on windows")
}
