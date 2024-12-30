up:
	docker compose up & \
	cargo run

up-test:
	docker compose up -d && \
	cargo test && \
	docker compose down
	
down :
	docker compose down