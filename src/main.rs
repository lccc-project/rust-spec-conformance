#![feature(os_str_display)]
use std::ffi::OsString;
use std::fs::FileType;
use std::io::{self, Write};
use std::num::NonZero;
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};

use jobserver::FromEnvErrorKind;

use rand::Rng;

mod compiler;

enum Mode {
    RunPass,
    RunFail,
    CompileOnly,
    CompileFail,
}

impl Mode {
    fn name(&self) -> &'static str {
        match self {
            Self::RunPass => "run-pass",
            Self::RunFail => "run-fail",
            Self::CompileOnly => "compile-only",
            Self::CompileFail => "compile-fail",
        }
    }

    fn requires_success(&self) -> bool {
        matches!(self, Self::RunPass | Self::CompileOnly)
    }

    fn is_run(&self) -> bool {
        matches!(self, Mode::RunPass | Mode::RunFail)
    }

    fn check_result(&self, result: std::process::ExitStatus) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            if result.signal().is_some() {
                return false;
            }
        }

        match self {
            Self::RunFail | Self::CompileFail => !result.success(),
            Self::RunPass | Self::CompileOnly => result.success(),
        }
    }
}

fn main() -> ExitCode {
    let mut args = std::env::args();
    let prg_name = args.next().unwrap();

    match real_main(&prg_name, args) {
        Ok(c) => c,
        Err(e) => {
            println!("{prg_name}: {e}");
            ExitCode::from(101u8)
        }
    }
}

fn real_main(prg_name: &str, mut args: impl Iterator<Item = String>) -> io::Result<ExitCode> {
    let rustc = std::env::var_os("TEST_RUN_RUSTC")
        .or_else(|| std::env::var_os("RUSTC"))
        .unwrap_or_else(|| OsString::from("rustc"));

    let mut schema_file = None::<PathBuf>;

    // while let Some(arg) = args.next() {
    //     match &*arg {
    //         "--rustc-kind" => {
    //             // Override the kind of rustc used
    //         }
    //     }
    // }

    let client = match unsafe { jobserver::Client::from_env_ext(true) }.client {
        Ok(client) => client,
        Err(e) => match e.kind() {
            FromEnvErrorKind::NoEnvVar => jobserver::Client::new(
                std::thread::available_parallelism()
                    .map(NonZero::get)
                    .unwrap_or(1),
            )?,
            FromEnvErrorKind::NoJobserver => jobserver::Client::new(1)?,
            k => {
                let io_kind = match k {
                    FromEnvErrorKind::NoEnvVar | FromEnvErrorKind::NoJobserver => unreachable!(),

                    FromEnvErrorKind::CannotParse => io::ErrorKind::InvalidData,
                    FromEnvErrorKind::CannotOpenPath | FromEnvErrorKind::CannotOpenFd => {
                        io::ErrorKind::NotFound
                    }
                    FromEnvErrorKind::NegativeFd => io::ErrorKind::InvalidInput,
                    FromEnvErrorKind::Unsupported => io::ErrorKind::Unsupported,
                    _ => io::ErrorKind::Other,
                };

                return Err(io::Error::new(
                    io_kind,
                    format!("Cannot open jobserver {e}"),
                ));
            }
        },
    };

    let tmp_dir = tempdir::TempDir::new("conformance-suite-build")?;

    let mut gen = rand::thread_rng();

    let mut failed_tests = Vec::new();
    'walkdir: for (ctr, entry) in walkdir::WalkDir::new("suite").into_iter().enumerate() {
        let entry = entry?;

        if entry.file_type().is_file() {
            use std::io::BufRead;
            let path = entry.into_path();

            let mut end = OsString::from("out-");
            end.push(&format!("{ctr}-{:08x}-", gen.gen::<u32>()));
            end.push(
                path.components()
                    .last()
                    .expect("Must have at least one component")
                    .as_os_str(),
            );
            let mut end = PathBuf::from(end);
            end.set_extension("out");

            let temp_dir_path = tmp_dir.path().join(end);
            let file = io::BufReader::new(std::fs::File::open(&path)?);

            let mut mode = None::<Mode>;
            let mut edition = None;

            for lines in file.lines() {
                let line = lines?;
                if line.starts_with("#!") {
                    continue;
                }
                let line = line.trim();

                if line.is_empty() {
                    continue;
                } else if let Some(comment) = line.strip_prefix("//") {
                    if comment.starts_with("!@") {
                        continue; // This is a disabled directive
                    }

                    if let Some(directive) = comment.strip_prefix("@").map(str::trim_start) {
                        let (directive, param) = directive
                            .split_once(":")
                            .map_or((directive, ""), |(directive, param)| (directive, param));
                        match directive.trim() {
                            "skip" => continue 'walkdir,
                            "compile-fail" => {
                                if let Some(mode) = &mode {
                                    eprintln!("Warning: {}: Directive compile-fail overriding previous mode {}", path.display(), mode.name());
                                }
                                mode = Some(Mode::CompileFail)
                            }
                            "compile-only" => {
                                if let Some(mode) = &mode {
                                    eprintln!("Warning: {}: Directive compile-only overriding previous mode {}", path.display(), mode.name());
                                }
                                mode = Some(Mode::CompileOnly)
                            }
                            "run-fail" => {
                                if let Some(mode) = &mode {
                                    eprintln!("Warning: {}: Directive run-fail overriding previous mode {}", path.display(), mode.name());
                                }
                                mode = Some(Mode::RunFail)
                            }
                            "run-pass" => {
                                if let Some(mode) = &mode {
                                    eprintln!("Warning: {}: Directive run-pass overriding previous mode {}", path.display(), mode.name());
                                }
                                mode = Some(Mode::RunPass)
                            }
                            "reference" => {}
                            "edition" => {
                                let ed = param.trim();
                                if let Some(edition) = edition {
                                    eprintln!("Warning: {}: Directive edition: {} overriding previous edition {}", path.display(), ed, edition);
                                }
                                edition = Some(ed.parse::<u32>().map_err(|e| {
                                    io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        format!("{}: Edition malformed {ed}", path.display()),
                                    )
                                })?);
                            }
                            x if x.starts_with("edition=") => {
                                let ed = x.strip_prefix("edition=").unwrap().trim_start();
                                if let Some(edition) = edition {
                                    eprintln!("Warning: {}: Directive edition={} overriding previous edition {}", path.display(), ed, edition);
                                }
                                edition = Some(ed.parse::<u32>().map_err(|e| {
                                    io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        format!("{}: Edition malformed {ed}", path.display()),
                                    )
                                })?);
                            }
                            x => eprintln!(
                                "Warning: {}: Unrecognized or malformed directive {x}",
                                path.display()
                            ),
                        }
                    }
                } else {
                    break;
                }
            }
            let mode = if let Some(mode) = mode {
                mode
            } else {
                eprintln!(
                    "Warning: Test {} does not have a mode directive. Assuming compile-only",
                    path.display()
                );
                Mode::CompileOnly
            };

            let edition = edition.unwrap_or(2021);
            print!("Running test {} ({})... ", path.display(), mode.name());

            let success = if mode.is_run() {
                let status1 = Command::new(&rustc)
                    .arg("--test")
                    .args(["--crate-name", "__"])
                    .arg("--edition")
                    .arg(format!("{edition}"))
                    .arg("-o")
                    .arg(&temp_dir_path)
                    .arg(&path)
                    .env_remove("RUSTC_BOOTSTRAP")
                    .stderr(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stdin(Stdio::null())
                    .output()?;

                if !status1.status.success() {
                    eprintln!("Test {} (compiler) stderr: ", path.display());
                    let _ = std::io::stderr().write_all(&status1.stderr);
                    false
                } else {
                    let status2 = Command::new(&temp_dir_path)
                        .stderr(Stdio::piped())
                        .stdout(Stdio::null())
                        .stdin(Stdio::null())
                        .output()?;
                    let success = mode.check_result(status2.status);

                    if !success {
                        eprintln!("Test {} stderr:", path.display());
                        let _ = std::io::stderr().write_all(&status2.stderr);
                    }
                    success
                }
            } else {
                let output = Command::new(&rustc)
                    .args(["--crate-type", "rlib"])
                    .args(["--crate-name", "__"])
                    .arg("-o")
                    .arg(&temp_dir_path)
                    .arg(&path)
                    .env_remove("RUSTC_BOOTSTRAP")
                    .stderr(Stdio::null())
                    .stdout(Stdio::piped())
                    .stdin(Stdio::null())
                    .output()?;

                let res = mode.check_result(output.status);

                if !res && !output.status.success() {
                    eprintln!("Test {} stderr: ", path.display());
                    let _ = std::io::stderr().write_all(&output.stderr);
                }
                res
            };

            if !success {
                if mode.requires_success() {
                    println!("failed")
                } else {
                    println!("unexpected success");
                }
                failed_tests.push(path);
            } else {
                if mode.requires_success() {
                    println!("passed");
                } else {
                    println!("expected failure");
                }
            }
        }
    }

    if !failed_tests.is_empty() {
        println!(
            "{} tests failed with rustc={}",
            failed_tests.len(),
            rustc.display()
        );
        for test in failed_tests {
            println!("\t{}", test.display());
        }
        Ok(ExitCode::FAILURE)
    } else {
        println!("All tests passed");
        Ok(ExitCode::SUCCESS)
    }
}
