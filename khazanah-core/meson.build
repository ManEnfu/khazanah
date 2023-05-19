# Cargo flags

cargo_opt  = [ '--manifest-path', meson.project_source_root() / 'khazanah-core/Cargo.toml' ]
cargo_opt += [ '--target-dir', meson.project_build_root()  / 'target' ]
cargo_env  = [ 'CARGO_HOME=' + meson.project_build_root()  / 'cargo-home' ]

if get_option('buildtype') == 'release'
  cargo_options += [ '--release' ]
  rust_target = 'release'
else
  rust_target = 'debug'
endif

# Build target

rust_lib = 'libkhazanah_core.rlib'

cargo_build = custom_target(
  rust_lib,
  build_by_default: true,
  build_always_stale: true,
  output: rust_lib,
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    cargo, 'build',
    cargo_opt,
    '&&',
    'cp', 'target' / rust_target / rust_lib, '@OUTPUT@',
  ]
)

