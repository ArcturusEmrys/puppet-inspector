# Puppet Inspector

Software for examining the data inside of virtual puppet files such as Inochi2D INP, etc.

## Building

I am very sorry you have to do this.

1. Install Rust. This is the easy part.
2. Install GTK.
  * If you're on Ubuntu you probably already have this and just need the dev packages.
  * On Windows, there are two paths to install GTK.
    * GTK itself recommends MSYS2, but GTK-RS recommends gvsbuild.
    * I actually chose neither! gvsbuild distributes nightlies as a ZIP that can be installed into `C:\gtk`. I set the following path vars:
      * `PATH` - add `C:\gtk\bin` to the existing list
      * `INCLUDE` - `C:\gtk\include;C:\gtk\include\cairo;C:\gtk\include\glib-2.0;C:\gtk\include\gobject-introspection-1.0;C:\gtk\lib\glib-2.0\include;` (if you have this already, also add them to the existing list)
      * `LIB` - `C:\gtk\lib`
      * `PKG_CONFIG_PATH` - `C:\gtk\lib\pkgconfig`
  * On Apple, god help you.