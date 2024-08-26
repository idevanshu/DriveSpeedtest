use std::fs::{File, OpenOptions};
use std::io::{Write, Read, BufWriter, BufReader, stdin, stdout, Error};
use std::time::Instant;
use std::process::Command;
use std::path::PathBuf;

const FILE_SIZE: usize = 1024 * 1024 * 1024; // 1 GB
const FILE_NAME: &str = "test_speed.bin";

fn main() -> Result<(), Error> {
    let drives = list_mounted_drives();

    // Display available drives
    println!("Available mounted drives:");
    for (i, drive) in drives.iter().enumerate() {
        println!("{}. {}", i + 1, drive.display());
    }

    // Prompt user to select a drive
    println!("Select a drive to test (enter the number):");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().expect("Please enter a valid number");

    if choice == 0 || choice > drives.len() {
        println!("Invalid choice!");
        return Ok(());
    }

    let selected_drive = &drives[choice - 1];
    let file_path = selected_drive.join(FILE_NAME);

    // Generate random data
    let data = vec![0u8; FILE_SIZE];

    // Write Test
    let start = Instant::now();
    let mut file = BufWriter::new(File::create(&file_path)?);
    file.write_all(&data)?;
    file.flush()?;
    let duration = start.elapsed();
    println!(
        "Write speed: {:.2} MB/s",
        (FILE_SIZE as f64 / (1024.0 * 1024.0)) / duration.as_secs_f64()
    );

    // Read Test
    let start = Instant::now();
    let mut file = BufReader::new(OpenOptions::new().read(true).open(&file_path)?);
    let mut buffer = vec![0u8; FILE_SIZE];
    file.read_exact(&mut buffer)?;
    let duration = start.elapsed();
    println!(
        "Read speed: {:.2} MB/s",
        (FILE_SIZE as f64 / (1024.0 * 1024.0)) / duration.as_secs_f64()
    );

    // Clean up
    std::fs::remove_file(file_path)?;

    Ok(())
}

fn list_mounted_drives() -> Vec<PathBuf> {
    // This function lists mounted drives by parsing the output of the `df` command (on Unix-like systems)
    let output = Command::new("df")
        .arg("-h")
        .output()
        .expect("Failed to execute `df` command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut drives = Vec::new();

    for line in output_str.lines().skip(1) { // Skip header line
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
            drives.push(PathBuf::from(parts[5]));
        }
    }
    drives
}
