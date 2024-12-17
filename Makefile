# Makefile for client & server GRPC Generation
# https://github.com/hyperium/tonic

# Environment Variables
SCHEMA_OUTPUT = ./crates/schema/generated/

.PHONY: clean
clean: ## Deletes the output directory.
	$(call print-info, "Cleaning project...")
	rm -f $(SCHEMA_OUTPUT)
	$(call print-success, "Project cleaned.")
