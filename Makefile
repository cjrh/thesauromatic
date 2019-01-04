default debug: source/*.d
	dub build --build=debug

release: source/*.d
	dub build --build=release

clean:
	rm thesauromatic
	rm -rf .dub/
