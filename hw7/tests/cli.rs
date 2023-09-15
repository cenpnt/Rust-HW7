use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_sort_number() -> TestResult {
    let expected = "Ascending order: [1, 2, 3, 5, 7]\nDescending order: [7, 5, 3, 2, 1]\n";
    let mut cmd = Command::cargo_bin("1_1")?;
    cmd.arg("1").arg("5").arg("2").arg("7").arg("3").assert().success().stdout(expected);

    Ok(())
}   

#[test]
fn test_sort_number_bubble() -> TestResult {
    let expected = "Ascending order: [1, 2, 3, 5, 7]\nDescending order: [7, 5, 3, 2, 1]\n";
    let mut cmd = Command::cargo_bin("1_2")?;
    cmd.arg("1").arg("5").arg("2").arg("7").arg("3").assert().success().stdout(expected);

    Ok(())
}  

#[test]
fn sort_points() -> TestResult {
    let expected = "Ascending order: [(1.0, 5.0), (2.0, 7.0)]\nDescending order: [(2.0, 7.0), (1.0, 5.0)]\n";
    let mut cmd = Command::cargo_bin("2_1")?;
    cmd.arg("1").arg("5").arg("2").arg("7").arg("3").assert().success().stdout(expected);

    Ok(())
}  

#[test]
fn sort_points_bubble() -> TestResult {
    let expected = "Ascending order: [(1.0, 3.0), (1.0, 5.0), (3.0, 2.0)]\nDescending order: [(3.0, 2.0), (1.0, 5.0), (1.0, 3.0)]\n";
    let mut cmd = Command::cargo_bin("2_2")?;
    cmd.arg("1").arg("5").arg("1").arg("3").arg("3").arg("2").assert().success().stdout(expected);

    Ok(())
}  
