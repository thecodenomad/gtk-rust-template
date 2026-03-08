gnome = import('gnome')

desktop_conf = configuration_data()
desktop_conf.set('app-id', app_id)
desktop_conf.set('icon', app_id)
desktop_file = i18n.merge_file(
    type: 'desktop',
    input: configure_file(
        input: '@0@.desktop.in.in'.format(base_id),
        output: '@BASENAME@',
        configuration: desktop_conf,
    ),
    output: '@0@.desktop'.format(app_id),
    po_dir: podir,
    install: true,
    install_dir: datadir / 'applications',
)

desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
    test('Validate desktop file', desktop_utils, args: [desktop_file])
endif

install_data(
    'com.example.gtk_rust_template.gschema.xml',
    install_dir: datadir / 'glib-2.0/schemas',
)

compile_schemas = find_program('glib-compile-schemas', required: false)
if compile_schemas.found()
    test(
        'Validate schema file',
        compile_schemas,
        args: ['--strict', '--dry-run', meson.current_source_dir()],
    )
endif

# Enable when icons are added
# subdir('icons')
