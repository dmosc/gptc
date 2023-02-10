#!/usr/bin/env bash

set -e;

RED='\033[0;31m';
BLUE="\033[0;34m";
YELLOW="\033[1;33m";
NO_COLOR="\033[0m";

info() {
	echo -e "${YELLOW}[info]: $@${NO_COLOR}";
}

message() {
	echo -e "${BLUE}[message]: $@${NO_COLOR}";
}

error() {
	echo -e "${RED}[error]: $@${NO_COLOR}";
	exit 1;
}

main() {
	BIN_PATH="$HOME/.local/bin";
	BIN_NAME="gptc";
	URL="https://github.com/dmosc/gptc/releases/latest/download/gptc";

	# If $BIN_PATH is not included in the main $PATH
	# we add it to make sure the binary is findable.
	if [[ ":$PATH:" != *":$BIN_PATH:"* ]]; then
		if [[ $SHELL == *"/zsh" ]]; then
			SHELL_PROFILE="$HOME/.zshrc";
		elif [[ $SHELL == *"/bash" ]]; then
			SHELL_PROFILE="$HOME/.bashrc";
		else
			error "Can't find shell config; try adding $BIN_PATH to your \$PATH variable.";
		fi
		message "Using $SHELL_PROFILE as shell config.";
		# Export $PATH with $BIN_PATH attached at the end and
		# append the command inside the $SHELL_PROFILE config.
		echo "export PATH=\"\$PATH:$BIN_PATH\"" >> $SHELL_PROFILE;
	fi


	if [[ -z ${OPENAI_KEY} ]]; then
		message "Adding \$OPENAI_KEY to $SHELL_PROFILE config file; make sure to set your personal API key.";
		echo "# Create an OpenAI API key; see https://platform.openai.com/account/api-keys." >> $SHELL_PROFILE;
		echo "export OPENAI_KEY=" >> $SHELL_PROFILE;
	fi

	message "Ensuring $BIN_PATH exists";
	mkdir -p $BIN_PATH;
	message "Downloading latest binary from $URL";
	curl -L $URL -o "$BIN_PATH/$BIN_NAME";
	chmod +x "$BIN_PATH/$BIN_NAME";
	message "Finished installing gptc command inside $BIN_PATH";
	info "Make sure to go to $SHELL_PROFILE and set a valid value for \$OPENAI_KEY to start making queries.";
	info "(i.e.): gptc --prompt \"What's the difference between apples and oranges?\"";
	exec $SHELL;
}

main || exit 1;
