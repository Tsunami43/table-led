include .env

up:
	docker compose up -d --build && make logs

logs:
	docker compose logs -f --tail=100 app

down:
	docker compose down -v

stop:
	docker compose stop app

start:
	docker compose start app && make logs

restart:
	docker compose restart app && make logs

db:
	docker compose exec -it postgres psql -h localhost -U ${DB_USER} -d ${DB_NAME}

ps:
	docker compose ps -a
