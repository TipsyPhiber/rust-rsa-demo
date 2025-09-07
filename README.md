# rust-rsa-demo
Author: Bruce Mikel
This is a simple command-line program written in Rust to demonstrate the core concepts of the RSA encryption and decryption algorithm. It is intended for educational purposes to make the theoretical mathematics of RSA tangible and interactive.

The program generates a fixed public/private key pair, prompts the user to enter a number, encrypts it, and then decrypts it back to the original number, showing the intermediate steps.
Prerequisites

To compile and run this program on Windows, you will need to set up the Rust development environment.
Step 1: Install Visual Studio Build Tools

Rust on Windows requires a C++ linker and the Windows SDK, which are included in the Visual Studio Build Tools.

    Download the installer from the official site: Build Tools for Visual Studio.

    Run the installer.

    In the "Workloads" tab, select "Desktop development with C++". This is the only option you need to check.

    Click "Install" and allow the installation to complete. A system restart is recommended afterward.

Step 2: Install Rust

    Go to the official Rust website: https://www.rust-lang.org/tools/install.

    Download and run rustup-init.exe.

    A terminal window will open. Press 1 and then Enter to proceed with the default installation.

    Once finished, close and re-open your terminal (Command Prompt or PowerShell) for the changes to take effect.

    Verify the installation by typing rustc --version. You should see the installed compiler version.

How to Run the Program

    Clone the Repository (or download the files):

    git clone https://github.com/TipsyPhiber/rust-rsa-demo.git
    cd rsa-demo

    Compile and Run with Cargo:
    Navigate to the project's root directory in your terminal and run the following command:

    cargo run

    Cargo, Rust's build tool and package manager, will handle everything: it will compile the dependencies, build the executable, and run the program in one step.

Sample Usage

When you run the program, it will first display the generated keys and then prompt you for input.

--- RSA Algorithm Demonstration ---

[ KEY GENERATION ]
Using primes p=61 and q=53
Public Key: (e=17, n=3233)
Private Key: (d=2753, n=3233)

[ ENCRYPTION / DECRYPTION CYCLE ]
Enter a number to encrypt (must be less than 3233): 42

Original Message: 42
Encrypting with Public Key...
  Ciphertext = 42^17 mod 3233 = 2108
Decrypting with Private Key...
  Decrypted Message = 2108^2753 mod 3233 = 42

Success! The original message was recovered.
