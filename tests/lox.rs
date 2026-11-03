use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use libtest_mimic::{Arguments, Failed, Trial};

#[derive(Default)]
struct Expect {
    stdout: Vec<String>,
    runtime_error: Option<String>,
    compile_errors: Vec<String>,
}

fn parse_expectations(source: &str) -> Expect {
    let mut e = Expect::default();
    for (i, line) in source.lines().enumerate() {
        let lineno = i + 1;
        if let Some(p) = line.find("/// expect: ") {
            e.stdout.push(line[p + "/// expect: ".len()..].to_string());
        } else if let Some(p) = line.find("/// expect runtime error: ") {
            e.runtime_error = Some(line[p + "/// expect runtime error: ".len()..].to_string())
        } else if let Some(p) = line.find("/// Error") {
            e.compile_errors
                .push(format!("[line {lineno}] {}", &line[p + "/// ".len()..]));
        }
    }
    e
}

fn run_test(path: &Path) -> Result<(), Failed> {
    let source = fs::read_to_string(path)?;
    let expect = parse_expectations(&source);

    let output = Command::new(env!("CARGO_BIN_EXE_lox")).arg(path).output()?;
    let stdout: Vec<_> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(String::from)
        .collect();
    let stderr: Vec<_> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(String::from)
        .collect();
    let code = output.status.code();

    if !expect.compile_errors.is_empty() {
        // compile errors exit with 65
        if code != Some(65) {
            return Err(format!("Expected exit 65, got {code:?}").into());
        }
        if stderr != expect.compile_errors {
            return Err(format!(
                "compile errors differ\nexpected: {:#?}\n   got: {:#?}",
                expect.compile_errors, stderr
            )
            .into());
        }
        return Ok(());
    }

    if let Some(msg) = &expect.runtime_error {
        // runtime errors exit with 70
        if code != Some(70) {
            return Err(format!("expected exit 70, got {code:?}").into());
        }
        if stderr.first() != Some(msg) {
            return Err(format!("expected error {msg:?}, got {:?}", stderr.first()).into());
        }

        if stdout != expect.stdout {
            return Err(format!(
                "stdout differs\nexpected: {:#?}\n  got: {:#?}",
                expect.stdout, stdout
            )
            .into());
        }
    }
    Ok(())
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let p = entry.unwrap().path();
        if p.is_dir() {
            collect(&p, out);
        } else if p.extension().is_some_and(|e| e == "lox") {
            out.push(p);
        }
    }
}

fn main() {
    let args = Arguments::from_args();
    let mut files = Vec::new();
    collect(Path::new("tests/lox"), &mut files);
    files.sort();

    let trials = files
        .into_iter()
        .map(|path| {
            let name = path
                .strip_prefix("tests/lox")
                .unwrap()
                .with_extension("")
                .to_string_lossy()
                .into_owned();
            Trial::test(name, move || run_test(&path))
        })
        .collect();

    libtest_mimic::run(&args, trials).exit();
}
