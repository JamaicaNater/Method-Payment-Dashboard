up:
	docker-compose -f docker-compose.yml build
	docker-compose -f docker-compose.yml up
up-db:
	docker-compose -f docker-compose.yml up db
run:
	cd app && RUST_BACKTRACE=1 RUST_LOG=debug DB_HOST=localhost DB_NAME=method_data DB_USER=user DB_PASS=pass DB_PORT=3306 cargo run
fmt:
	cd app && cargo fmt
clean:
	docker-compose down --remove-orphans
	docker volume rm method_assesment_my-db
