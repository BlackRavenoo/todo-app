use std::{io::Write, process::Stdio};



pub fn select(
    options: Vec<String>,
    args: Vec<String>,
) -> Result<Vec<String>, String> {
    let options = options.join("\n");

    let fzf = std::process::Command::new("fzf")
        .args(args)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn();

    let mut fzf = match fzf {
        Ok(fzf) => fzf,
        Err(e) => {
            return Err(format!("Failed to spawn fzf process: {:?}", e))
        }
    };

    fzf.stdin.take()
        .expect("Failed to open stdin")
        .write_all(options.as_bytes())
        .expect("Failed to write data to fzf stdin");

    let selected = std::str::from_utf8(
        &fzf.wait_with_output()
        .expect("Failed to read fzf stdout")
        .stdout
    )
    .expect("Failed to create str from byte array")
    .split("\n").map(|s| s.to_string()).collect::<Vec<_>>();

    Ok(selected[..selected.len() - 1].to_vec())
}