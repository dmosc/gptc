# gptc

A light weight CLI to interact with the OpenAI models from your terminal written in Rust. OpenAI's LLMs are bringing a whole new dimension to the development experience and it's important to bridge the communication with our every day tools.

## Installation and setup

Run the following script to download the latest `gptc` binary:

```
curl -fsSL https://raw.githubusercontent.com/dmosc/gptc/main/scripts/install.sh | sh
```

The script installs `gptc`'s binary inside `$HOME/.local/bin` and appends the path to your `$PATH` variable. The path will be created if it doesn't exist.

Also, the script registers the `$OPENAI_KEY` environment variable inside your shell configuration file (i.e. `.zshrc`, `.bashrc`).

Once the script finishes downloading the CLI, open your shell configuration file (i.e. `vim $HOME/[.zshrc, .bashrc]`) and set a valid `$OPENAI_KEY` key. [You can get an API key from OpenAI's platform](https://platform.openai.com/account/api-keys).

## Usage

```
gptc --help
gptc --prompt "What's the difference between apples and oranges?"
```

## Troubleshooting

_Since it's a first version, feel free to ping me if something's not right._

### Installation

If `gptc` is failing to install, read through the error message. Hopefully is informative enough to let you know what's going on.

### Usage

If the installation is successfull, make sure that:

- `$OPENAI_KEY` environment variable is set and loaded; try to `echo $OPENAI_KEY` and see if you get the key back.
- Try `gptc --help` to get additional info on flags and what they do.
