# Litra CLI

This is a command-line interface (CLI) for controlling a Litra Beam device on Linux.

## Prerequisites

This application requires access to the USB device. To grant the necessary permissions, a udev rule needs to be set.

## Installation

1. Clone this repository:

```bash
git clone https://github.com/yourusername/litra-cli.git
cd litra-cli
```

2. Build the project:

```bash
cargo build --release
```

3. Copy the udev rules file to the rules directory and reload udev:

```bash
sudo cp src/99-litra.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

## Usage

After installation, you can control your Litra Beam device with the following commands:

```bash
./target/release/litra-cli on  # Turns the Litra Beam on
./target/release/litra-cli off # Turns the Litra Beam off
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
