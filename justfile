
project := "replica"

alias t := test
alias pre := test-all
alias b := build
alias rel := release
alias fmt := format

os := `uname`


# run the standard tests
test:
    clear
    /bin/rm -fr tests/tback-tmp
    cargo test
    just restore


# run the standard tests + clippy and fmt
test-all:
    clear
    /bin/rm -fr tests/tback-tmp
    cargo test -- --include-ignored && just format
    just restore

# restore the test files so they 
restore:
    git checkout .test-replica/data/run2-file.json tests/changed-file.txt

# clean the project
clean:
    /bin/rm -fr tests/tback-tmp
    cargo clean

# build the debug target
build:
    clear
    cargo build

# run fmt and clippy
format:
    cargo fmt && cargo clippy

# build the docs
docs:
    cargo doc --no-deps --open

# run the debug app
run:
    clear && cargo run --bin replica

# build the release
release:
    clear
    cargo build --release --bins

# watch the current folders and run tests when a file is changed
watch:
    watchexec -d 500 -c -e rs cargo test && cargo fmt && cargo clippy

# cover - runs code test coverage report and writes to coverage folder
cover:
    /bin/rm -fr tests/tback-tmp
    cargo tarpaulin --out html --output-dir coverage && mv coverage/tarpaulin-report.html coverage/index.html
    just restore

# start a http server in the coverage folder
serve-cover:
    cd coverage && python3 -m http.server 8080

# merge the develop branch to main
merge:
    git push && git checkout main && git pull && git merge develop && git push && git checkout develop

# install the app
install:
    just release
    ./install.sh
