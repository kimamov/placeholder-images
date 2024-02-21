all: dev

dev: db run_dev 


db:
	docker compose up -d


run_dev:
	cargo watch -x run -w 'src'