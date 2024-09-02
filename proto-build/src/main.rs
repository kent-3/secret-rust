//! Build Secret Network proto files. This build script clones the SecretNetwork version
//! specified in the SECRET_NETWORK_REV constant and then uses that to build the required
//! proto files for further compilation. This is based on the proto-compiler code
//! in github.com/informalsystems/ibc-rs

#![allow(unused)]

use regex::Regex;
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process,
    sync::atomic::{self, AtomicBool},
};
use walkdir::WalkDir;

/// Suppress log messages
static QUIET: AtomicBool = AtomicBool::new(false);

/// The secret network commit or tag to be cloned and used to build the proto files
const SECRET_NETWORK_REV: &str = "v1.13.3";

const SECRET_REPO: &str = "https://github.com/scrtlabs/SecretNetwork.git";

/// The directory generated cosmos-sdk proto files go into in this repo
const SECRET_SDK_PROTO_DIR: &str = "../secret-sdk-proto/src/prost/";
/// A temporary directory to clone SecretNetwork repo into
const SECRET_DIR: &str = "SecretNetwork";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google", "tendermint"];

/// Log info to the console (if `QUIET` is disabled)
macro_rules! info {
    ($msg:expr) => {
        if !is_quiet() {
            println!("[info] {}", $msg)
        }
    };
    ($fmt:expr, $($arg:tt)+) => {
        info!(&format!($fmt, $($arg)+))
    };
}

fn main() {
    if is_github() {
        set_quiet();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = SECRET_SDK_PROTO_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    let temp_secret_dir = tmp_build_dir.join("secret");

    let secret_repo_dir: PathBuf = SECRET_DIR.parse().unwrap();
    if secret_repo_dir.exists() {
        fs::remove_dir_all(secret_repo_dir.clone()).unwrap();
    }

    fs::create_dir_all(&temp_secret_dir).unwrap();

    clone_secret(SECRET_NETWORK_REV);
    output_secret_version(&temp_secret_dir);
    compile_secret_proto_and_services(&temp_secret_dir);
    copy_generated_files(&temp_secret_dir, &proto_dir.join("secret"));

    info!("Running rustfmt on prost/tonic-generated code");
    run_rustfmt(&proto_dir);

    fs::remove_dir_all(secret_repo_dir.clone()).unwrap();

    if is_github() {
        println!(
            "Rebuild protos with proto-build (secret rev: {})",
            SECRET_NETWORK_REV
        );
    }
}

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn set_quiet() {
    QUIET.store(true, atomic::Ordering::Relaxed);
}

/// Parse `--github` flag passed to `proto-build` on the eponymous GitHub Actions job.
/// Disables `info`-level log messages, instead outputting only a commit message.
fn is_github() -> bool {
    env::args().any(|arg| arg == "--github")
}

fn run_cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new(&cmd)
        .args(args)
        .stdout(stdout)
        .status()
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => panic!(
                "error running '{:?}': command not found. Is it installed?",
                cmd.as_ref()
            ),
            _ => panic!("error running '{:?}': {:?}", cmd.as_ref(), e),
        });

    if !exit_status.success() {
        match exit_status.code() {
            Some(code) => panic!("{:?} exited with error code: {:?}", cmd.as_ref(), code),
            None => panic!("{:?} exited without error code", cmd.as_ref()),
        }
    }
}

fn run_buf(config: &str, proto_path: impl AsRef<Path>, out_dir: impl AsRef<Path>) {
    run_cmd(
        "buf",
        [
            "generate",
            "--template",
            config,
            "--include-imports",
            "-o",
            &out_dir.as_ref().display().to_string(),
            &proto_path.as_ref().display().to_string(),
        ],
    );
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    run_cmd("git", args)
}

fn run_rustfmt(dir: &Path) {
    let mut args = ["--edition", "2021"]
        .iter()
        .map(Into::into)
        .collect::<Vec<OsString>>();

    args.extend(
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension() == Some(OsStr::new("rs")))
            .map(|e| e.into_path())
            .map(Into::into),
    );

    run_cmd("rustfmt", args);
}

fn clone_secret(revision: &str) {
    info!("Cloning Secret Network repo...");
    run_git([
        "clone",
        "--depth=1",
        "--filter=blob:none",
        "--sparse",
        "--branch",
        revision,
        SECRET_REPO,
    ]);
    run_git([
        "-C",
        SECRET_DIR,
        "sparse-checkout",
        "set",
        "proto",
        "third_party/proto",
    ]);
}

fn output_secret_version(out_dir: &Path) {
    let path = out_dir.join("SECRET_COMMIT");
    fs::write(path, SECRET_NETWORK_REV).unwrap();
}

fn compile_secret_proto_and_services(out_dir: &Path) {
    info!(
        "Compiling secret-network .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let secret_dir = Path::new(SECRET_DIR);

    let proto_includes_paths = [
        // format!("{}/../proto", root),
        format!("{}/proto", secret_dir.display()),
        format!("{}/third_party/proto", secret_dir.display()),
    ];

    // Paths
    let proto_paths = [
        format!("{}/proto/secret/compute", secret_dir.display()),
        format!("{}/proto/secret/emergencybutton", secret_dir.display()),
        format!("{}/proto/secret/intertx", secret_dir.display()),
        format!("{}/proto/secret/registration", secret_dir.display()),
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Enable generation of `prost::Name` annotations for all types
    let mut config = prost_build::Config::new();
    config.enable_type_names();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    // Compile all of the proto files, along with grpc service clients
    info!("Compiling proto definitions and clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(out_dir)
        .extern_path(".cosmos", "::cosmos_sdk_proto::cosmos")
        .extern_path(".ibc", "::cosmos_sdk_proto::ibc")
        .extern_path(".tendermint", "::tendermint_proto")
        .compile_with_config(config, &protos, &includes)
        .unwrap();
    info!("=> Done!");

    // TODO - Use `buf` to have separate files for the tonic clients
    // let proto_path = Path::new(SECRET_DIR).join("proto");
    // run_buf("buf.secret.gen.yaml", proto_path, out_dir);
}

/// collect_protos walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(to_dir).unwrap_or_default();
    create_dir_all(to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    /// Regex substitutions to apply to the prost-generated output
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Use `tendermint-proto` proto definitions
        ("(super::)+tendermint", "tendermint_proto"),
        // Feature-gate gRPC client modules
        (
            "/// Generated client implementations.",
            "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]",
        ),
        // Feature-gate gRPC impls which use `tonic::transport`
        (
            "impl(.+)tonic::transport(.+)",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             impl${1}tonic::transport${2}",
        ),
        // Feature-gate gRPC server modules
        (
            "/// Generated server implementations.",
            "/// Generated server implementations.\n\
             #[cfg(feature = \"grpc\")]",
        ),
    ];

    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let mut contents = fs::read_to_string(src)?;

    for &(regex, replacement) in REPLACEMENTS {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    fs::write(dest, &contents)
}

fn patch_file(path: impl AsRef<Path>, pattern: &Regex, replacement: &str) -> io::Result<()> {
    let mut contents = fs::read_to_string(&path)?;
    contents = pattern.replace_all(&contents, replacement).to_string();
    fs::write(path, &contents)
}

/// Fix clashing type names in prost-generated code. See cosmos/cosmos-rust#154.
fn apply_patches(proto_dir: &Path) {
    for (pattern, replacement) in [
        ("enum Validators", "enum Policy"),
        (
            "stake_authorization::Validators",
            "stake_authorization::Policy",
        ),
    ] {
        patch_file(
            &proto_dir.join("cosmos-sdk/cosmos.staking.v1beta1.rs"),
            &Regex::new(pattern).unwrap(),
            replacement,
        )
        .expect("error patching cosmos.staking.v1beta1.rs");
    }
}
