test:
	$(MAKE) clean
	cargo build -r
	ROCKET_LOG_LEVEL=normal ./target/release/minecraft_server_starter

clean:
	rm ./run/./world/session.lock ./run/./world_nether/session.lock ./run/./world_the_end/session.lock 2> /dev/null || echo > /dev/null