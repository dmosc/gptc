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
	URL="https://github.com/dmosc/gptc/releases/latest/download/gptc";

	if [[ -e "$BIN_PATH/$BIN_NAME" ]]; then
		curl -fsSL -o /tmp/gptc $URL;
		mv /tmp/gptc "$BIN_PATH/$BIN_NAME";
		chmod +x "$BIN_PATH/$BIN_NAME";
		message "Finished updating gptc binary to the latest version.";
		info "Try gptc --help for more information.";
		exec $SHELL;
	else
		message "gptc is not installed on this computer.";
		info "Try instead running: curl -fsSL https://raw.githubusercontent.com/dmosc/gptc/main/scripts/install.sh | sh";
	fi
}

main || exit 1;
