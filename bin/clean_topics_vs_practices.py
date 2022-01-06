#!/usr/bin/env python3
import json


def main():
    with open("config.json", encoding="utf-8") as f:
        config = json.load(f)

    concepts = {c['slug'] for c in config['concepts']}

    for practice_exercise in config['exercises']['practice']:
        if practice_exercise['topics'] is None:
            continue

        practice_exercise['practices'].extend((topic for topic in practice_exercise['topics'] if topic in concepts))
        practice_exercise['topics'] = [topic for topic in practice_exercise['topics'] if topic not in concepts]

    for concept in concepts:
        count = 0
        for practice_exercise in config['exercises']['practice']:
            if concept in practice_exercise['practices']:
                count += 1
                if count > 10:
                    practice_exercise['practices'].remove(concept)
                    practice_exercise['topics'].append(concept)

    for practice_exercise in config['exercises']['practice']:
        practice_exercise['practices'].sort()

        if practice_exercise['topics'] is not None:
            practice_exercise['topics'].sort()


    with open("config.json", 'w', encoding="utf-8") as f:
        json.dump(config, f, indent=2, ensure_ascii=False)
        f.write('\n')

    print("Updated config.json")


if __name__ == '__main__':
    main()
