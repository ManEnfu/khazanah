# Cargo flags

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())

cargo_bin  = find_program('cargo')
cargo_opt  = [ '--manifest-path', meson.project_source_root() / 'khazanah/Cargo.toml' ]
cargo_opt += [ '--target-dir', meson.project_build_root()  / 'target' ]
cargo_env  = [ 'CARGO_HOME=' + meson.project_build_root()  / 'cargo-home' ]

if get_option('buildtype') == 'release'
  cargo_options += [ '--release' ]
  rust_target = 'release'
else
  rust_target = 'debug'
endif

# Build target

khazanah_bin = 'khazanah'

cargo_build = custom_target(
  'khazanah-cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'khazanah',
  console: true,
  command: [
    cargo, 'build',
    cargo_opt, '&&', 'cp', 'target' / rust_target / meson.project_name(), '@OUTPUT@',
  ]
)
