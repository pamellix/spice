# Spice - CLI Utility for Database Operations Management

![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

`spice` is a CLI utility written in Rust, designed for managing database operations such as migrations and backups.

## Requirements

- [Rust](https://www.rust-lang.org/) 1.56 or newer
- Cargo (should be installed along with Rust)

## Installation

1. **Clone the repository to your computer**:

   ```sh
   git clone https://github.com/pamellix/spice.git
2. **Navigate to the project directory**:

    ```sh
    cd spice
3. **Build the project using Cargo**:

    ```sh
    cargo build --release
This command will create an executable file in the target/release folder.

## Running the CLI Utility

After successfully building the project, you can run the CLI utility through Cargo or directly from the compiled binary file.

## Running via Cargo

To run the program with Cargo, use the following command:

    cargo run -- <command> [options]
For example:

    cargo run -- migrate -o backup.sql
## Running from the Binary File:

1. **Go to the target/release folder**:
    ```sh
    cd target/release
2. **Run the program**:
    ```sh
    ./spice <command> [options]

## Running the Program Directly from Terminal

To make the `spice` program executable directly from the terminal without needing `cargo run`, follow these steps:

1. **Build the project in release mode**:

   ```sh
   cargo build --release

This will create an executable file in the target/release directory.

2. **Move the executable to a directory in your PATH**:

Move the compiled binary to a directory that's included in your system's PATH, such as /usr/local/bin:

    sudo mv target/release/spice /usr/local/bin/
Now you can run the program from anywhere in the terminal.

3. **Verify the installation**:

Run the following command to ensure the program is accessible:

    spice --help
This should display the help information for the spice program.

After installing, you can use the program directly:

    spice migrate -o backup.sql

## Usage

The program supports multiple commands. For example, the migrate command can be used to migrate the database or create a backup.

### Example migrate Command: 

    cargo run -- migrate -t pull --db_type postgres --host localhost -u user -p password -d mydatabase -o backup.sql
#### Parameters Description:

`-t`, `--operation_type`: Specifies the operation type (pull or migrate). Defaults to pull.<br><br>
`--db_type`: Specifies the database type (e.g., postgres, and for this release, it supports only postgres).<br><br>
`--host`: Specifies the database host (e.g., localhost).<br><br>
`-u`, `--user`: Specifies the database username.<br><br>
`-p`, `--password`: Specifies the database password.<br><br>
`-d`, `--database`: Specifies the database name.<br><br>
`-o`, `--output_file`: Specifies the output file name for creating a backup.<br><br>

#### Configuration with a Config File
The CLI utility supports loading configuration from a config.json file. If this file exists in the project root directory, the program will load parameters from it, allowing you to avoid specifying them on the command line.

## Making Changes and Testing
Modify the source code as needed.

### Compile and run the program to test your changes:

    cargo run -- <command> [options]

  
### Use Git to commit changes:

    git add .
    git commit -m "Your message"
  
## License
This project is licensed under the MIT License. See the LICENSE file for more details.
