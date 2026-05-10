cargo-sasha:
	cargo install \
		--path crates/cargo-sasha \
		--force

develop:
	cargo sasha update daemon && \
	cargo sasha logs service
