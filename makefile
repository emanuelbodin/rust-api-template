up:
	docker compose up & \
	cargo run

up-test:
	cargo test
	
down :
	docker compose down