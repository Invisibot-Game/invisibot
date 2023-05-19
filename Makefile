PHONY: migrate, clean, reset

DB_CONTAINER_NAME=invisibot-db-1

mock: mock.sql
	docker exec -i ${DB_CONTAINER_NAME} psql -U invisibot invisibot < mock.sql

migrate:
	cd backend && cargo sqlx migrate run --source postgres/migrations -D postgres://invisibot:password@localhost:5432/invisibot && cd ..

clean:
	echo 'DROP SCHEMA public CASCADE; CREATE SCHEMA public;' | docker exec -i ${DB_CONTAINER_NAME} psql -U invisibot invisibot

reset:
	make clean
	make migrate
	make mock
