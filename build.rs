use std::process::Command;

fn main() {
  let commit = Command::new("git")
    .args(&["rev-parse", "HEAD"])
    .output()
    .unwrap();
  let branch = Command::new("git")
    .args(&["rev-parse", "--abbrev-ref", "HEAD"])
    .output()
    .unwrap();

  let git_hash = String::from_utf8(commit.stdout).unwrap();
  let git_branch = String::from_utf8(branch.stdout).unwrap();

  println!("cargo:rustc-env=GIT_HASH={}", git_hash);
  println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
}
