#!/usr/bin/env bash

RED='\033[0;31m';
BLUE="\033[0;34m";
YELLOW="\033[1;33m";
NO_COLOR="\033[0m";

info() {
	echo "${YELLOW}[info]: $@${NO_COLOR}";
}

message() {
	echo "${BLUE}[message]: $@${NO_COLOR}";
}

error() {
	echo "${RED}[error]: $@${NO_COLOR}";
	exit 1;
}

main() {
	BIN_PATH="/usr/local/bin";
	BIN_NAME="gptc";
	URL="https://github.com/dmosc/gptc/releases/latest/download";
	OS=$(uname -s)

	if [[ $OS == *"Linux"* ]]; then
		OS_PATH="dist/gptc-ubuntu-latest/gptc"
	elif [[ $OS == *"Darwin"* ]]; then
		OS_PATH="dist/gptc-macos-latest/gptc"
	elif [[ $OS == *"Windows"* ]]; then
		OS_PATH="dist/gptc-windows-latest/gptc"
	else
		error "OS: $OS not supported."
	fi
	message "Identified $OS as current OS.";

	if [[ $SHELL == *"/zsh" ]]; then
		SHELL_PROFILE="$HOME/.zshrc";
	elif [[ $SHELL == *"/bash" ]]; then
		SHELL_PROFILE="$HOME/.bashrc";
	else
		error "Can't find shell config; try adding $BIN_PATH to your \$PATH variable.";
	fi
	message "Using $SHELL_PROFILE as shell config.";

	# If $BIN_PATH is not included in the main $PATH
	# we add it to make sure the binary is findable.
	if [[ ":$PATH:" != *":$BIN_PATH:"* ]]; then
		# Export $PATH with $BIN_PATH attached at the end and
		# append the command inside the $SHELL_PROFILE config.
		message "Appending $BIN_PATH to \$PATH variable inside $SHELL_PROFILE.";
		echo "export PATH=\"\$PATH:$BIN_PATH\"" >> $SHELL_PROFILE;
	fi

	if [[ -z ${OPENAI_KEY} ]]; then
		# See https://stackoverflow.com/a/15185051/11672866 for more information on
		# stdout/stderr redirection.
		read -p "Enter your OpenAI's API key: " openai_key < /dev/tty;
		message "Adding \$OPENAI_KEY to $SHELL_PROFILE config file with the provided API key.";
		echo "export OPENAI_KEY=$openai_key" >> $SHELL_PROFILE;
	fi

	message "Ensuring $BIN_PATH exists";
	mkdir -p $BIN_PATH;
	message "Downloading latest binary from $URL/$OS_PATH";
	curl -L "$URL/$OS_PATH" -o "$BIN_PATH/$BIN_NAME";
	chmod +x "$BIN_PATH/$BIN_NAME";
	message "Finished installing gptc command inside $BIN_PATH";
	info "Try gptc --help for more information.";
	exec $SHELL;
}

main || exit 1;
