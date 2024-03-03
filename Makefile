.PHONY: audit check coverage format machete test all

audit:
	@cargo audit

check:
	@cargo clippy

coverage:
	@cargo tarpaulin

format:
	@cargo fmt

machete:
	@cargo machete

test:
	@cargo nextest run

all: format check test coverage machete audit
