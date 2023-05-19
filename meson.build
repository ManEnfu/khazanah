# Project Definition

project('khazanah', 'rust', 
          version: '0.1.0',
          license: 'GPL-3.0-or-later',
    meson_version: '>= 0.59.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

# Dependencies

dependency('glib-2.0', version: '>= 2.66')
dependency('gio-2.0', version: '>= 2.66')
dependency('gtk4', version: '>= 4.6.0')
dependency(
  'libadwaita-1', version: '>= 1.2.0',
  fallback: ['libadwaita', 'libadwaita_dep'],
  default_options: ['tests=false', 'examples=false', 'vapi=false']
)

cargo = find_program(
  'cargo',
  native: true,
  required: true,
)

version = meson.project_version()

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
# iconsdir = datadir / 'icons'
# podir = meson.project_source_root() / 'po'
# gettext_package = meson.project_name()

# i18n = import('i18n')
# gnome = import('gnome')

subdir('khazanah-core')
subdir('khazanah-gtk')

# subdir('data')
# subdir('src')
# subdir('po')

# gnome.post_install(
#      glib_compile_schemas: true,
#     gtk_update_icon_cache: true,
#   update_desktop_database: true,
# )
