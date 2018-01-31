# setup.py based on https://github.com/getsentry/milksnake
from setuptools import setup


NAME = 'griddfs'


def build_native(spec):
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
    )

    spec.add_cffi_module(
        module_path='%s._native' % NAME,
        dylib=lambda: build.find_dylib(NAME, in_path='target/release'),
        header_filename=lambda: build.find_header('%s.h' % NAME, in_path='include'),
    )


setup(
    name=NAME,
    version='0.1.0',
    packages=[NAME],
    author='Mathias Rav',
    license='GPL3+',
    author_email='m@git.strova.dk',
    description='DFS on a grid',
    # long_description=readme,
    include_package_data=True,
    zip_safe=False,
    platforms='any',
    install_requires=['milksnake'],
    setup_requires=['milksnake'],
    milksnake_tasks=[
        build_native,
    ],
)
