up:
	docker compose up -d && \
	cargo run

up-test:
	docker compose up -d && \
	cargo nextest run && \
	docker compose down
	
down :
	docker compose down