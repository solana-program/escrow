# Install dependencies
install:
	pnpm install

# Generate IDL from Rust code using Codama
generate-idl:
	@echo "Generating IDL..."
	pnpm run generate-idl

# Generate clients from IDL using Codama
generate-clients: generate-idl
	@echo "Generating clients..."
	pnpm run generate-clients

# Build the program
build: generate-idl generate-clients build-test-hook
	cd program && cargo-build-sbf

# Build test hook program variants (allow + deny)
build-test-hook:
	cd tests/test-hook-program && cargo-build-sbf --features allow
	cp target/deploy/test_hook_program.so target/deploy/test_hook_allow.so
	cd tests/test-hook-program && cargo-build-sbf --features deny
	cp target/deploy/test_hook_program.so target/deploy/test_hook_deny.so

# Format / lint code
fmt:
	cargo fmt -p escrow-program -p tests-escrow-program
	@cd program && cargo clippy --all-targets -- -D warnings
	@cd tests && cargo clippy --all-targets -- -D warnings
	pnpm format

check:
	cd program && cargo check --features idl
	pnpm run format:check

# Run unit tests
unit-test:
	cargo test -p escrow-program

# Run integration tests (use --with-cu to track compute units)
integration-test *args:
	#!/usr/bin/env bash
	set -e
	if [[ "{{args}}" == *"--with-cu"* ]]; then
		mkdir -p .cus
		rm -f .cus/results.txt
		CU_TRACKING=1 cargo test -p tests-escrow-program -- --test-threads=1
		echo ""
		echo "╔══════════════════════════════════════════════════════════════╗"
		echo "║              Compute Units Summary                           ║"
		echo "╠══════════════════════════════════════════════════════════════╣"
		if [ -f .cus/results.txt ]; then
			sort .cus/results.txt | uniq | while IFS=, read name cus; do
				printf "║ %-44s │ %9s   ║\n" "$name" "$cus"
			done
		fi
		echo "╚══════════════════════════════════════════════════════════════╝"
	else
		cargo test -p tests-escrow-program "$@"
	fi

# Run all tests (use --with-cu to track compute units)
test *args: build unit-test (integration-test args)

# ******************************************************************************
# Deployment (requires txtx CLI: cargo install txtx)
# ******************************************************************************

[private]
check-txtx:
	@command -v txtx >/dev/null 2>&1 || { echo "Error: txtx not found. Install with: cargo install txtx"; exit 1; }

# Deploy to devnet (supervised mode with web UI)
deploy-devnet: check-txtx
	txtx run deploy -e devnet

# Deploy to devnet (unsupervised/CI mode)
deploy-devnet-ci: check-txtx
	txtx run deploy -e devnet -u

# Deploy to localnet (for testing with local validator)
deploy-localnet: check-txtx
	txtx run deploy -e localnet

# ******************************************************************************
# Release
# ******************************************************************************

# Prepare a new release (bumps versions, generates changelog)
[confirm('Start release process?')]
release:
	#!/usr/bin/env bash
	set -euo pipefail

	if [ -n "$(git status --porcelain)" ]; then
		echo "Error: Working directory not clean"
		exit 1
	fi

	command -v git-cliff &>/dev/null || { echo "Install git-cliff: cargo install git-cliff"; exit 1; }

	# Get current versions
	rust_version=$(grep "^version" clients/rust/Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
	ts_version=$(node -p "require('./clients/typescript/package.json').version")

	echo "Current versions:"
	echo "  Rust client:       $rust_version"
	echo "  TypeScript client: $ts_version"
	echo ""

	read -p "New version: " version
	[ -z "$version" ] && { echo "Version required"; exit 1; }

	echo "Updating to $version..."

	# Update Rust client version
	sed -i.bak "s/^version = \".*\"/version = \"$version\"/" clients/rust/Cargo.toml
	rm -f clients/rust/Cargo.toml.bak

	# Update TypeScript client version
	cd clients/typescript && npm version "$version" --no-git-tag-version --allow-same-version
	cd ../..

	echo "Generating CHANGELOG..."
	last_tag=$(git tag -l "v*" --sort=-version:refname | head -1)
	if [ -z "$last_tag" ]; then
		git-cliff --config .github/cliff.toml --tag "v$version" --output CHANGELOG.md --strip all
	elif [ -f CHANGELOG.md ]; then
		git-cliff "$last_tag"..HEAD --tag "v$version" --config .github/cliff.toml --strip all > CHANGELOG.new.md
		cat CHANGELOG.md >> CHANGELOG.new.md
		mv CHANGELOG.new.md CHANGELOG.md
	else
		git-cliff "$last_tag"..HEAD --tag "v$version" --config .github/cliff.toml --output CHANGELOG.md --strip all
	fi

	git add clients/rust/Cargo.toml clients/typescript/package.json CHANGELOG.md

	echo ""
	echo "Ready! Next steps:"
	echo "  git commit -m 'chore: release v$version'"
	echo "  git push origin HEAD"
	echo "  Trigger 'Publish Rust Client' and 'Publish TypeScript Client' workflows"
