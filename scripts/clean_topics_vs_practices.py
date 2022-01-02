import json


def main():
    with open("config.json") as f:
        config = json.load(f)

    concepts = [c['slug'] for c in config['concepts']]
    concepts.sort()
    # print('\n'.join(concepts))

    for practice_exercise in config['exercises']['practice']:
        if practice_exercise['topics'] is None:
            continue

        for topic in practice_exercise['topics']:
            if topic in concepts:
                practice_exercise['topics'].remove(topic)
                practice_exercise['practices'].append(topic)

    with open("new_config.json", 'w') as f:
        json.dump(config, f, indent=2)
    
    print(f"Generated a new config.json as new_config.json. Verify the changes then replace config.json with new_config.json")

main()
