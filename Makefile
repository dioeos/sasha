dev:
	cargo make --no-workspace daemon-dev-flow && \
		cargo make --no-workspace cli-dev-flow && \
		cargo make --no-workspace live-logs-flow

cargo-sasha:
	cargo install \
		--path crates/cargo-sasha \
		--force

develop:
	cargo sasha update daemon && \
	journalctl --user -u sasha.service -n 100 -f

service-env:
	systemctl --user start sasha.service && \
	systemctl --user start quickshell.service

reload-env:
	systemctl --user daemon-reload && \
	systemctl --user restart sasha.service
