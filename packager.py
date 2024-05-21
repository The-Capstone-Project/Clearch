import subprocess
import os

def package_executable_to_deb(rust_project_path):
    # Change to the Rust project directory
    os.chdir(rust_project_path)

    # Build the Rust project
    try:
        subprocess.run(["cargo", "build", "--release"], check=True)
    except subprocess.CalledProcessError:
        print("Build failed")
        return

    #install cargo-deb if not installed
    try:
        subprocess.run(["cargo", "install", "cargo-deb"], check=True)
    except subprocess.CalledProcessError:
        print("cargo-deb installation failed")
        return
        

    # Package the executable into a .deb file using cargo-deb
    output_path = subprocess.run(["cargo", "deb"], check=True, capture_output=True)
    deb_file_path = output_path.stdout.decode().strip().split("\n")[-1]

    # Verify the .deb file using dpkg-deb
    try:
        subprocess.run(["dpkg-deb", "-I", deb_file_path], check=True)
        print("Verification successful")
    except subprocess.CalledProcessError:
        print("Verification failed")
        return
    
    print("Deb file path: ", deb_file_path)

    return deb_file_path

if __name__ == "__main__":
    package_executable_to_deb(os.curdir)