use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

const TEST_FILE_PATH: &str = "tests/cat.png";

#[test]
fn test_encode() -> Result<(), Box<dyn std::error::Error>> {
  let message = "This is a secret message";
  let chunk_type = "teSt";
  let password = "test_password_123456789";
  std::env::set_var("TEST_PASSWORD", password);
  let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

  cmd
    .args(&["encode", TEST_FILE_PATH, chunk_type, message])
    .assert()
    .success();

  Ok(())
}

#[test]
fn test_decode() -> Result<(), Box<dyn std::error::Error>> {
  let chunk_type = "teSt";
  let password = "test_password_123456789";
  std::env::set_var("TEST_PASSWORD", password);
  let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

  cmd
    .args(&["decode", TEST_FILE_PATH, chunk_type])
    .assert()
    .success()
    .stdout("This is a secret message\n");

  Ok(())
}

#[test]
fn test_print() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

  cmd
    .args(&["print", TEST_FILE_PATH])
    .assert()
    .success()
    .stdout(predicate::str::contains("teSt"));

  Ok(())
}

#[test]
fn test_remove() -> Result<(), Box<dyn std::error::Error>> {
  let chunk_type = "teSt";
  let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

  cmd
    .args(&["remove", TEST_FILE_PATH, chunk_type])
    .assert()
    .success();

  Ok(())
}
