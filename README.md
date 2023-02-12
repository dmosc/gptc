# gptc

A light weight CLI to interact with the OpenAI models from your terminal written in Rust. OpenAI's LLMs are bringing a whole new dimension to the development experience and it's important to bridge the communication with our every day tools.

## Installation and setup

Run the following script to download the latest `gptc` binary:

```
curl -fsSL https://raw.githubusercontent.com/dmosc/gptc/main/scripts/install.sh | sh
```

During installation, `gptc` will prompt you for an OpenAI API key and export the `$OPENAI_KEY` environment variable inside your shell configuration file (i.e. `.zshrc`, `.bashrc`) with the provided key. The script installs `gptc`'s binary inside `/usr/local/bin` and appends the path to your `$PATH` variable if it isn't registered [(get an API key from OpenAI's platform)](https://platform.openai.com/account/api-keys).

## Usage

```
gptc --help
gptc "linux script to install a binary in bin/ directory"
gptc "create an index.html file with common html boilerplate"
```

## Troubleshooting

_Since it's a first version, feel free to ping me if something's not right._

### Installation

If `gptc` is failing to install, read through the error message. Hopefully is informative enough to let you know what's going on.

### Usage

If the installation is successfull, make sure that:

- `$OPENAI_KEY` environment variable is set and loaded; try to `echo $OPENAI_KEY` and see if you get the key back.
- Try `gptc --help` to get additional info on flags and what they do.
