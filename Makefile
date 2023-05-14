.PHONY: migrate, clean, reset

DB_CONTAINER_NAME=invisibot-server-db-1

migrate:
	cd backend && cargo sqlx migrate run --source postgres/migrations && cd ..

clean:
	echo 'DROP SCHEMA public CASCADE; CREATE SCHEMA public;' | docker exec -i ${DB_CONTAINER_NAME} psql -U invisibot invisibot

reset:
	make clean
	make migrate
