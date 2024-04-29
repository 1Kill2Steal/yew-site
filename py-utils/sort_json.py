import json
import argparse

def sort_json(data):
    """
    Sort the JSON input file with all its elements by prioritizing numeric
    sorting, otherwise do lexicographical sorting instead.
    """
    if isinstance(data, dict):
        # Check for numeric sorting.
        if all(key.isdigit() for key in data.keys()):
            # Sort dictionary keys numerically
            sorted_keys = sorted(data.keys(), key=lambda x: int(x))
        # Otherwise do lexicograpthical sorting.
        else:
            sorted_keys = sorted(data.keys())

        sorted_data = {k: sort_json(data[k]) for k in sorted_keys}
        return sorted_data
    elif isinstance(data, list):
        sorted_list = [sort_json(item) for item in data]
        return sorted_list
    else:
        # Base case: non-dict, non-list
        return data

def main():
    """
    Handle the JSON files by parsing them as arguments (input and output files)
    and sorting them where only the input file is required and if there's no
    output file provided then it directly sorts the input file.
    """
    parser = argparse.ArgumentParser(description='Sort JSON files based on arguments.')
    parser.add_argument(
        '--input', '-i', \
        required=True, \
        help='Specifying the input JSON file', \
    )
    parser.add_argument(
        '--output', '-o', \
        required=False, \
        help='Specifying the output JSON file (Default: input file)', \
    )
    args = parser.parse_args()

    input_file_path = args.input
    output_file_path = args.output

    if not output_file_path:
        output_file_path = input_file_path

    with open(input_file_path, "r") as input_file:
        input_json = json.load(input_file)

    # Sort the input file and write the sorted result into the output file
    # without ensuring ASCII encoding (because you may have UTF-8 characters in
    # the JSON file.)
    sorted_input_json = sort_json(input_json)

    with open(output_file_path, "w") as output_file:
        json.dump(
            sorted_input_json, \
            output_file, \
            ensure_ascii=False, \
            indent=2, \
        )


if __name__ == "__main__":
    main()
