alias d := dev
alias w := watch


set dotenv-required := true
set dotenv-load := true
set dotenv-path := "./.env"
set export :=true


default:
    just --list 

[doc("Run application in watch mode")]
watch:
    cargo watch -x run

dev:
    cargo watch --qcx run


[doc("Lint the application")]
lint:
    cargo clippy --all


[doc("Build")]
build:
    cargo build --release 

