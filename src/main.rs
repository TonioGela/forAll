#![allow(non_snake_case)]
use colored::Colorize;
use std::env::{args, current_dir};
use std::fs::{read_dir, DirEntry};
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use subprocess::{Exec, Popen, PopenError, Redirection};

fn main() -> std::io::Result<()> {
    let dirs: Vec<DirEntry> = cwd_child_dirs()?;
    let command: String = command_from_args().trim().to_owned();

    let mut processes: Vec<(Popen, PathBuf)> = dirs
        .into_iter()
        .map(|d| d.path())
        .map(|d| run_command_in_dir(&command, &d).map(|x| (x, d)))
        .collect::<Result<Vec<(Popen, PathBuf)>>>()?;

    let reports: Vec<String> = processes
        .iter_mut()
        .map(|(e, p)| wait_and_get_report(e, p))
        .collect::<Result<Vec<String>>>()?;

    let report: String = reports
        .into_iter()
        .fold(String::new(), |acc, s| format!("{}\n{}", acc, s));

    Ok(println!("{}", report.trim_matches('\n')))
}

fn wait_and_get_report(handle: &mut Popen, path: &PathBuf) -> Result<String> {
    let (std_out, std_err) = handle.communicate(None)?;

    let std_out = std_out.unwrap_or_default();
    let std_err = std_err.unwrap_or_default();

    let error = Error::new(ErrorKind::InvalidData, "Unable to extract filename");
    let path: String = path.canonicalize().and_then(|p| {
        p.file_name()
            .map(|x| x.to_string_lossy().into())
            .ok_or(error)
    })?;

    let error = |x: PopenError| Error::new(ErrorKind::BrokenPipe, x.to_string());

    let output: String = if handle.wait().map_err(error)?.success() {
        format!("{}:\n{}", path.green(), std_out)
    } else {
        format!("{}:\n{}\n{}", path.red(), std_out, std_err) //TODO! migliorare
    };

    Ok(output.replace("\n\n", "\n").to_owned())
}

fn run_command_in_dir(command: &str, path: &PathBuf) -> Result<Popen> {
    let error = |x: PopenError| Error::new(ErrorKind::BrokenPipe, x.to_string());
    Exec::shell(command)
        .cwd(path)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .detached()
        .popen()
        .map_err(error)
}

fn command_from_args() -> String {
    args().skip(1).collect::<Vec<String>>().join(" ")
}

fn cwd_child_dirs() -> std::io::Result<Vec<DirEntry>> {
    let cwd = current_dir()?;
    let dirs = read_dir(cwd.as_path())?;
    let dirs = dirs.collect::<std::io::Result<Vec<DirEntry>>>()?;
    let dirs = dirs
        .into_iter()
        .filter(|x| x.file_type().map_or(false, |x| x.is_dir()))
        .collect();
    Ok(dirs)
}
