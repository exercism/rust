#!/usr/bin/env python
"""
Script to initialize an exercise for Exercism's Rust track

Why a Python script in the Rust track repo? Distribution.
A rust program would either need to be precompiled for various
platforms, and available for download (in which case it wouldn't
conveniently work in the repository), or would need to be included
as a sub-crate and compiled locally. A python script can simply
be a single file and depend on the user's system Python, if desired.

This module requires Python3.5 or newer
"""
from __future__ import print_function

try:
    import collections
    import json
    import os
    import shlex
    import subprocess
    import string
    import sys
    import urllib.request

    from contextlib import contextmanager
    from datetime import datetime
    from uuid import uuid4
except ImportError:
    print("This script requires Python 3.5 or higher", file=sys.stderr)
    # exiting like this isn't great for library use, but at least it's a quick fail
    sys.exit(1)

# check version info
if sys.version_info[0] != 3 or sys.version_info[1] < 5:
    print("This script requires Python 3.5 or higher", file=sys.stderr)
    # exiting like this isn't great for library use, but at least it's a quick fail
    sys.exit(1)


def output_of(cmd, check_returncode=True):
    "Return the stdout of the given command"
    sp = subprocess.run(shlex.split(cmd),
                        stdout=subprocess.PIPE,
                        universal_newlines=True)
    if check_returncode:
        sp.check_returncode()
    return sp.stdout.strip()


REPO_ROOT = output_of('git rev-parse --show-toplevel')
EXERCISES = os.path.join(REPO_ROOT, 'exercises')
ITEM_NAME_CHARS = {c for c in string.ascii_lowercase + string.digits + '_'}
VERSION = "0.1.0"


def to_item_name(description):
    "Produce a valid rust item name from arbitrary inputs"
    item = description.lower().replace(' ', '_')
    item = [c for c in item if c in ITEM_NAME_CHARS]
    while len(item) > 0 and item[0] in string.digits:
        item = item[1:]
    if len(item) == 0:
        raise ValueError("Could not produce an item name from " + description)
    return item


def url_for(name, file):
    return "https://raw.githubusercontent.com/exercism/problem-specifications/master/exercises/{name}/{file}".format(
        name=name, file=file,
    )


def get_problem_specification(name):
    """
    Try to get problem specifications for the exercise of the given name.

    If the problem specifications repo doesn't exist, returns None.
    Otherwise, returns a dict, of which the values might be None or str.
    """
    try:
        with urllib.request.urlopen(url_for(name, 'canonical-data.json')) as response:
            return json.loads(response.read())
    except (urllib.request.URLError, json.JSONDecodeError):
        pass


@contextmanager
def inside(path):
    cwd = os.getcwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(cwd)


def make_exercise(name):
    "Make a new exercise with the specified name"
    with inside(EXERCISES):
        if os.path.exists(name):
            print("{} already exists; aborting".format(name), file=sys.stderr)
            sys.exit(1)
        subprocess.run(['cargo', 'new', name])
    exercise_dir = os.path.join(EXERCISES, name)
    # blank out the default lib.rs
    with inside(exercise_dir):
        with open(os.path.join('src', 'lib.rs'), 'w') as lib_rs:
            lib_rs.write('')
        os.mkdir('tests')
        with inside('tests'):
            with open('{}.rs'.format(name), 'w') as tests_rs:
                print("use {}::*;".format(name), file=tests_rs)
                print(file=tests_rs)

    cd = get_problem_specification(name)
    if cd is None:
        print("No problem specification for {} found".format(name))
        make_new_exercise(name, exercise_dir)
        with inside(exercise_dir):
            generate_readme(name, False)
    else:
        make_exercise_with_specifications(name, exercise_dir, cd)
        with inside(exercise_dir):
            generate_readme(name, True)


def make_new_exercise(name, exercise_dir):
    print("Creating new exercise from scratch...")
    with inside(exercise_dir):
        os.mkdir('.meta')
        with inside('.meta'):
            with open('description.md', 'w') as description:
                print("Describe your exercise here", file=description)
            with open('metadata.yml', 'w') as metadata:
                print("---", file=metadata)
                print("blurb: \"{} (created {})\"".format(
                    name, datetime.now().isoformat()), file=metadata)
                print("source: \"\"", file=metadata)
                print("source_url: \"\"", file=metadata)
        with inside('tests'):
            with open('{}.rs'.format(name), 'a') as tests_rs:
                print("// Add your tests here", file=tests_rs)


def make_exercise_with_specifications(name, exercise_dir, canonical_data):
    print("Creating exercise from specification...")
    with open(os.path.join(exercise_dir, 'tests', '{}.rs'.format(name)), 'a') as tests_rs:
        print("fn process_case(input: ???, expected: ???) {", file=tests_rs)
        print("    unimplemented!()", file=tests_rs)
        print("}", file=tests_rs)
        print(file=tests_rs)
        first_case = True

        def literal(item):
            if isinstance(item, str):
                return '"{}"'.format(item)
            elif isinstance(item, list):
                return "vec![{}]".format(
                    ', '.join((literal(i) for i in item))
                )
            elif isinstance(item, dict):
                return """
                    {
                        let mut hm = ::std::collections::HashMap::new();
                        {}
                        hm
                    }
                """.strip().format('\n                        '.join((
                    "hm.insert({}, {});".format(literal(k), literal(v))
                    for k, v in item.items()))
                )
            else:
                return str(item)

        def write_case(case):
            nonlocal first_case

            print("#[test]", file=tests_rs)
            if first_case:
                first_case = False
            else:
                print("#[ignore]", file=tests_rs)
            print("fn test_{}() {".format(to_item_name(case['description'])), file=tests_rs)
            print("    process_case({}, {});".format(literal(case['input']), literal(case['expected'])),
                  file=tests_rs)
            print("}", file=tests_rs)
            print(file=tests_rs)

        def write_cases(cases):
            for item in cases:
                if all(key in item for key in ('description', 'input', 'expected')):
                    write_case(item)
                if 'cases' in item:
                    write_cases(item['cases'])

        write_cases(canonical_data['cases'])


def prompt(prompt, validator):
    """
    Prompt the user for a value

    Validator is a function which accepts the user's input and either
    returns a (possibly transformed) value, or raises an exception.
    On an exception, the user is asked again.
    """
    while True:
        try:
            return validator(input(prompt).strip())
        except Exception as e:
            print("Problem: {}".format(e))


def update_config(name):
    "Update the configuration based on user input"
    with inside(REPO_ROOT):
        with open('config.json') as config_json:
            config = json.load(config_json, object_pairs_hook=collections.OrderedDict)

    while True:
        conf_values = collections.OrderedDict()
        conf_values['uuid'] = str(uuid4())
        conf_values['slug'] = name
        conf_values['core'] = False

        def unlock_validator(v):
            if len(v) == 0:
                return None
            if not any(v == ex['slug'] for ex in config['exercises']):
                raise ValueError("{} is not an existing exercise slug".format(v))
            return v
        conf_values['unlocked_by'] = prompt(
            "Exercise slug which unlocks this (blank for None): ", unlock_validator)

        def difficulty_validator(v):
            i = int(v)
            if i <= 0 or i > 10:
                raise ValueError("difficulty must be > 0 and <= 10")
            return i
        conf_values['difficulty'] = prompt(
            "Difficulty for this exercise([1...10]): ", difficulty_validator)

        def topics_validator(v):
            topics = [t.strip() for t in v.split(',') if len(t.strip()) > 0]
            if len(topics) == 0:
                raise ValueError("must enter at least one topic")
            return topics
        conf_values['topics'] = prompt(
            "List of topics for this exercise, comma-separated: ", topics_validator)

        print("You have configured this exercise as follows:")
        print(json.dumps(conf_values, sort_keys=True, indent=4))

        yn = input('Is this correct? (y/N): ').strip().lower()
        if len(yn) > 0 and yn[0] == 'y':
            break

    if not any(conf_values['difficulty'] == ex['difficulty'] for ex in config['exercises']):
        config['exercises'].append(conf_values)
        config['exercises'].sort(key=lambda ex: ex['difficulty'])
    else:
        # find the index bounds before which we might insert this
        first_idx = None
        last_idx = None
        for idx, exercise in enumerate(config['exercises']):
            if 'difficulty' in exercise and exercise['difficulty'] == conf_values['difficulty'] and first_idx is None:
                first_idx = idx
            if 'difficulty' in exercise and exercise['difficulty'] != conf_values['difficulty'] and first_idx is not None:
                last_idx = idx
        if last_idx is None:
            last_idx = len(config['exercises'])

        def binary_search(start_idx, end_idx):
            if start_idx == end_idx:
                return start_idx
            mid_idx = start_idx + ((end_idx - start_idx) // 2)

            def easy_hard_validator(v):
                v = v.lower()[0]
                if v not in {'e', 'h'}:
                    raise ValueError("must enter 'easier' or 'harder' or a substring")
                return v
            relative_difficulty = prompt(
                "Is {} easier or harder than {}: ".format(
                    name, config['exercises'][mid_idx]['slug']
                ),
                easy_hard_validator
            )

            if relative_difficulty == 'e':
                return binary_search(start_idx, mid_idx)
            else:
                return binary_search(mid_idx + 1, end_idx)

        while True:
            insert_idx = binary_search(first_idx, last_idx)
            if insert_idx == 0:
                ptext = "{} is the easiest exercise in the track.".format(name)
            elif insert_idx == len(config['exercises']):
                ptext = "{} is the hardest exercise in the track.".format(name)
            else:
                ptext = "{} fits between {} and {} in difficulty.".format(
                    name,
                    config['exercises'][insert_idx - 1]['slug'],
                    config['exercises'][insert_idx]['slug'],
                )
            print("You have indicated that {}".format(ptext))
            yn = input('Is this correct? (y/N): ').strip().lower()
            if len(yn) > 0 and yn[0] == 'y':
                break

        config['exercises'].insert(insert_idx, conf_values)

    with inside(REPO_ROOT):
        with open('config.json', 'w') as config_json:
            json.dump(
                config,
                config_json,
                sort_keys=False,
                indent=2,
            )


@contextmanager
def git_master(git_path):
    "A context inside of which you are on the clean master branch"
    with inside(git_path):
        dirty = len(output_of('git status --porcelain')) > 0
        if dirty:
            subprocess.run(['git', 'stash'])
        branch = output_of('git rev-parse --abbrev-ref HEAD')
        if branch != 'master':
            subprocess.run(['git', 'checkout', 'master'])
        subprocess.run(['git', 'pull'])

        try:
            yield
        finally:
            if branch != 'master':
                subprocess.run(['git', 'checkout', branch])
            if dirty:
                subprocess.run(['git', 'stash', 'pop'])


def generate_readme(exercise_name, get_problem_specification):
    configlet = None
    with inside(os.path.join(REPO_ROOT, 'bin')):
        if not os.path.exists('configlet') and not os.path.exists('configlet.exe'):
            with inside(REPO_ROOT):
                subprocess.run('fetch-configlet')
        for configlet_name in ('configlet', 'configlet.exe'):
            if os.path.exists(configlet_name):
                configlet = configlet_name
                break
        if configlet is None:
            print("Could not locate configlet; aborting", file=sys.stderr)
            sys.exit(1)
    if get_problem_specification:
        with inside(os.path.join(REPO_ROOT, '..')):
            if os.path.exists('problem-specifications'):
                with git_master('problem-specifications'):
                    with inside(REPO_ROOT):
                        subprocess.run([
                            os.path.join('bin', configlet),
                            'generate', '.',
                            '--only', exercise_name,
                            '--spec-path',
                            os.path.join('..', 'problem-specifications')
                        ])
            else:
                subprocess.run(
                    ['git', 'clone', 'https://github.com/exercism/problem-specifications.git']
                )
                with inside(REPO_ROOT):
                    subprocess.run([
                        os.path.join('bin', configlet),
                        'generate', '.',
                        '--only', exercise_name,
                        '--spec-path',
                        os.path.join('..', 'problem-specifications')
                    ])
    else:
        with inside(REPO_ROOT):
            subprocess.run([
                os.path.join('bin', configlet),
                'generate', '.',
                '--only', exercise_name,
            ])


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description='Create a Rust Track exercise for Exercism')
    parser.add_argument('name', help='name of the exercise to create')
    parser.add_argument('--dont-create-exercise', action='store_true',
                        help='Don\'t create the exercise. Useful when just updating config.json')
    parser.add_argument('--dont-update-config', action='store_true',
                        help='Don\'t update config.json. Useful when you don\'t yet '
                        'have a sense of exercise difficulty.')
    parser.add_argument('--version', action='version', version=VERSION)

    args = parser.parse_args()

    if not args.dont_create_exercise:
        make_exercise(args.name)

    if not args.dont_update_config:
        update_config(args.name)
