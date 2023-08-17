run:
	@cd os && make run

update:
	@cd os && cargo update
	@cd user && cargo update

clean:
	@cd os && cargo clean
	@cd user && cargo clean
