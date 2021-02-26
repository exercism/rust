# check_exercises.sh for Windows Rust Developers

It is possible to run `check_exercises.sh` on Windows 10, pointing to the Windows location for your GitHub repository.  This is done with the Ubuntu on Windows subsystem.

## Enable Developer Mode
To run Ubuntu on Windows, you need to be in Developer Mode.

 - Open Settings
 - Open Update and Security
 - Select For Developers on Left Side
 - Change to Developer Mode from Sideload Apps

## Install

Start a PowerShell as Administrator.

Run the following:

    Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux

## Run bash

The `bash` command now gives you a terminal in a Ubuntu Linux instance.  You have access to Windows files via /mnt/[drive_letter]

Example: Windows user directory would be

    /mnt/c/Users/username

## Installing Rust

Inside bash, you will not have access to Window's Rust.  You need to install the Linux version of Rust.

    curl -sf -L https://static.rust-lang.org/rustup.sh | sh

You also need to install a cc linker for Rust.

    sudo apt-get install build-essential

## Running Tests

    cd /mnt/c/[path of github project]
    _test/check_exercises.sh

This will re-download and build any crates needed, as they only existed in your Windows Rust.
