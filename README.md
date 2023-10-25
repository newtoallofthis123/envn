# EnvN

EnvN is the secret manager for busy developers. It is a very simple to use tool that allows you to store your secrets *encrypted* in a simple file
that you can theoretically commit to your repository.

## Idealogy and Design

EnvN is designed with simplicity in mind. It is a single binary that you can download and use a package manager to install. It is also designed to be as user friendly as possible for people who are not familiar with the command line.

EnvN can be used as per as your convenience. You can use it as a TUI or a fully compatible CLI with piping and all. It is also designed to be as portable as possible.

Written in rust, EnvN is quite fast and efficient. It is also designed to be as secure as possible. It uses the AES-256 encryption algorithm to encrypt your secrets and store them all in a single SQLite database file.

> You don't need to have SQLite installed on your system. EnvN comes with a pre-compiled SQLite library.

## Installation

### Using Cargo

```bash
cargo install envn
```

### Getting the Binary for your OS

- [Windows](https://github.com/newtoallofthis123/envn/releases/download/v0.1.0/envn.exe)

- [Linux](https://github.com/newtoallofthis123/envn/releases/download/v0.1.0/envn)

Help us by building binaries for other OSes and submitting a PR.

## Usage

### Initial Setup

The first time you open EnvN, you will be asked to enter a password. This password will be used to encrypt your secrets. You will be asked to enter this password every time you open EnvN.

You can later reset this, but you will *lose all your secrets*. So be careful.

### Available Commands

![Main](/assets/main.png)

- `set` - Add a new secret
- `get` - Get an existing secret
- `edit` - Edit an existing secret
- `delete` - Delete an existing secret
- `show` - List all secrets
- `load` - Load secrets to a file
- `from` - Load secrets from a file

For more information, run `envn help`.

### Understanding the CLI Interface

I tried to make the CLI interface as user friendly as possible. You can use the CLI interface in two ways:

- **Interactive Mode** - This is the default mode. You can use the arrow keys to navigate through the options and press enter to select an option. You can also use the `tab` key to switch between the options and the `space` key to select an option.

- **Command Mode** - You can use pass in arguments to the CLI to directly execute a command. For example, `envn set` will directly execute the `set` command. You can also use the `--help` flag to get help for a command. For example, `envn set --help` will show you the help for the `set` command.

## Contributing

You can contribute to EnvN by:

- Reporting bugs
- Suggesting new features
- Adding new features and fixing bugs

## License

EnvN is licensed under the MIT License. See [LICENSE](/LICENSE) for more information.
