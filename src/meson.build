inc_src = include_directories('../src')

gtk_dep = dependency('gtk4', required : true)
adw_dep = dependency('libadwaita-1', required : true)

cargo = find_program('cargo', required: true)

custom_target('csven',
  output: 'csven',
  build_by_default: true,
  install : true,
  install_dir : get_option('bindir'),
  command: [
    'sh', '-c',
    'cargo build --release && cp ../target/release/csven src/csven'
  ],
)