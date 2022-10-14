test:
	$(MAKE) build
	ROCKET_LOG_LEVEL=normal ./target/release/minecraft_server_starter
build:
	$(MAKE) clean
	cargo build -r
	

clean:
	# pkill java 2> /dev/null || echo > /dev/null
	rm ./run/./world/session.lock ./run/./world_nether/session.lock ./run/./world_the_end/session.lock 2> /dev/null || echo > /dev/null

release:
	$(MAKE) clean
	upx --best --lzma ./target/release/minecraft_server_starter