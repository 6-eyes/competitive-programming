use std::io::Write;

fn main() {
    // 0. set instant
    let instant = std::time::Instant::now();

    // 1. fetch `rustc` path
    let rustc = {
        let Ok(o) = std::process::Command::new("rustup").args([ "which", "rustc" ]).stdin(std::process::Stdio::null()).stderr(std::process::Stdio::null()).output().inspect_err(|e| eprintln!("unable to execute command: {e}")) else { return };
        if o.status.success() {
            let out = String::from_utf8_lossy(&o.stdout);
            out.trim().to_string()
        }
        else {
            eprintln!("failed to execute 'rustup' command");
            return;
        }
    };

    const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
    let manifest_dir = std::path::Path::new(MANIFEST_DIR);
    println!("using manifest dir: {}", manifest_dir.display());

    // 2. fetch template path
    let Ok(template) = manifest_dir.join("template.rs").canonicalize().inspect_err(|e| println!("unable to canonicalize path: {e}")) else { return };
    println!("using template '{}'", template.display());

    // 3. create temp dir
    let Ok(dir) = TempDirGuard::new_dir().inspect_err(|e| eprintln!("failed creating temp directory: {e}")) else { return };
    println!("created temp directory '{}'", dir.display());

    // 4. test template
    let exec = dir.join("template_test");
    // 4.1. rustc --test atcoder/template.rs -o mytests
    let Ok(o) = std::process::Command::new(rustc).arg("--test").arg(&template).arg("-o").arg(&exec).stdin(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status().inspect_err(|e| eprintln!("unable to execute rustc command: {e}")) else { return };
    if o.success() {
        println!("created test executable '{}'", exec.display());
    }
    else {
        if let Some(c) = o.code() {
            eprintln!("'rustc' command failed with code {c}");
        }
        else {
            eprintln!("'rustc' command failed.");
        }

        return;
    }

    // 4.2. ./mytests --skip test_1
    let Ok(o) = std::process::Command::new(exec).args([ "--skip", "test_1" ]).stdin(std::process::Stdio::null()).stderr(std::process::Stdio::null()).output().inspect_err(|e| eprintln!("unable to run test executable: {e}")) else { return };
    if o.status.success() {
        println!("template passed");
    }
    else {
        // print output
        let o = String::from_utf8_lossy(&o.stdout);
        eprintln!("template test execution failed. output:\n{o}");
    }

    // 5. fetch next abc number
    let Ok(ac_dir) = std::fs::read_dir(manifest_dir).inspect_err(|e| eprintln!("unable to read atcoder directory '': {e}")) else { return };
    let latest = ac_dir.filter_map(Result::ok).filter(|d| d.file_type().is_ok_and(|f| f.is_dir())).filter_map(|d| d.file_name().to_string_lossy().strip_prefix("abc").and_then(|s| s.parse::<u32>().ok())).max().unwrap_or_default();
    println!("found latest contest: {latest}");

    // 6. create next contest
    let next = manifest_dir.join(format!("abc{}", latest + 1));
    if let Err(e) = std::fs::create_dir_all(&next) {
        eprintln!("unable to create directory '{}': {e}", next.display());
    }
    else {
        println!("created directory '{}'", next.display());
    }

    // 7. copy template
    if ('a'..='e').any(|c| std::fs::copy(&template, next.join(format!("{c}.rs"))).inspect(|_| println!("copied template to file '{c}.rs'")).is_err()) {
        eprintln!("failed to copy template");
        return;
    }

    // 8. update Cargo.toml
    let config = manifest_dir.join("Cargo.toml");

    // 8.1. create buffer
    // [[bin]]
    // name = "abc476_e"
    // path = "abc476/e.rs"
    let mut buf = String::new();
    ('a'..='e').for_each(|c| {
        let part = format!("\n[[bin]]\nname = \"abc{n}_{c}\"\npath = \"abc{n}/{c}.rs\"\n", n = latest + 1);
        buf.push_str(&part);
    });

    // 8.2. open file
    let Ok(mut f) = std::fs::OpenOptions::new().append(true).write(true).create(false).open(&config).inspect_err(|e| eprintln!("unable to open config file '{}': {e}", config.display())) else { return };
    // 8.3. write to file
    if let Err(e) = f.write_all(&buf.into_bytes()) {
        eprintln!("error updating '{}': {}", config.display(), e);
        return;
    }

    println!("updated config '{}'", config.display());
    println!("contest 'abc{}' created in took {}ms.", latest + 1, instant.elapsed().as_millis());
}

#[derive(Debug)]
#[repr(transparent)]
struct TempDirGuard(std::path::PathBuf);

impl TempDirGuard {
    fn new_dir() -> std::io::Result<Self> {
        let base = std::env::temp_dir();
        let pid = std::process::id();

        for attempt in 0..100 {
            let Ok(nanos) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|v| v.subsec_nanos()) else {
                return Err(std::io::Error::other("unable to fetch system time"));
            };

            let path = base.join(format!("atcoder-{pid}-{nanos}-{attempt}"));
            match std::fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e),
            }
        }

        Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "failed to create a unique temp dir"))
   }
}

impl std::ops::Deref for TempDirGuard {
    type Target = std::path::Path;

    fn deref(&self) -> &Self::Target {
        self.0.as_path()
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        match std::fs::remove_dir_all(&self.0) {
            Ok(()) => println!("cleaned up temp dir"),
            Err(e) => println!("failed to perform cleanup: {e}"),
        }
    }
}
