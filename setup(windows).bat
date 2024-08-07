@echo off

REM Check if Rust is installed
where rustc >nul 2>nul
if %errorlevel% neq 0 (
    echo Rust is not installed. Please install Rust from https://www.rust-lang.org/
    exit /b 1
)

REM Create a new Rust project
cargo new stellar_reality
cd stellar_reality

REM Add dependencies to Cargo.toml
echo. >> Cargo.toml
echo [dependencies] >> Cargo.toml
echo rand = "0.8.5" >> Cargo.toml
echo termion = "1.5.6" >> Cargo.toml

echo Setup complete! To start playing the game:
echo 1. Copy the game code into src\main.rs
echo 2. Run 'cargo run' in the stellar_reality directory