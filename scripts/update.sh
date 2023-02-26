#!/usr/bin/env bash

info() {
	echo "[info]: $@";
}

message() {
	echo "[message]: $@";
}

error() {
	echo "[error]: $@";
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
