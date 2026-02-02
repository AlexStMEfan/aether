# aether/Makefile
SHELL := /bin/bash
.DEFAULT_GOAL := help

.PHONY: help
help: ## Показать это сообщение
	@echo "Aether — балансировщик нового поколения"
	@echo ""
	@echo "Основные команды:"
	@echo "  make build          Собрать все компоненты"
	@echo "  make test           Запустить тесты"
	@echo "  make fmt            Форматирование кода"
	@echo "  make lint           Проверка линтером"
	@echo "  make docs           Сгенерировать документацию"
	@echo "  make dev            Запустить локальную среду разработки"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Собрать все компоненты
	@cargo build --release

.PHONY: test
test: ## Запустить тесты
	@cargo test --workspace

.PHONY: fmt
fmt: ## Форматирование кода
	@cargo fmt --all

.PHONY: lint
lint: ## Проверка линтером
	@cargo clippy --workspace --all-targets -- -D warnings

.PHONY: docs
docs: ## Сгенерировать документацию
	@cargo doc --workspace --no-deps --open

.PHONY: dev
dev: ## Запустить локальную среду разработки
	@echo "Запуск локального окружения..."
	@docker-compose -f docker-compose.dev.yml up -d 2>/dev/null || echo "Docker Compose не настроен (создайте docker-compose.dev.yml)"

.PHONY: clean
clean: ## Очистить артефакты сборки
	@cargo clean
	@rm -rf target/ docs/target/