pkgdatadir = get_option('prefix') / get_option('datadir') / meson.project_name()

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', 'done')
conf.set_quoted('LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))
conf.set_quoted('PKGDATADIR', pkgdatadir)
conf.set_quoted('APPLICATION_ID', application_id)
conf.set_quoted('PROFILE', get_option('profile'))

rust_sources = files(
  'main.rs',
  'schema.rs',
  'app/constants.rs',
  'app/config.rs',
  'app/application.rs',
  'app/window.rs',
  'core/local/lists.rs',
  'core/local/tasks.rs',
  'core/local/mod.rs',
  'core/mod.rs',
  'models/list.rs',
  'models/task.rs',
  'models/mod.rs',
  'storage/database.rs',
  'storage/mod.rs',
  'widgets/app.rs',
  'widgets/task_container.rs',
  'widgets/details.rs',
  'widgets/list.rs',
  'widgets/mod.rs',
  'widgets/sidebar.rs',
  'widgets/task_list.rs',
)

sources = [cargo_sources, rust_sources]

cargo_script = find_program(join_paths(meson.project_source_root(), 'build-aux/cargo.sh'))
cargo_release = custom_target(
  'cargo-build',
  build_by_default: true,
  input: sources,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    cargo_script,
    meson.project_build_root(),
    meson.project_source_root(),
    '@OUTPUT@',
    get_option('buildtype'),
    meson.project_name(),
  ]
)

subdir('app')
