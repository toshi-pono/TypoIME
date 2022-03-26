output_dir = output
app_name = typo_ime
bin_name = typo_ime

.PHONY: app
app:
	cargo build --release
	mkdir -p $(output_dir)/$(app_name).app/Contents
	mkdir $(output_dir)/$(app_name).app/Contents/MacOS
	mkdir $(output_dir)/$(app_name).app/Contents/Resources
	cp Info.plist $(output_dir)/$(app_name).app/Contents/
	cp target/release/$(bin_name) $(output_dir)/$(app_name).app/Contents/MacOS/
	cp -r resources/* $(output_dir)/$(app_name).app/Contents/Resources/

.PHONY: clean
clean:
	rm -rf $(output_dir)/$(app_name).app

.PHONY: run
run:
	./$(output_dir)/$(app_name).app/Contents/MacOS/$(bin_name)
