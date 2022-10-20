release:
	$(eval TIME=$(shell date '+%Y-%m-%d_%H-%M'))
	$(eval DIR=$(shell echo ./build/builds/$(TIME)))

	@if [ ! -d $(DIR) ]; then \
		echo "Dir DOES NOT exist"; \
		mkdir $(DIR); \
		cp -r ./build/template/. $(DIR)/.; \
		$(MAKE) -C minecraft_server_starter release; \
		mv ./minecraft_server_starter/target/release/minecraft_server_starter $(DIR)/.; \
		mkdir $(DIR)/static; \
		cd ./frontend && NUXT_PUBLIC_API_URL="" npx nuxi generate; \
		mv ./.output/public/* ../$(DIR)/static/; \
	else \
		echo "You have lately created a build"; \
		echo "Please remove that build or wait a minute!"; \
	fi

	@# $(MAKE) -C minecraft_server_starter release