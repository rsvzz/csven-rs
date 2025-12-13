# Makefile auxiliar para proyectos Meson

build:     
	meson setup builddir --prefix=/usr  ninja -C builddir

install:
	DESTDIR=$(DESTDIR) meson install -C builddir

clean:
	ninja -C builddir clean

