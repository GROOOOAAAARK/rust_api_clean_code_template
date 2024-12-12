# Rust Clean Code Api template

This project is a template for Rust API development.

## Architecture

This project's architecture is based on the Clean Architecture principles described by Robert C. Martin in his book "Clean Architecture: A Craftsman's Guide to Software Structure and Design". I tried to follow the principles as closely as possible, but some parts could be improved.

The project's architecture is as follows:

- **Domain**: Contains the data models.
- **Usecases**: Contains the business logic.
- **Adapters**: Contains the adapters for the external services and the controllers.
- **Infrastructure**: Contains the infrastructure code.

## Testing

This project tests are divided in 2 categories:

- **unit**: Contains the unit tests for the usecases.
- **e2e**: Contains the end to end tests for the whole application.
