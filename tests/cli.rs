use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn prints_tree_without_default_noise() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("main.py"), "").unwrap();
    fs::create_dir(dir.path().join("__pycache__")).unwrap();
    fs::write(
        dir.path().join("__pycache__").join("main.cpython-312.pyc"),
        "",
    )
    .unwrap();

    Command::cargo_bin("pytree")
        .unwrap()
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("main.py"))
        .stdout(predicate::str::contains("0 directories, 1 file"))
        .stdout(predicate::str::contains("__pycache__").not());
}

#[test]
fn uses_unicode_tree_connectors_by_default() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("main.py"), "").unwrap();

    Command::cargo_bin("pytree")
        .unwrap()
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("└── main.py"))
        .stdout(predicate::str::contains("0 directories, 1 file"))
        .stdout(predicate::str::contains("`-- main.py").not());
}

#[test]
fn supports_ascii_tree_connectors() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("main.py"), "").unwrap();

    Command::cargo_bin("pytree")
        .unwrap()
        .arg("--ascii")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("`-- main.py"))
        .stdout(predicate::str::contains("0 directories, 1 file"));
}

#[test]
fn all_shows_default_noise() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("__pycache__")).unwrap();
    fs::write(dir.path().join(".env.example"), "").unwrap();

    Command::cargo_bin("pytree")
        .unwrap()
        .arg("--all")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("__pycache__"))
        .stdout(predicate::str::contains(".env.example"));
}

#[test]
fn hides_dotfiles_by_default() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join(".env.example"), "").unwrap();
    fs::write(dir.path().join("main.py"), "").unwrap();

    Command::cargo_bin("pytree")
        .unwrap()
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("main.py"))
        .stdout(predicate::str::contains(".env.example").not());
}

#[test]
fn respects_gitignore_unless_disabled() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join(".gitignore"), "ignored.py\n").unwrap();
    fs::write(dir.path().join("ignored.py"), "").unwrap();

    Command::cargo_bin("pytree")
        .unwrap()
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("ignored.py").not());

    Command::cargo_bin("pytree")
        .unwrap()
        .arg("--no-gitignore")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("ignored.py"));
}

#[test]
fn dirs_only_skips_files() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("pkg")).unwrap();
    fs::write(dir.path().join("main.py"), "").unwrap();

    Command::cargo_bin("pytree")
        .unwrap()
        .arg("--dirs-only")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("pkg"))
        .stdout(predicate::str::contains("main.py").not());
}

#[test]
fn limits_depth() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("pkg").join("inner")).unwrap();
    fs::write(dir.path().join("pkg").join("inner").join("module.py"), "").unwrap();

    Command::cargo_bin("pytree")
        .unwrap()
        .arg("--depth")
        .arg("1")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("pkg"))
        .stdout(predicate::str::contains("inner").not())
        .stdout(predicate::str::contains("module.py").not());
}

#[test]
fn renders_json() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("main.py"), "").unwrap();

    Command::cargo_bin("pytree")
        .unwrap()
        .arg("--format")
        .arg("json")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"main.py\""))
        .stdout(predicate::str::contains("\"kind\": \"file\""));
}

#[test]
fn applies_custom_ignore_pattern() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("keep.py"), "").unwrap();
    fs::write(dir.path().join("drop.py"), "").unwrap();

    Command::cargo_bin("pytree")
        .unwrap()
        .arg("--ignore")
        .arg("drop.py")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("keep.py"))
        .stdout(predicate::str::contains("drop.py").not());
}
