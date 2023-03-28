platform := linux/amd64

remove:
	docker-compose rm -fsv && docker-compose down -v

database:
	docker-compose up -d database

start:
	docker-compose up -d database app

restart: remove database

migrate:
	python3 data_faker.py


.PHONY: remove database restart start migrate