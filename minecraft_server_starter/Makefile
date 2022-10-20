test:
	$(MAKE) build_dev
	ROCKET_LOG_LEVEL=normal PANEL_SERVE_FRONTEND=false ./target/release/minecraft_server_starter
build:
	$(MAKE) clean
	cargo build -r
	
build_dev:
	$(MAKE) clean
	cargo build -r --features "debug"

clean:
	# pkill java 2> /dev/null || echo > /dev/null
	rm ./run/./world/session.lock ./run/./world_nether/session.lock ./run/./world_the_end/session.lock 2> /dev/null || echo > /dev/null

release:
	$(MAKE) clean
	$(MAKE) build
	upx --best --lzma ./target/release/minecraft_server_starter