inc_src = include_directories('../src')

gtk_dep = dependency('gtk4', required : true)
adw_dep = dependency('libadwaita-1', required : true)

cargo = find_program('cargo', required: true)

install_data('../data/styles/io.github.rsvzz.csven.css', install_dir: get_option('prefix') / 'share/csven/styles')
install_data('../data/metainfo/io.github.rsvzz.csven.metainfo.xml', install_dir: get_option('prefix') / 'share/metainfo')
install_data('../data/icons/io.github.rsvzz.csven.svg', install_dir: get_option('prefix') / 'share/icons/hicolor/scalable/apps')
install_data('../data/desktop/io.github.rsvzz.csven.desktop', install_dir: get_option('prefix') /'share/applications')

icons_file = files(
   '../data/ui/csven.ui',
   '../data/ui/verb.ui',
   '../data/ui/about.ui'
)
install_data(icons_file, install_dir: get_option('prefix') / 'share/csven/ui')

custom_target('csven',
  output: 'csven',
  build_by_default: true,
  install : true,
  install_dir : get_option('bindir'),
  command: ['cargo', 'build', '--release', '--manifest-path', meson.project_source_root() + '/Cargo.toml'
  ,'--target-dir', meson.project_build_root() / 'src',
   '&&', 'cp', meson.project_build_root() / 'src/release/csven', '@OUTPUT@']
)