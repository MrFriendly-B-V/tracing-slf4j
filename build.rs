use color_eyre::eyre::Error;
use color_eyre::Result;
use std::env::var;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{fs, io};
use cfg_if::cfg_if;

fn main() -> Result<()> {
    build_java()
}

pub fn build_java() -> Result<()> {
    println!("cargo:rerun-if-changed=java");

    let manifest_dir = PathBuf::from(var("CARGO_MANIFEST_DIR")?);
    let outdir = PathBuf::from(var("OUT_DIR")?);
    let java_outdir = outdir.join("java");

    copy_dir_all(manifest_dir.join("java"), &java_outdir)?;

    run_gradle_command("shadowjar", &java_outdir)?;

    let builddir = java_outdir.join("build").join("libs");

    let outjar = fs::read_dir(builddir)?
        .into_iter()
        .find(|entry| {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => return false,
            };
            let fname = entry.file_name();
            let fname = fname.to_string_lossy();

            fname.starts_with("tracing-slf4j") && fname.ends_with("all.jar")
        })
        .and_then(|x| x.ok())
        .ok_or(Error::msg("Could not find output jar"))?
        .path();

    fs::copy(outjar, outdir.join("dependencies.jar"))?;

    run_gradle_command("clean", &java_outdir)?;

    Ok(())
}

fn gradle_command_name() -> &'static str {
    cfg_if! {
        if #[cfg(unix)] {
            "./gradlew"
        } else if #[cfg(windows)] {
            "gradlew.bat"
        } else {
            compiler_error!("Platform not supported");
        }
    }
}

fn run_gradle_command(cmd: &str, java_dir: &Path) -> Result<()> {
    let output = Command::new(gradle_command_name())
        .arg(cmd)
        .current_dir(java_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stdout = String::from_utf8(output.stdout)?;
        eprintln!("{stdout}");
        let stderr = String::from_utf8(output.stderr)?;
        eprintln!("{stderr}");

        Err(Error::msg("Building Java dependency failed"))
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
