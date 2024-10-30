[![CI/CD](https://github.com/nogibjj/Javidan_Individual_Project_2/actions/workflows/main.yaml/badge.svg)](https://github.com/nogibjj/Javidan_Individual_Project_2/actions/workflows/main.yaml)

# Rust CLI Project with SQLite Integration

## Overview
This project demonstrates the power of Rust by implementing a command-line interface (CLI) that interacts with a SQLite database. It includes Create, Read, Update, and Delete (CRUD) operations, load the csv into SQLite database,  showcasing the capabilities of Rust in building efficient and safe software. Additionally, this project uses Github Actions to test, build, and lint the Rust codebase, generating an optimized Rust binary as an artifact.

## Key Features
- **CRUD Operations with SQLite**: Interact with a local SQLite database using Rust.
- **Optimized Binary**: Automated process to generate an optimized binary using GitLab Actions.
- **Automated Testing, Building, and Linting**: Implemented GitLab Actions to ensure code quality and maintainability using centralized Makefile.

## Performance metrics 
This project is intended to demonstrate simple CRUD operations, as these types of operations are considered lightweight and not ideal for direct performance comparisons. The Python code performing the same operations can be found in this repository ([GithubLink](https://github.com/nogibjj/Javidan_Karimli_IDS706_PythonInteractWithSql)) and also executes in a short amount of time. Although direct measurements are not provided in this case, the Rust code is technically much faster than the Python code under heavy load. This is because Rust uses a compilation-based approach that converts all the code into a binary format executable directly by the OS on multiple cores and threads. In contrast, Python relies on an interpreter that scans the code during runtime, and the GIL (Global Interpreter Lock) prevents true multicore execution. Performance measurements for the Rust code can be found in the Sample Images section below.

## How LLM was Used
In the development process, an LLM (Language Learning Model) was utilized to refactor the initial Python code into Rust. It assisted in generating efficient, idiomatic Rust code and ensuring adherence to best practices in error handling and memory management. Additionally, the LLM provided guidance on using Rust's syntax effectively for function-based implementations.

## Dependencies
- **Rust**: Make sure you have Rust installed. Follow instructions .
- **SQLite**: A lightweight, self-contained database engine. You can install it using your package manager or download it from .
- **Libraries**: reqwest, rusqlite, csv, clap


## Getting Started
1. Clone the repository:
   ```bash
   git clone https://github.com/nogibjj/Javidan_Individual_Project_2.git
   cd Javidan_Individual_Project_2
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the project:
   ```bash
   cargo run
   ```

4. Test the code:
   ```bash
   make test
   ```

5. Check the project code:
   ```bash
   make check
   ```
6. Check the linting:
   ```bash
   make lint
   ```

7. Format the code:
   ```bash
   make format
   ```

8. To execute CRUD operations:
   - **Create**: Use the `insert` command followed by the necessary parameters.
   ```bash
      cargo run -- insert "http://example.com" 1 "FED" "FORM1" "2024-01-01"
   ```

   - **Read**: Use the `read` command to retrieve entries.
   ```bash
      cargo run -- read 1 # Id
   ```
   - **Update**: Use the `update` command followed by the entry identifier and new data.
   ```bash
      cargo run -- update 1 "http://new-url.com" "NEW_FED" "NEW_FORM" "2024-12-31"
   ```

   - **Delete**: Use the `delete` command followed by the entry identifier.
   ```bash
      cargo run -- delete 1
   ```

## GitHub Actions
The project utilizes GitLab CI/CD for automated testing, building, and linting. The configuration file (`.github/workflows/main.yml`) includes jobs to:
- Test the codebase to ensure that all functionalities work as expected.
- Build the optimized binary to be provided as an artifact.
- Lint the code using Clippy to maintain code quality.
- Format the code using rustfmt

## Optimized Rust Binary
The GitLab CI/CD pipeline is configured to generate an optimized Rust binary. After successful builds, you can download the binary as an artifact from the pipeline page on GitLab.

## Video Demonstration
[Click here to watch the video demonstration](https://youtu.be/TA_TSB0uwNo) 

## Sample Images

**Rust CLI Insert**
![Run Example Console Output](https://github.com/nogibjj/Javidan_Individual_Project_2/blob/ae1c063839c6440b9d85a5d7a0a13685c9733b46/data/insert.png)


**Rust CLI Read**
![Run Example Console Output](https://github.com/nogibjj/Javidan_Individual_Project_2/blob/ae1c063839c6440b9d85a5d7a0a13685c9733b46/data/read.png)


**Rust CLI Update**
![Run Example Console Output](https://github.com/nogibjj/Javidan_Individual_Project_2/blob/ae1c063839c6440b9d85a5d7a0a13685c9733b46/data/update.png)


**Rust CLI Delete**
![Run Example Console Output](https://github.com/nogibjj/Javidan_Individual_Project_2/blob/ae1c063839c6440b9d85a5d7a0a13685c9733b46/data/delete.png)



## License
This project is licensed under the MIT License - see the [LICENSE](https://github.com/nogibjj/Javidan_Individual_Project_2/blob/ae1c063839c6440b9d85a5d7a0a13685c9733b46/LICENSE.txt) file for details.
